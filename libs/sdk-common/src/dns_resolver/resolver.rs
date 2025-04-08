use anyhow::Result;
use hickory_resolver::{
    config::{ResolverConfig, ResolverOpts},
    proto::{
        rr::{dnssec::Algorithm, DNSSECRecordType, Record},
        serialize::binary::BinEncodable,
    },
    AsyncResolver,
};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait::async_trait]
pub trait DnsResolver {
    async fn txt_lookup_with_dnssec(&self, name: String) -> Result<(Vec<String>, Option<DnssecProof>)>;
}

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
        Ok((txt_records, None))
    }
}

fn validate_dnssec_records(records: &[Record]) -> Result<()> {
    for record in records {
        if let Some(dnskey) = record.data() {
            // Check RSA key size
            if dnskey.algorithm() == Algorithm::RSASHA1 
                || dnskey.algorithm() == Algorithm::RSASHA1NSEC3SHA1 {
                return Err(anyhow::anyhow!("SHA-1 signatures are not allowed"));
            }

            // Check RSA key size if it's an RSA key
            if matches!(dnskey.algorithm(), 
                Algorithm::RSASHA256 | 
                Algorithm::RSASHA512) {
                let key_data = dnskey.to_bytes()?;
                if key_data.len() * 8 < 1024 {
                    return Err(anyhow::anyhow!("RSA key size is less than 1024 bits"));
                }
            }
        }
    }

    Ok(())
}
