use crate::swap_in::{SwapChainData, SwapChainInfo};

use super::{db::SqliteStorage, error::PersistResult};

impl SqliteStorage {
    pub(crate) fn get_swap_chain_data(
        &self,
        address: &str,
    ) -> PersistResult<Option<SwapChainData>> {
        todo!()
    }

    pub(crate) fn set_swap_chain_data(
        &self,
        address: &str,
        chain_data: &SwapChainData,
        chain_info: &SwapChainInfo,
    ) -> PersistResult<()> {
        todo!()
    }
}
