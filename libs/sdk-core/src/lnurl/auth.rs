use std::sync::Arc;

use sdk_common::prelude::{LnUrlError, LnUrlResult, LnurlAuthSigner};

use crate::node_api::NodeAPI;
use crate::bitcoin::{
    hashes::{sha256, Hash, HashEngine, Hmac, HmacEngine},
    secp256k1::{Message, Secp256k1},
    bip32::{ChildNumber, ExtendedPubKey},
};

pub(crate) struct SdkLnurlAuthSigner {
    node_api: Arc<dyn NodeAPI>,
}

impl SdkLnurlAuthSigner {
    pub fn new(node_api: Arc<dyn NodeAPI>) -> Self {
        Self { node_api }
    }
}

#[tonic::async_trait]
impl LnurlAuthSigner for SdkLnurlAuthSigner {
    async fn derive_bip32_pub_key(&self, derivation_path: &[ChildNumber]) -> LnUrlResult<Vec<u8>> {
        let xpriv = self.node_api.derive_bip32_key(derivation_path.to_vec()).await?;
        Ok(ExtendedPubKey::from_priv(&Secp256k1::new(), &xpriv)
            .encode()
            .to_vec())
    }

    async fn sign_ecdsa(&self, msg: &[u8], derivation_path: &[ChildNumber]) -> LnUrlResult<Vec<u8>> {
        let xpriv = self.node_api.derive_bip32_key(derivation_path.to_vec()).await?;
        let sig = Secp256k1::new().sign_ecdsa(
            &Message::from_slice(msg).map_err(|_| LnUrlError::generic("Failed to sign"))?,
            &xpriv.private_key,
        );
        Ok(sig.serialize_der().to_vec())
    }

    async fn hmac_sha256(
        &self,
        key_derivation_path: &[ChildNumber],
        input: &[u8],
    ) -> LnUrlResult<Vec<u8>> {
        let priv_key = self
            .node_api
            .derive_bip32_key(key_derivation_path.to_vec()).await?;
        let mut engine = HmacEngine::<sha256::Hash>::new(priv_key.encode().as_slice());
        engine.input(input);
        Ok(Hmac::<sha256::Hash>::from_engine(engine)
            .as_byte_array()
            .to_vec())
    }
}
