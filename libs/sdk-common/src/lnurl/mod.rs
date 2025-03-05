pub mod error;
pub mod model;
pub mod specs;

use super::prelude::*;

/// Replaces the scheme, host and port with a local mockito host. Preserves the rest of the path.
#[cfg(test)]
pub(crate) fn maybe_replace_host_with_mock_test_host(
    lnurl_endpoint: String,
) -> LnUrlResult<String> {
    // During tests, the mockito test URL chooses a free port. This cannot be known in advance,
    // so the URL has to be adjusted dynamically.
    let url = crate::input_parser::tests::MOCK_HTTP_SERVER.url(lnurl_endpoint.clone());
    let mock_endpoint_url =
        reqwest::Url::parse(&url).map_err(|e| LnUrlError::InvalidUri(e.to_string()))?;
    let mut parsed_lnurl_endpoint =
        reqwest::Url::parse(&lnurl_endpoint).map_err(|e| LnUrlError::InvalidUri(e.to_string()))?;

    parsed_lnurl_endpoint
        .set_host(mock_endpoint_url.host_str())
        .map_err(|e| LnUrlError::InvalidUri(e.to_string()))?;
    let _ = parsed_lnurl_endpoint.set_scheme(mock_endpoint_url.scheme());
    let _ = parsed_lnurl_endpoint.set_port(mock_endpoint_url.port());

    Ok(parsed_lnurl_endpoint.to_string())
}

#[cfg(not(test))]
pub(crate) fn maybe_replace_host_with_mock_test_host(
    lnurl_endpoint: String,
) -> LnUrlResult<String> {
    // When not called from a test, we fallback to keeping the URL intact
    Ok(lnurl_endpoint)
}

#[cfg(test)]
mod tests {
    use bitcoin::secp256k1::rand;
    use bitcoin::secp256k1::rand::distributions::{Alphanumeric, DistString};

    pub fn rand_string(len: usize) -> String {
        Alphanumeric.sample_string(&mut rand::thread_rng(), len)
    }
}
