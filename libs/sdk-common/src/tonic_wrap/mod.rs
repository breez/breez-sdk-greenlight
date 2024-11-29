use std::{error::Error, fmt::Display, future::Future};

use log::debug;

pub struct Status(pub tonic::Status);

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "status: {:?}, message: {:?}, details: {:?}, metadata: {:?}, source: {:?}",
            self.0.code(),
            self.0.message(),
            self.0.details(),
            self.0.metadata(),
            self.0.source(),
        )
    }
}

pub struct TransportError(pub tonic::transport::Error);

const BROKEN_CONNECTION_STRINGS: [&str; 3] = [
    "http2 error: keep-alive timed out",
    "connection error: address not available",
    "connection error: timed out",
];

impl Display for TransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "description: {:?}, source: {:?}",
            self.0.to_string(),
            self.0.source(),
        )
    }
}

pub async fn with_connection_fallback<T, M, F>(
    main: M,
    fallback: impl FnOnce() -> F,
) -> Result<T, tonic::Status>
where
    M: Future<Output = Result<T, tonic::Status>>,
    F: Future<Output = Result<T, tonic::Status>>,
    T: std::fmt::Debug,
{
    let res = main.await;
    let status = match res {
        Ok(t) => return Ok(t),
        Err(s) => s,
    };

    debug!(
        "with_connection_fallback: initial call failed with: {:?}",
        status
    );
    let source = match status.source() {
        Some(source) => source,
        None => return Err(status),
    };

    let error: &tonic::transport::Error = match source.downcast_ref() {
        Some(error) => error,
        None => return Err(status),
    };

    if error.to_string() != "transport error" {
        return Err(status);
    }

    let source = match error.source() {
        Some(source) => source,
        None => return Err(status),
    };

    // It's a bit of a guess which errors can occur here. hyper Io errors start
    // with 'connection error'. These are some of the errors seen before.
    if !BROKEN_CONNECTION_STRINGS.contains(&source.to_string().as_str()) {
        debug!("transport error string is: '{}'", source.to_string());
        return Err(status);
    }

    debug!(
        "with_connection_fallback: initial call failed due to broken connection. Retrying fallback."
    );

    fallback().await
}
