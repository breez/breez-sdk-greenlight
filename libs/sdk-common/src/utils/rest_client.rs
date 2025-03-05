use log::*;
use reqwest::Client;
use std::{collections::HashMap, time::Duration};

use crate::error::{ServiceConnectivityError, ServiceConnectivityErrorKind};

const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

#[sdk_macros::async_trait]
pub trait RestClient: Send + Sync {
    /// Makes a GET request and logs on DEBUG.
    /// ### Arguments
    /// - `url`: the URL on which GET will be called
    async fn get(&self, url: &str) -> Result<(String, u16), ServiceConnectivityError>;

    /// Makes a POST request, and logs on DEBUG.
    /// ### Arguments
    /// - `url`: the URL on which POST will be called
    /// - `headers`: the optional POST headers
    /// - `body`: the optional POST body
    async fn post(
        &self,
        url: &str,
        headers: Option<HashMap<String, String>>,
        body: Option<String>,
    ) -> Result<(String, u16), ServiceConnectivityError>;
}

pub struct ReqwestRestClient {
    client: Client,
}
impl ReqwestRestClient {
    pub fn new() -> Result<Self, ServiceConnectivityError> {
        let client = Client::builder()
            .build()
            .map_err(Into::<ServiceConnectivityError>::into)?;
        Ok(ReqwestRestClient { client })
    }
}

#[sdk_macros::async_trait]
impl RestClient for ReqwestRestClient {
    async fn get(&self, url: &str) -> Result<(String, u16), ServiceConnectivityError> {
        debug!("Making GET request to: {url}");
        let response = self.client.get(url).timeout(REQUEST_TIMEOUT).send().await?;
        let status = response.status().into();
        let raw_body = response.text().await?;
        debug!("Received response, status: {status}");
        trace!("raw response body: {raw_body}");

        Ok((raw_body, status))
    }

    async fn post(
        &self,
        url: &str,
        headers: Option<HashMap<String, String>>,
        body: Option<String>,
    ) -> Result<(String, u16), ServiceConnectivityError> {
        debug!("Making POST request to: {url}");
        let mut req = self.client.post(url).timeout(REQUEST_TIMEOUT);
        if let Some(headers) = headers {
            for (key, value) in headers.iter() {
                req = req.header(key, value);
            }
        }
        if let Some(body) = body {
            req = req.body(body);
        }
        let response = req.send().await?;
        let status = response.status();
        let raw_body = response.text().await?;
        debug!("Received response, status: {status}");
        trace!("raw response body: {raw_body}");

        Ok((raw_body, status.into()))
    }
}

pub fn parse_json<T>(json: &str) -> Result<T, ServiceConnectivityError>
where
    for<'a> T: serde::de::Deserialize<'a>,
{
    serde_json::from_str::<T>(json).map_err(|e| {
        ServiceConnectivityError::new(ServiceConnectivityErrorKind::Json, e.to_string())
    })
}

pub async fn get_and_check_success<C: RestClient + ?Sized>(
    rest_client: &C,
    url: &str,
) -> Result<(String, u16), ServiceConnectivityError> {
    let (raw_body, status) = rest_client.get(url).await?;
    #[allow(clippy::manual_range_contains)]
    if status < 200 || status >= 300 {
        let err = format!("GET request {url} failed with status: {status}");
        error!("{err}");
        return Err(ServiceConnectivityError::new(
            ServiceConnectivityErrorKind::Status,
            err,
        ));
    }

    Ok((raw_body, status))
}
