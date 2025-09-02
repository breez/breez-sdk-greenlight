use tonic::async_trait;
use vss_client::client::VssClient;
use vss_client::error::VssError;
use vss_client::types::{
    DeleteObjectRequest, GetObjectRequest, GetObjectResponse, KeyValue, ListKeyVersionsRequest,
    PutObjectRequest,
};
use vss_client::util::retry::RetryPolicy;

use crate::ldk::store::versioned_store::{Error, VersionedStore};

pub struct VssStore<P: RetryPolicy<E = VssError> + Send + Sync> {
    client: VssClient<P>,
    store_id: String,
}

impl<P: RetryPolicy<E = VssError> + Send + Sync> VssStore<P> {
    pub fn new(client: VssClient<P>, store_id: String) -> Self {
        Self { client, store_id }
    }
}

#[async_trait]
impl<P: RetryPolicy<E = VssError> + Send + Sync> VersionedStore for VssStore<P> {
    async fn get(&self, key: String) -> Result<Option<(Vec<u8>, i64)>, Error> {
        let request = GetObjectRequest {
            store_id: self.store_id.clone(),
            key,
        };

        match self.client.get_object(&request).await {
            Ok(GetObjectResponse { value: Some(kv) }) => Ok(Some((kv.value, kv.version))),
            Ok(GetObjectResponse { value: None }) => Ok(None),
            Err(VssError::NoSuchKeyError(_)) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    async fn put(&self, key: String, value: Vec<u8>, version: i64) -> Result<(), Error> {
        let key_value = KeyValue {
            key,
            version,
            value,
        };
        let request = PutObjectRequest {
            store_id: self.store_id.clone(),
            transaction_items: vec![key_value],
            ..Default::default()
        };

        self.client.put_object(&request).await?;
        Ok(())
    }

    async fn delete(&self, key: String) -> Result<(), Error> {
        let key_value = KeyValue {
            key,
            version: -1,
            value: Vec::new(),
        };

        let request = DeleteObjectRequest {
            store_id: self.store_id.clone(),
            key_value: Some(key_value),
        };

        self.client.delete_object(&request).await?;
        Ok(())
    }

    async fn list(&self) -> Result<Vec<(String, i64)>, Error> {
        let mut request = ListKeyVersionsRequest {
            store_id: self.store_id.clone(),
            ..Default::default()
        };
        let mut versions = Vec::new();
        loop {
            let mut response = self.client.list_key_versions(&request).await?;
            versions.append(&mut response.key_versions);
            if response
                .next_page_token
                .as_deref()
                .unwrap_or_default()
                .is_empty()
            {
                break;
            }
            request.page_token = response.next_page_token;
        }

        let versions = versions
            .into_iter()
            .map(|kv| (kv.key, kv.version))
            .collect();
        Ok(versions)
    }
}

impl From<VssError> for Error {
    fn from(err: VssError) -> Self {
        match err {
            VssError::NoSuchKeyError(_) => {
                Error::Internal("Received VssError::NoSuchKeyError".to_string())
            }
            VssError::ConflictError(e) => Error::Conflict(e),
            _ => Error::Internal(format!("{err:?}")),
        }
    }
}
