//! Sends the [`Call`]s of [`crate::exchange`] with reqwest (feature `http`).

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};

use crate::constants::HTTP_CLIENT;
use crate::exchange::{
    Answer, Call, ConsultResponse, File, HttpMethod, HttpRequest, SendFileResponse,
    SunatClientError, VerifyTicketResponse,
};
use crate::models::{
    ConsultDocument, Credentials, SendFileTarget, ValidateCdp, VerifyTicketTarget,
};

/// Pauses before retrying a request SUNAT's front proxy refused
const RATE_LIMIT_PAUSES: [Duration; 2] = [Duration::from_secs(1), Duration::from_secs(2)];

#[derive(Default)]
pub struct SunatClient {
    /// Where the tokens of the guias are requested; SUNAT's by default
    pub security_api_url: Option<String>,
}

impl SunatClient {
    pub async fn send_file(
        &self,
        target: &SendFileTarget,
        file: &File,
        credentials: &Credentials,
    ) -> Result<SendFileResponse, SunatClientError> {
        let call = Call::SendFile {
            target: target.clone(),
            file: file.clone(),
        };
        match self.exchange(&call, credentials).await? {
            Answer::SendFile(response) => Ok(response),
            _ => unreachable!("a SendFile call answers SendFile"),
        }
    }

    pub async fn verify_ticket(
        &self,
        target: &VerifyTicketTarget,
        ticket: &str,
        credentials: &Credentials,
    ) -> Result<VerifyTicketResponse, SunatClientError> {
        let call = Call::VerifyTicket {
            target: target.clone(),
            ticket: ticket.to_string(),
        };
        match self.exchange(&call, credentials).await? {
            Answer::VerifyTicket(response) => Ok(response),
            _ => unreachable!("a VerifyTicket call answers VerifyTicket"),
        }
    }

    /// BillConsultService getStatus: status of a document already sent
    pub async fn get_status(
        &self,
        url: &str,
        document: &ConsultDocument,
        credentials: &Credentials,
    ) -> Result<ConsultResponse, SunatClientError> {
        let call = Call::GetStatus {
            url: url.to_string(),
            document: document.clone(),
        };
        self.consult(&call, credentials).await
    }

    /// BillConsultService getStatusCdr: CDR of a document already sent
    pub async fn get_status_cdr(
        &self,
        url: &str,
        document: &ConsultDocument,
        credentials: &Credentials,
    ) -> Result<ConsultResponse, SunatClientError> {
        let call = Call::GetStatusCdr {
            url: url.to_string(),
            document: document.clone(),
        };
        self.consult(&call, credentials).await
    }

    /// BillValidService validaCDPcriterios
    pub async fn validate_cdp(
        &self,
        url: &str,
        cdp: &ValidateCdp,
        credentials: &Credentials,
    ) -> Result<ConsultResponse, SunatClientError> {
        let call = Call::ValidateCdp {
            url: url.to_string(),
            cdp: cdp.clone(),
        };
        self.consult(&call, credentials).await
    }

    /// BillValidService verificaCPEarchivo
    pub async fn validate_file(
        &self,
        url: &str,
        file: &File,
        credentials: &Credentials,
    ) -> Result<ConsultResponse, SunatClientError> {
        let call = Call::ValidateFile {
            url: url.to_string(),
            file: file.clone(),
        };
        self.consult(&call, credentials).await
    }

    async fn consult(
        &self,
        call: &Call,
        credentials: &Credentials,
    ) -> Result<ConsultResponse, SunatClientError> {
        match self.exchange(call, credentials).await? {
            Answer::Consult(response) => Ok(response),
            _ => unreachable!("a consult call answers Consult"),
        }
    }

    /// Sends a call, with an OAuth token when it needs one. SUNAT revokes
    /// tokens before they expire: a call rejected with HTTP 401 is retried
    /// once with a new token (unless the token was given in the credentials).
    /// Calls refused by SUNAT's rate limiting are retried after a pause.
    async fn exchange(
        &self,
        call: &Call,
        credentials: &Credentials,
    ) -> Result<Answer, SunatClientError> {
        if call.needs_token() {
            let token = self.token(credentials).await?;
            let (status, body) = send(&call.request(credentials, Some(&token))?).await?;
            if status != 401 || credentials.token.is_some() {
                return call.parse(status, &body);
            }
            forget_token(credentials, &token);
            let token = self.token(credentials).await?;
            let (status, body) = send(&call.request(credentials, Some(&token))?).await?;
            return call.parse(status, &body);
        }

        let request = call.request(credentials, None)?;
        let mut pauses = RATE_LIMIT_PAUSES.iter();
        loop {
            let (status, body) = send(&request).await?;
            match (call.parse(status, &body), pauses.next()) {
                (Err(SunatClientError::RateLimited), Some(pause)) => {
                    tokio::time::sleep(*pause).await
                }
                (answer, _) => return answer,
            }
        }
    }

    /// The token of the credentials, a cached one, or a new one. One token is
    /// requested at a time, so concurrent calls don't revoke each other's.
    async fn token(&self, credentials: &Credentials) -> Result<String, SunatClientError> {
        if let Some(token) = &credentials.token {
            return Ok(token.clone());
        }
        let key = token_key(credentials);
        if let Some(token) = cached_token(&key) {
            return Ok(token);
        }
        let _requesting = TOKEN_REQUEST.lock().await;
        if let Some(token) = cached_token(&key) {
            return Ok(token);
        }
        let call = Call::Token {
            url: self.security_api_url.clone(),
        };
        let Answer::Token(token) = Box::pin(self.exchange(&call, credentials)).await? else {
            unreachable!("a Token call answers Token");
        };
        store_token(key, &token.access_token, token.expires_in);
        Ok(token.access_token)
    }
}

/// Sends the request, again once if the connection failed
async fn send(request: &HttpRequest) -> Result<(u16, String), SunatClientError> {
    let build = || {
        let mut builder = match request.method {
            HttpMethod::Get => HTTP_CLIENT.get(&request.url),
            HttpMethod::Post => HTTP_CLIENT.post(&request.url),
        };
        for (name, value) in &request.headers {
            builder = builder.header(name, value);
        }
        if let Some(body) = &request.body {
            builder = builder.body(body.clone());
        }
        builder
    };
    let response = match build().send().await {
        Err(error) if error.is_connect() => build().send().await?,
        other => other?,
    };
    let status = response.status().as_u16();
    Ok((status, response.text().await?))
}

/// Tokens of each client and SOL user, reused until a minute before they expire
static TOKENS: LazyLock<Mutex<HashMap<String, (String, Instant)>>> =
    LazyLock::new(Default::default);

/// Held while a token is requested
static TOKEN_REQUEST: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

fn token_key(credentials: &Credentials) -> String {
    let client_id = credentials.client_id.as_deref().unwrap_or_default();
    format!("{client_id}:{}", credentials.username)
}

/// Forgets the rejected token, unless another call already replaced it
fn forget_token(credentials: &Credentials, rejected: &str) {
    if let Ok(mut tokens) = TOKENS.lock() {
        let key = token_key(credentials);
        if tokens.get(&key).is_some_and(|(token, _)| token == rejected) {
            tokens.remove(&key);
        }
    }
}

fn cached_token(key: &str) -> Option<String> {
    let tokens = TOKENS.lock().ok()?;
    let (token, reuse_until) = tokens.get(key)?;
    (Instant::now() < *reuse_until).then(|| token.clone())
}

fn store_token(key: String, token: &str, expires_in: u64) {
    let reuse_until = Instant::now() + Duration::from_secs(expires_in.saturating_sub(60));
    if let Ok(mut tokens) = TOKENS.lock() {
        tokens.insert(key, (token.to_string(), reuse_until));
    }
}

#[cfg(test)]
mod tests {
    use super::{cached_token, forget_token, store_token, token_key};
    use crate::models::Credentials;

    #[test]
    fn tokens_are_reused_until_a_minute_before_they_expire() {
        store_token("client-a:user".into(), "token-a", 3600);
        assert_eq!(cached_token("client-a:user").as_deref(), Some("token-a"));

        // Less than a minute left: request a new one
        store_token("client-b:user".into(), "token-b", 30);
        assert_eq!(cached_token("client-b:user"), None);

        assert_eq!(cached_token("unknown"), None);
    }

    #[test]
    fn only_the_rejected_token_is_forgotten() {
        let credentials = Credentials {
            username: "user".into(),
            client_id: Some("client-c".into()),
            ..Default::default()
        };
        store_token(token_key(&credentials), "token-c", 3600);

        // Another call replaced the token it got rejected
        forget_token(&credentials, "an-older-token");
        assert_eq!(cached_token("client-c:user").as_deref(), Some("token-c"));

        forget_token(&credentials, "token-c");
        assert_eq!(cached_token("client-c:user"), None);
    }
}
