use anyhow::{anyhow, Result};
use bitcoin::psbt::raw::ProprietaryKey;
use bitcoin::psbt::{Output, PartiallySignedTransaction};
use log::error;

const PSBT_OUT_DNSSEC_PROOF: u8 = 0x35;

pub fn add_dnssec_proof(psbt: &mut PartiallySignedTransaction, output_index: usize, name: &str, proof: &[u8]) -> Result<()> {
    if output_index >= psbt.outputs.len() {
        return Err(anyhow!("Output index out of bounds"));
    }

    let output = &mut psbt.outputs[output_index];
    
    // Create the proprietary key for BIP 353 DNSSEC proof
    let key = ProprietaryKey {
        prefix: b"BIP353".to_vec(),
        subtype: PSBT_OUT_DNSSEC_PROOF,
        key: vec![],
    };

    // Construct the value: <1-byte-length-prefixed name><proof>
    let mut value = vec![name.len() as u8];
    value.extend_from_slice(name.as_bytes());
    value.extend_from_slice(proof);

    // Add the proof to the PSBT output
    output.proprietary.insert(key, value);

    Ok(())
}

pub fn get_dnssec_proof(output: &Output) -> Option<(String, Vec<u8>)> {
    for (key, value) in &output.proprietary {
        if key.prefix == b"BIP353" && key.subtype == PSBT_OUT_DNSSEC_PROOF {
            if value.is_empty() {
                error!("Empty DNSSEC proof value");
                return None;
            }

            let name_len = value[0] as usize;
            if name_len + 1 > value.len() {
                error!("Invalid DNSSEC proof format");
                return None;
            }

            let name = String::from_utf8_lossy(&value[1..name_len + 1]).to_string();
            let proof = value[name_len + 1..].to_vec();

            return Some((name, proof));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::psbt::Psbt;

    #[test]
    fn test_dnssec_proof() -> Result<()> {
        let mut psbt = Psbt::new_v2(vec![]);
        psbt.outputs.push(Output::default());

        let test_name = "user@example.com";
        let test_proof = vec![1, 2, 3, 4];

        // Add proof
        add_dnssec_proof(&mut psbt, 0, test_name, &test_proof)?;

        // Get proof
        let (name, proof) = get_dnssec_proof(&psbt.outputs[0])
            .ok_or_else(|| anyhow!("Failed to get DNSSEC proof"))?;

        assert_eq!(name, test_name);
        assert_eq!(proof, test_proof);

        Ok(())
    }
} 