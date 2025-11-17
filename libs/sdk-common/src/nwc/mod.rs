use nostr_sdk::nips::nip47::{Error, NostrWalletConnectURI as _NostrWalletConnectURI};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NostrWalletConnectUri {
    pub public_key: String,
    pub secret: String,
    pub relays: Vec<String>,
    pub lud16: Option<String>,
}

impl From<_NostrWalletConnectURI> for NostrWalletConnectUri {
    fn from(value: _NostrWalletConnectURI) -> Self {
        Self {
            public_key: value.public_key.to_hex(),
            secret: value.secret.to_secret_hex(),
            relays: value
                .relays
                .into_iter()
                .map(|url| url.to_string())
                .collect(),
            lud16: value.lud16,
        }
    }
}

pub(crate) fn parse_nwc_uri(uri: &str) -> Result<NostrWalletConnectUri, Error> {
    _NostrWalletConnectURI::parse(uri).map(Into::into)
}

#[cfg(test)]
mod tests {
    use crate::nwc::parse_nwc_uri;

    #[test]
    fn test_parse_nwc_uri() {
        let uri = "nostr+walletconnect://b889ff5b1513b641e2a139f661a661364979c5beee91842f8f0ef42ab558e9d4?relay=wss%3A%2F%2Frelay.damus.io&secret=71a8c14c1407c113601079c4302dab36460f0ccd0ad506f1f2dc73b5100e4f3c";
        let uri = parse_nwc_uri(uri).unwrap();
        assert_eq!(
            uri.public_key,
            "b889ff5b1513b641e2a139f661a661364979c5beee91842f8f0ef42ab558e9d4"
        );
        assert_eq!(uri.relays, vec!["wss://relay.damus.io"]);
        assert_eq!(
            uri.secret,
            "71a8c14c1407c113601079c4302dab36460f0ccd0ad506f1f2dc73b5100e4f3c"
        );
        assert_eq!(uri.lud16, None);
    }
}
