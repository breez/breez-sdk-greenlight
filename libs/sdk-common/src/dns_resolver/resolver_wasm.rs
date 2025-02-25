use std::time::Duration;

use anyhow::{anyhow, Result};
use serde::Deserialize;

use crate::utils::rest_client;

/// Response structure according to <https://developers.cloudflare.com/1.1.1.1/encryption/dns-over-https/make-api-requests/dns-json/#response-fields>
#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(rename = "Status")]
    pub status: i32,
    #[serde(default)]
    #[serde(rename = "Answer")]
    pub answer: Vec<Answer>,
}

#[derive(Debug, Deserialize)]
pub struct Answer {
    pub name: String,
    #[serde(rename = "type")]
    pub record_type: i32,
    pub data: String,
}

pub(crate) async fn txt_lookup(dns_name: String) -> Result<Vec<String>> {
    let url = format!("https://cloudflare-dns.com/dns-query?name={dns_name}&type=TXT");
    let raw_body = rest_client::get_reqwest_client()?
        .get(url)
        .header("Accept", "application/dns-json")
        .timeout(Duration::from_secs(30))
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;
    let res: Response = serde_json::from_str(&raw_body)?;
    match res.status {
        0 => Ok(res
            .answer
            .into_iter()
            .filter(|a| a.name == dns_name && a.record_type == 16)
            .map(|a| a.data.replace("\"", ""))
            .collect()),
        n => Err(anyhow!("Error response received from DNS service: {n}")),
    }
}
