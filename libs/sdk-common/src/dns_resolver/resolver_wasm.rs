use anyhow::{anyhow, Result};
use dns_parser::{Builder, Packet, RData, ResponseCode};
use dns_parser::{QueryClass, QueryType};

use crate::utils::rest_client;

pub(crate) async fn txt_lookup(dns_name: String) -> Result<Vec<String>> {
    let mut builder = Builder::new_query(1, true);
    builder.add_question(&dns_name, false, QueryType::TXT, QueryClass::IN);
    let req_bytes = builder
        .build()
        .map_err(|_| anyhow!("Error building DNS query"))?;
    let client = rest_client::get_reqwest_client()?;
    let res_bytes = client
        .post("https://cloudflare-dns.com/dns-query")
        .body(req_bytes)
        .header("Accept", "application/dns-message")
        .header("Content-Type", "application/dns-message")
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;
    let packet = Packet::parse(&res_bytes)?;
    if packet.header.response_code != ResponseCode::NoError {
        return Err(anyhow!(
            "Received error response from DNS query: {}",
            packet.header.response_code
        ));
    }
    let res = packet
        .answers
        .into_iter()
        .filter_map(|answer| {
            if let RData::TXT(txt) = answer.data {
                Some(
                    txt.iter()
                        .filter_map(|t| std::str::from_utf8(t).ok())
                        .collect::<Vec<_>>()
                        .concat(),
                )
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Ok(res)
}
