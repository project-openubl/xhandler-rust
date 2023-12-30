import { OidcClientSettings } from "oidc-client-ts";
import { ENV } from "./env";

export const oidcClientSettings: OidcClientSettings = {
  authority: ENV.OIDC_SERVER_URL || "http://localhost:9001/realms/openubl",
  client_id: ENV.OIDC_CLIENT_ID || "openubl-ui",
  redirect_uri: window.location.href,
  post_logout_redirect_uri: window.location.href.split("?")[0],
};
