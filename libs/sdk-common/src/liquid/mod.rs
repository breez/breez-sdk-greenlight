pub mod bip21;
pub use bip21::*;

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};
    use elements::AssetId;

    use crate::input_parser::tests::get_bip21_rounding_test_vectors;
    use crate::prelude::*;

    #[cfg(all(target_family = "wasm", target_os = "unknown"))]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[sdk_macros::async_test_all]
    async fn test_liquid_address_bip21_rounding() -> Result<()> {
        let asset_id = AssetId::LIQUID_BTC.to_string();
        for (amount_sat, amount_btc) in get_bip21_rounding_test_vectors() {
            let addr = format!("liquidnetwork:tlq1qqw5ur50rnvcx33vmljjtnez3hrtl6n7vs44tdj2c9fmnxrrgzgwnhw6jtpn8cljkmlr8tgfw9hemrr5y8u2nu024hhak3tpdk?amount={amount_btc}&assetid={asset_id}");

            match parse(&addr, None).await? {
                InputType::LiquidAddress {
                    address: addr_with_amount_parsed,
                } => {
                    assert_eq!(addr_with_amount_parsed.amount_sat, Some(amount_sat));
                }
                _ => return Err(anyhow!("Invalid type parsed")),
            }
        }

        Ok(())
    }

    #[sdk_macros::async_test_all]
    async fn test_liquid_address_bip21_rounding_reverse() -> Result<()> {
        for (amount_sat, amount_btc) in get_bip21_rounding_test_vectors() {
            let data = LiquidAddressData {
                address: "tlq1qqw5ur50rnvcx33vmljjtnez3hrtl6n7vs44tdj2c9fmnxrrgzgwnhw6jtpn8cljkmlr8tgfw9hemrr5y8u2nu024hhak3tpdk".to_string(),
                network: crate::model::Network::Bitcoin,
                asset_id: Some(AssetId::LIQUID_BTC.to_string()),
                amount: None,
                amount_sat: Some(amount_sat),
                label: None,
                message: None,
            };

            let serialized = data
                .to_uri()
                .map_err(|e| anyhow!("BIP21 URI serialization error {e:?}"))?;

            assert!(serialized.contains(&format!("amount={amount_btc}")));
        }

        Ok(())
    }
}
