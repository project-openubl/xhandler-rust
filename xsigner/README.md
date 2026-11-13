# XSigner

Signs UBL XML documents with an enveloped RSA-SHA256 XML signature, as SUNAT
requires.

Pure Rust: Canonical XML 1.0 is implemented on top of `quick-xml`, and keys
are handled with RustCrypto (`rsa`, `x509-cert`, `p12-keystore`). No system
libraries are needed, so the crate builds for Android and iOS too.

```rust
use xsigner::{RsaKeyPair, XSigner};

let key_pair = RsaKeyPair::from_pkcs12(&std::fs::read("certificate.pfx")?, "password")?;
let signed_xml: Vec<u8> = XSigner::from_path("invoice.xml")?.sign(&key_pair)?;
```
