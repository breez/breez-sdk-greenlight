use crate::swap_in::SwapChainData;

use super::{db::SqliteStorage, error::PersistResult};

impl SqliteStorage {
    pub(crate) fn get_swap_chain_data(&self, address: &str) -> PersistResult<SwapChainData> {
        todo!()
    }

    pub(crate) fn set_swap_chain_data(&self, address: &str, chain_data: &SwapChainData) -> PersistResult<()> {
        todo!()
    }
}