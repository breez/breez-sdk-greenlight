use anyhow::{anyhow, Result};
use log::error;
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    proto::{
        rr::{dnssec::Algorithm, DNSSECRecordType, Record},
        serialize::binary::BinEncodable,
    },
    AsyncResolver,
};

const MIN_RSA_KEY_SIZE: u32 = 1024;

pub struct DnssecProof {
    pub authentication_chain: Vec<Record>,
}

pub async fn txt_lookup_with_dnssec(name: String) -> Result<(Vec<String>, Option<DnssecProof>)> {
    let mut opts = ResolverOpts::default();
    opts.validate = true;  // Enable DNSSEC validation
    opts.dnssec_ok = true;

    let resolver = AsyncResolver::tokio(ResolverConfig::default(), opts)?;
    let response = resolver.txt_lookup(name).await?;

    let mut txt_records = Vec::new();
    let mut dnssec_records = Vec::new();

    for record in response.as_lookup().records() {
        // Extract TXT records
        if let Some(txt) = record.data() {
            if let Some(txt_data) = txt.as_txt() {
                txt_records.push(txt_data.to_string());
            }
        }

        // Collect DNSSEC records
        if record.record_type() == DNSSECRecordType::DNSKEY.into() 
            || record.record_type() == DNSSECRecordType::DS.into()
            || record.record_type() == DNSSECRecordType::RRSIG.into() {
            dnssec_records.push(record.clone());
        }
    }

    // Validate DNSSEC records
    if !dnssec_records.is_empty() {
        validate_dnssec_records(&dnssec_records)?;
        Ok((txt_records, Some(DnssecProof {
            authentication_chain: dnssec_records,
        })))
    } else {
        error!("No DNSSEC records found");
        Ok((txt_records, None))
    }
}

fn validate_dnssec_records(records: &[Record]) -> Result<()> {
    for record in records {
        if let Some(dnskey) = record.data() {
            // Check RSA key size
            if dnskey.algorithm() == Algorithm::RSASHA1 
                || dnskey.algorithm() == Algorithm::RSASHA1NSEC3SHA1 {
                error!("SHA-1 signatures are not allowed");
                return Err(anyhow!("SHA-1 signatures are not allowed"));
            }

            // Check RSA key size if it's an RSA key
            if matches!(dnskey.algorithm(), 
                Algorithm::RSASHA256 | 
                Algorithm::RSASHA512) {
                let key_data = dnskey.to_bytes()?;
                if key_data.len() * 8 < MIN_RSA_KEY_SIZE as usize {
                    error!("RSA key size is less than 1024 bits");
                    return Err(anyhow!("RSA key size is less than 1024 bits"));
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Mutex;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref MOCK_TXT_RECORDS: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
    }

    pub fn set_mock_txt_records(domain: String, records: Vec<String>) {
        MOCK_TXT_RECORDS.lock().unwrap().insert(domain, records);
    }

    pub fn clear_mock_txt_records() {
        MOCK_TXT_RECORDS.lock().unwrap().clear();
    }

    #[tokio::test]
    async fn test_txt_lookup() {
        // Test cases will be added here
    }
} 