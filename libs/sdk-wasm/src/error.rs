use crate::breez_sdk::SdkError;

impl From<anyhow::Error> for SdkError {
    fn from(err: anyhow::Error) -> Self {
        Self::Generic(err.to_string())
    }
}