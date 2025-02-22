use std::{error::Error, fmt::Display};

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

/// Executes the given grpc call function. If an error is returned that
/// indicates the connection broke, the call is tried again.
#[macro_export]
macro_rules! with_connection_retry {
    ($f:expr) => {{
        use log::debug;

        async {
            let res = $f.await;
            let status = match res {
                Ok(t) => return Ok(t),
                Err(s) => s,
            };
            let status_str = status.to_string();

            debug!("with_connection_fallback: initial call failed with: {status_str}");

            if !status_str.contains("transport error") {
                return Err(status);
            }

            debug!(
                "with_connection_fallback: got transport error with source '{}'.
                    Retrying fallback.",
                status_str
            );

            $f.await
        }
    }};
}
