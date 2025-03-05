use anyhow::Result;
use hickory_resolver::config::{ResolverConfig, ResolverOpts};
use hickory_resolver::TokioAsyncResolver;
use lazy_static::lazy_static;

lazy_static! {
    static ref DNS_RESOLVER: TokioAsyncResolver = {
        let mut opts = ResolverOpts::default();
        opts.validate = true;

        TokioAsyncResolver::tokio(ResolverConfig::default(), opts)
    };
}

pub(crate) async fn txt_lookup(dns_name: String) -> Result<Vec<String>> {
    let txt_lookup = DNS_RESOLVER.txt_lookup(dns_name).await?;
    let records: Vec<String> = txt_lookup.iter().map(|r| r.to_string()).collect();
    Ok(records)
}
