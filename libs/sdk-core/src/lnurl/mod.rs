pub mod pay;

use anyhow::Result;

/// Replaces the scheme, host and port with a local mockito host. Preserves the rest of the path.
#[cfg(test)]
pub(crate) fn maybe_replace_host_with_mockito_test_host(lnurl_endpoint: String) -> Result<String> {
    /// During tests, the mockito test URL chooses a free port. This cannot be known in advance,
    /// so the URL has to be adjusted dynamically.
    let mockito_endpoint_url = reqwest::Url::parse(&mockito::server_url())?;
    let mut parsed_lnurl_endpoint = reqwest::Url::parse(&lnurl_endpoint)?;

    parsed_lnurl_endpoint.set_host(mockito_endpoint_url.host_str())?;
    let _ = parsed_lnurl_endpoint.set_scheme(mockito_endpoint_url.scheme());
    let _ = parsed_lnurl_endpoint.set_port(mockito_endpoint_url.port());

    Ok(parsed_lnurl_endpoint.to_string())
}

#[cfg(not(test))]
pub(crate) fn maybe_replace_host_with_mockito_test_host(lnurl_endpoint: String) -> Result<String> {
    /// When not called from a test, we fallback to keeping the URL intact
    Ok(lnurl_endpoint)
}
