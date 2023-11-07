use std::net::TcpStream;
use std::str::FromStr;

use reqwest::Url;

use crate::error::{SdkError, SdkResult};

#[tonic::async_trait]
pub trait NeedsConnectivity {
    fn get_endpoint_url(&self) -> String;

    /// Lightweight connectivity check.
    /// Performs a DNS lookup and tries to open a TCP connection to the endpoint.
    async fn check_connectivity(&self) -> SdkResult<()> {
        let base_url = self.get_endpoint_url();
        let url = Url::from_str(&base_url).map_err(|_| SdkError::ServiceConnectivity {
            err: format!("Invalid URL: {base_url}"),
        })?;
        let addrs = url
            .socket_addrs(|| None)
            .map_err(|e| SdkError::ServiceConnectivity {
                err: format!("Cannot resolve {base_url} URL because {e}"),
            })?;

        TcpStream::connect(&*addrs)
            .map(|_| ())
            .map_err(|e| SdkError::ServiceConnectivity {
                err: format!("Cannot connect to {base_url} because {e}"),
            })
    }
}
