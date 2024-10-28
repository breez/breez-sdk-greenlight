use std::sync::Arc;

use anyhow::Result;
use gl_client::bitcoin::{
    hashes::{sha256, Hash, HashEngine, Hmac, HmacEngine},
    secp256k1::{Message, Secp256k1},
    util::bip32::ExtendedPubKey,
};
use sdk_common::{
    lightning::bitcoin::bip32::ChildNumber,
    prelude::{LnUrlError, LnUrlResult, LnurlAuthSigner},
};

use crate::node_api::NodeAPI;

pub(crate) struct SdkLnurlAuthSigner {
    node_api: Arc<dyn NodeAPI>,
}

impl SdkLnurlAuthSigner {
    pub fn new(node_api: Arc<dyn NodeAPI>) -> Self {
        Self { node_api }
    }

    fn convert_derivation_path(
        derivation_path: &[ChildNumber],
    ) -> Result<Vec<bitcoin::util::bip32::ChildNumber>, LnUrlError> {
        let res: Result<Vec<bitcoin::util::bip32::ChildNumber>, bitcoin::util::bip32::Error> =
            derivation_path
                .iter()
                .map(|cn| match cn {
                    ChildNumber::Hardened { index } => {
                        bitcoin::util::bip32::ChildNumber::from_hardened_idx(*index)
                    }
                    ChildNumber::Normal { index } => {
                        bitcoin::util::bip32::ChildNumber::from_normal_idx(*index)
                    }
                })
                .collect();
        res.map_err(|e| LnUrlError::Generic(format!("Could not convert derivation path: {e}")))
    }
}

impl LnurlAuthSigner for SdkLnurlAuthSigner {
    fn derive_bip32_pub_key(&self, derivation_path: &[ChildNumber]) -> LnUrlResult<Vec<u8>> {
        let xpriv = self
            .node_api
            .derive_bip32_key(Self::convert_derivation_path(derivation_path)?)?;
        Ok(ExtendedPubKey::from_priv(&Secp256k1::new(), &xpriv)
            .encode()
            .to_vec())
    }

    fn sign_ecdsa(&self, msg: &[u8], derivation_path: &[ChildNumber]) -> LnUrlResult<Vec<u8>> {
        let xpriv = self
            .node_api
            .derive_bip32_key(Self::convert_derivation_path(derivation_path)?)?;
        let sig = Secp256k1::new().sign_ecdsa(
            &Message::from_slice(msg).map_err(|_| LnUrlError::generic("Failed to sign"))?,
            &xpriv.private_key,
        );
        Ok(sig.serialize_der().to_vec())
    }

    fn hmac_sha256(
        &self,
        key_derivation_path: &[ChildNumber],
        input: &[u8],
    ) -> LnUrlResult<Vec<u8>> {
        let priv_key = self
            .node_api
            .derive_bip32_key(Self::convert_derivation_path(key_derivation_path)?)?;
        let mut engine = HmacEngine::<sha256::Hash>::new(priv_key.encode().as_slice());
        engine.input(input);
        Ok(Hmac::<sha256::Hash>::from_engine(engine)
            .as_inner()
            .to_vec())
    }
}
