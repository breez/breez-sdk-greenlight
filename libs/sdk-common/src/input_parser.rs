use std::collections::HashMap;
use std::ops::Not;
use std::str::FromStr;

use ::bip21::Uri;
use anyhow::{anyhow, bail, Context, Result};
use bitcoin::bech32;
use bitcoin::bech32::FromBase32;
use log::{debug, error};
use percent_encoding::NON_ALPHANUMERIC;
use regex::Regex;
use serde::{Deserialize, Serialize};
use LnUrlRequestData::*;

use crate::prelude::*;
use crate::utils::Arc;

const USER_BITCOIN_PAYMENT_PREFIX: &str = "user._bitcoin-payment";
const BOLT12_PREFIX: &str = "lno";
const LNURL_PAY_PREFIX: &str = "lnurl";
const BIP353_PREFIX: &str = "bitcoin:";
const LIGHTNING_PREFIX: &str = "lightning";

/// Parses generic user input, typically pasted from clipboard or scanned from a QR.
///
/// Can optionally be provided a collection of [ExternalInputParser] that are used if an input is not
/// recognized.
///
/// # Examples
///
/// ## On-chain BTC addresses (incl. BIP 21 URIs)
///
/// ```
/// use sdk_common::prelude::{InputType::*, parse};
///
/// #[tokio::main]
/// async fn main() {
///     assert!(matches!( parse("1andreas3batLhQa2FawWjeyjCqyBzypd", None).await, Ok(BitcoinAddress{address: _}) ));
///     assert!(matches!( parse("1andreas3batLhQa2FawWjeyjCqyBzypd?amount=0.00002000", None).await, Ok(BitcoinAddress{address: _}) ));
///     assert!(matches!( parse("1andreas3batLhQa2FawWjeyjCqyBzypd?amount=0.00002000&label=Hello", None).await, Ok(BitcoinAddress{address: _}) ));
///     assert!(matches!( parse("1andreas3batLhQa2FawWjeyjCqyBzypd?amount=0.00002000&label=Hello&message=Msg", None).await, Ok(BitcoinAddress{address: _}) ));
///
///     assert!(matches!( parse("BITCOIN:1andreas3batLhQa2FawWjeyjCqyBzypd", None).await, Ok(BitcoinAddress{address: _}) ));
///     assert!(matches!( parse("bitcoin:1andreas3batLhQa2FawWjeyjCqyBzypd", None).await, Ok(BitcoinAddress{address: _}) ));
///     assert!(matches!( parse("bitcoin:1andreas3batLhQa2FawWjeyjCqyBzypd?amount=0.00002000", None).await, Ok(BitcoinAddress{address: _}) ));
///     assert!(matches!( parse("bitcoin:1andreas3batLhQa2FawWjeyjCqyBzypd?amount=0.00002000&label=Hello", None).await, Ok(BitcoinAddress{address: _}) ));
///     assert!(matches!( parse("BITCOIN:1andreas3batLhQa2FawWjeyjCqyBzypd?amount=0.00002000&label=Hello&message=Msg", None).await, Ok(BitcoinAddress{address: _}) ));
///     assert!(matches!( parse("bitcoin:1andreas3batLhQa2FawWjeyjCqyBzypd?amount=0.00002000&label=Hello&message=Msg", None).await, Ok(BitcoinAddress{address: _}) ));
/// }
/// ```
///
/// ## BOLT 11 invoices
///
/// ```
/// use sdk_common::prelude::{InputType::*, parse};
///
/// #[tokio::main]
/// async fn main() {
///     let invoice = "lnbc110n1p38q3gtpp5ypz09jrd8p993snjwnm68cph4ftwp22le34xd4r8ftspwshxhmnsdqqxqyjw5qcqpxsp5htlg8ydpywvsa7h3u4hdn77ehs4z4e844em0apjyvmqfkzqhhd2q9qgsqqqyssqszpxzxt9uuqzymr7zxcdccj5g69s8q7zzjs7sgxn9ejhnvdh6gqjcy22mss2yexunagm5r2gqczh8k24cwrqml3njskm548aruhpwssq9nvrvz";
///     assert!(matches!( parse(invoice, None).await, Ok(Bolt11{invoice: _}) ));
///     assert!(matches!( parse( &format!("lightning:{}", invoice), None).await, Ok(Bolt11{invoice: _}) ));
///
///     // BIP 21 with LN fallback parses to a LN invoice
///     let btc_address = "1andreas3batLhQa2FawWjeyjCqyBzypd";
///     assert!(matches!( parse( &format!("bitcoin:{}?lightning={}", btc_address, invoice), None).await, Ok(Bolt11{invoice: _}) ));
/// }
/// ```
///
/// ## Web URLs
///
/// ```
/// use sdk_common::prelude::{InputType::*, parse};
///
/// #[tokio::main]
/// async fn main() {
///     assert!(matches!( parse("https://breez.technology", None).await, Ok(Url{url: _}) ));
///     assert!(matches!( parse("https://breez.technology/test-path", None).await, Ok(Url{url: _}) ));
///     assert!(matches!( parse("https://breez.technology/test-path?arg=val", None).await, Ok(Url{url: _}) ));
/// }
/// ```
///
/// ### Web URLs with `lightning` query param with an LNURL value.
///
/// ```no_run
/// use sdk_common::prelude::{InputType::*, parse};
///
/// #[tokio::main]
/// async fn main() {
///     assert!(matches!( parse("https://breez.technology?lightning=lnurl1d...", None).await, Ok(LnUrlWithdraw{data: _}) ));
/// }
/// ```
///
/// ## LNURL
///
/// Both the bech32 and the raw (non-bech32, but with specific prefixes) variants are supported.
///
///
/// ### LNURL pay request
///
/// ```no_run
/// use sdk_common::prelude::{InputType::*, LnUrlRequestData::*, parse};
/// use anyhow::Result;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let lnurl_pay_url = "lnurl1dp68gurn8ghj7mr0vdskc6r0wd6z7mrww4excttsv9un7um9wdekjmmw84jxywf5x43rvv35xgmr2enrxanr2cfcvsmnwe3jxcukvde48qukgdec89snwde3vfjxvepjxpjnjvtpxd3kvdnxx5crxwpjvyunsephsz36jf";
///
///     assert!(matches!( parse(lnurl_pay_url, None).await, Ok(LnUrlPay{data: _, bip353_address: _}) ));
///     // assert!(matches!( parse("lnurlp://domain.com/lnurl-pay?key=val", None).await, Ok(LnUrlPay{data: _}) ));
///     // assert!(matches!( parse("lightning@address.com", None).await, Ok(LnUrlPay{data: _}) ));
///
///     if let Ok(LnUrlPay{data: pd, bip353_address}) = parse(lnurl_pay_url, None).await {
///         assert_eq!(pd.callback, "https://localhost/lnurl-pay/callback/db945b624265fc7f5a8d77f269f7589d789a771bdfd20e91a3cf6f50382a98d7");
///         assert_eq!(pd.max_sendable, 16000); // Max sendable amount, in msats
///         assert_eq!(pd.max_sendable_sats(), 16); // Max sendable amount, in sats
///         assert_eq!(pd.min_sendable, 4000); // Min sendable amount, in msats
///         assert_eq!(pd.min_sendable_sats(), 4); // Min sendable amount, in sats
///         assert_eq!(pd.comment_allowed, 0);
///         assert_eq!(pd.metadata_vec()?.len(), 3);
///     }
///
///     Ok(())
/// }
/// ```
///
/// ### LNURL withdraw request
///
/// ```no_run
/// use sdk_common::prelude::{InputType::*, LnUrlRequestData::*, parse};
///
/// #[tokio::main]
/// async fn main() {
///     let lnurl_withdraw_url = "lnurl1dp68gurn8ghj7mr0vdskc6r0wd6z7mrww4exctthd96xserjv9mn7um9wdekjmmw843xxwpexdnxzen9vgunsvfexq6rvdecx93rgdmyxcuxverrvcursenpxvukzv3c8qunsdecx33nzwpnvg6ryc3hv93nzvecxgcxgwp3h33lxk";
///
///     assert!(matches!( parse(lnurl_withdraw_url, None).await, Ok(LnUrlWithdraw{data: _}) ));
///
///     if let Ok(LnUrlWithdraw{data: wd}) = parse(lnurl_withdraw_url, None).await {
///         assert_eq!(wd.callback, "https://localhost/lnurl-withdraw/callback/e464f841c44dbdd86cee4f09f4ccd3ced58d2e24f148730ec192748317b74538");
///         assert_eq!(wd.k1, "37b4c919f871c090830cc47b92a544a30097f03430bc39670b8ec0da89f01a81");
///         assert_eq!(wd.min_withdrawable, 3000); // Min withdrawable amount, in msats
///         assert_eq!(wd.min_withdrawable_sats(), 3); // Min withdrawable amount, in sats
///         assert_eq!(wd.max_withdrawable, 12000); // Max withdrawable amount, in msats
///         assert_eq!(wd.max_withdrawable_sats(), 12); // Max withdrawable amount, in sats
///         assert_eq!(wd.default_description, "sample withdraw");
///     }
/// }
/// ```
///
/// ### LNURL auth request
///
/// ```no_run
/// use sdk_common::prelude::{InputType::*, LnUrlRequestData::*, parse};
///
/// #[tokio::main]
/// async fn main() {
///     let lnurl_auth_url = "lnurl1dp68gurn8ghj7mr0vdskc6r0wd6z7mrww4excttvdankjm3lw3skw0tvdankjm3xdvcn6vtp8q6n2dfsx5mrjwtrxdjnqvtzv56rzcnyv3jrxv3sxqmkyenrvv6kve3exv6nqdtyv43nqcmzvdsnvdrzx33rsenxx5unqc3cxgeqgntfgu";
///
///     assert!(matches!( parse(lnurl_auth_url, None).await, Ok(LnUrlAuth{data: _}) ));
///
///     if let Ok(LnUrlAuth{data: ad}) = parse(lnurl_auth_url, None).await {
///         assert_eq!(ad.k1, "1a855505699c3e01be41bddd32007bfcc5ff93505dec0cbca64b4b8ff590b822");
///     }
/// }
/// ```
///
/// ## External input parsing
///
/// ```no_run
/// use sdk_common::prelude::{ExternalInputParser, InputType::*, parse};
///
/// #[tokio::main]
/// async fn main() {
///     let external_parser = ExternalInputParser {
///         provider_id: "provider_id".to_string(),
///         input_regex: "(.*)(provider.domain)(.*)".to_string(),
///         parser_url: "http://external-parser-domain.com/<input>".to_string(),
///     };
///
///     let data = "151931provider.domain069135";
///
///     // Parse will make an http GET request to http://external-parser-domain.com/151931provider.domain069135
///     // The following assertion assumes the response contains a bolt11 invoice
///     if let Ok(Bolt11 {invoice}) = parse(data, Some(&[external_parser])).await {
///         assert_eq!(invoice.bolt11, "lnbc110n1p38q3gtpp5ypz09jrd8p993snjwnm68cph4ftwp22le34xd4r8ftspwshxhmnsdqqxqyjw5qcqpxsp5htlg8ydpywvsa7h3u4hdn77ehs4z4e844em0apjyvmqfkzqhhd2q9qgsqqqyssqszpxzxt9uuqzymr7zxcdccj5g69s8q7zzjs7sgxn9ejhnvdh6gqjcy22mss2yexunagm5r2gqczh8k24cwrqml3njskm548aruhpwssq9nvrvz");
///     }
/// }
/// ```
pub async fn parse(
    input: &str,
    external_input_parsers: Option<&[ExternalInputParser]>,
) -> Result<InputType> {
    let rest_client: Arc<dyn RestClient> = Arc::new(ReqwestRestClient::new()?);
    parse_with_rest_client(rest_client.as_ref(), input, external_input_parsers).await
}

pub async fn parse_with_rest_client<C: RestClient + ?Sized>(
    rest_client: &C,
    input: &str,
    external_input_parsers: Option<&[ExternalInputParser]>,
) -> Result<InputType> {
    let input = input.trim();

    // Try to parse the destination as a bip353 address.
    let (bip353_parsed_input, is_bip353) = match bip353_parse(input).await {
        Some(value) => (value, true),
        None => (input.to_string(), false),
    };

    if let Ok(input_type) = parse_core(rest_client, &bip353_parsed_input).await {
        let input_type = if is_bip353 {
            match input_type {
                #[cfg(feature = "liquid")]
                InputType::Bolt12Offer { offer, .. } => InputType::Bolt12Offer {
                    offer,
                    bip353_address: Some(input.to_string()),
                },
                InputType::LnUrlPay { data, .. } => InputType::LnUrlPay {
                    data,
                    bip353_address: Some(input.to_string()),
                },
                i => bail!("Unexpected input type was resolved from a BIP353 address: {i:?}"),
            }
        } else {
            input_type
        };

        return Ok(input_type);
    }

    if let Some(external_input_parsers) = external_input_parsers {
        return parse_external(rest_client, input, external_input_parsers).await;
    }

    Err(anyhow!("Unrecognized input type"))
}

fn get_by_key(tuple_vector: &[(&str, &str)], key: &str) -> Option<String> {
    tuple_vector
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| v.to_string())
}

fn parse_bip353_metadata(query_params: &[(&str, &str)]) -> Bip353Metadata {
    Bip353Metadata {
        label: get_by_key(query_params, "label"),
        message: get_by_key(query_params, "message"),
        amount: get_by_key(query_params, "amount")
            .and_then(|a| a.parse::<f64>().ok())
            .map(|btc| (btc * 100_000_000.0) as u64),
    }
}

fn concatenate_txt_records(records: Vec<String>) -> Option<String> {
    // As per RFC 1035, TXT records are one or more character-strings
    // Each character-string is limited to 255 characters
    records.into_iter().find(|r| r.starts_with(BIP353_PREFIX))
}

fn parse_bip353_record(bip353_record: String) -> Option<String> {
    let (_, query_part) = bip353_record.split_once("?")?;
    let query_params = querystring::querify(query_part);

    // Try BOLT12 and LNURL-pay first
    if let Some(value) = get_by_key(&query_params, BOLT12_PREFIX)
        .or_else(|| get_by_key(&query_params, LNURL_PAY_PREFIX))
    {
        return Some(value);
    }

    // Try lightning= parameter for BOLT11
    get_by_key(&query_params, "lightning")
}

fn is_valid_bip353_record(decoded: &str) -> bool {
    if !decoded.to_lowercase().starts_with(BIP353_PREFIX) {
        error!(
            "Invalid decoded TXT data (doesn't begin with: {})",
            BIP353_PREFIX
        );
        return false;
    }

    // Validate record format according to BIP 0353
    let parts: Vec<&str> = decoded.splitn(2, '?').collect();
    if parts.len() != 2 {
        error!("Invalid BIP353 record format: missing query parameters");
        return false;
    }

    true
}

fn extract_bip353_record(records: Vec<String>) -> Option<String> {
    // As per BIP 0353: "Resolvers encountering multiple "bitcoin:"-matching TXT records 
    // at the same label MUST treat the records as invalid"
    let bip353_records: Vec<String> = records
        .into_iter()
        .filter(|record| is_valid_bip353_record(record))
        .collect();

    match bip353_records.len() {
        0 => None,
        1 => concatenate_txt_records(bip353_records),
        _ => {
            error!("Multiple BIP353 records found - invalid according to spec");
            None
        }
    }
}

async fn bip353_parse(input: &str) -> Option<String> {
    let (local_part, domain) = input.split_once('@')?;
    
    // Validate both parts are within the DNS label size limit.
    // See <https://datatracker.ietf.org/doc/html/rfc1035#section-2.3.4>
    if local_part.len() > 63 || domain.len() > 63 {
        error!("BIP353: Local part or domain exceeds DNS label size limit of 63 characters");
        return None;
    }

    // Validate local part characters according to BIP 0353
    if !local_part.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.') {
        error!("BIP353: Local part contains invalid characters");
        return None;
    }

    // Query for TXT records of a domain with DNSSEC validation
    let dns_name = format!("{}.{}.{}", local_part, USER_BITCOIN_PAYMENT_PREFIX, domain);
    let (records, dnssec_proof) = match dns_resolver::txt_lookup_with_dnssec(dns_name).await {
        Ok((records, Some(proof))) => (records, proof),
        Ok((_, None)) => {
            error!("BIP353: Missing required DNSSEC signatures");
            return None;
        }
        Err(e) => {
            debug!("No BIP353 TXT records found or DNSSEC validation failed: {}", e);
            return None;
        }
    };

    let bip353_record = extract_bip353_record(records)?;
    parse_bip353_record(bip353_record)
}

/// Core parse implementation
async fn parse_core<C: RestClient + ?Sized>(rest_client: &C, input: &str) -> Result<InputType> {
    // Covers BIP 21 URIs and simple onchain BTC addresses (which are valid BIP 21 with the 'bitcoin:' prefix)
    if let Ok(bip21_uri) = prepend_if_missing("bitcoin:", input).parse::<Uri<'_>>() {
        let bitcoin_addr_data = bip21_uri.into();

        // Special case of LN BOLT11 with onchain fallback
        // Search for the `lightning=bolt11` param in the BIP21 URI and, if found, extract the bolt11
        let mut invoice_param: Option<LNInvoice> = None;
        if let Some(query) = input.split('?').collect::<Vec<_>>().get(1) {
            invoice_param = querystring::querify(query)
                .iter()
                .find(|(key, _)| key == &"lightning")
                .map(|(_, value)| parse_invoice(value))
                .transpose()?;
        }

        return match invoice_param {
            None => Ok(InputType::BitcoinAddress {
                address: bitcoin_addr_data,
            }),
            Some(invoice) => Ok(InputType::Bolt11 { invoice }),
        };
    }

    #[cfg(feature = "liquid")]
    if let Ok(address) = parse_liquid_address(input) {
        return Ok(InputType::LiquidAddress { address });
    }

    #[cfg(feature = "liquid")]
    if let Ok(offer) = parse_bolt12_offer(input) {
        return Ok(InputType::Bolt12Offer {
            offer,
            bip353_address: None,
        });
    }

    if let Ok(invoice) = parse_invoice(input) {
        return Ok(InputType::Bolt11 { invoice });
    }

    // Public key serialized in compressed form (66 hex chars)
    if let Ok(_node_id) = bitcoin::secp256k1::PublicKey::from_str(input) {
        return Ok(InputType::NodeId {
            node_id: input.into(),
        });
    }

    // Possible Node URI (check for separator symbol, try to parse pubkey, ignore rest)
    if let Some('@') = input.chars().nth(66) {
        if let Ok(_node_id) = bitcoin::secp256k1::PublicKey::from_str(&input[..66]) {
            return Ok(InputType::NodeId {
                node_id: input.into(),
            });
        }
    }

    if let Ok(url) = reqwest::Url::parse(input) {
        if ["http", "https"].contains(&url.scheme()) {
            if let Some((_key, value)) = url
                .query_pairs()
                .find(|p| p.0 == "lightning" || p.0 == "LIGHTNING")
            {
                if let Ok((domain, lnurl_endpoint, ln_address)) = lnurl_decode(&value) {
                    return resolve_lnurl(rest_client, domain, lnurl_endpoint, ln_address).await;
                }
            }
            return Ok(InputType::Url { url: input.into() });
        }
    }

    // Try to strip the "lightning:" prefix from possible lnurl string. If prefix is not there, default to original input
    let input = input
        .strip_prefix("lightning:")
        .or(input.strip_prefix("LIGHTNING:"))
        .unwrap_or(input);
    if let Ok((domain, lnurl_endpoint, ln_address)) = lnurl_decode(input) {
        return resolve_lnurl(rest_client, domain, lnurl_endpoint, ln_address).await;
    }

    Err(anyhow!("Unrecognized input type"))
}

/// Parse input using provided external parsers.
async fn parse_external<C: RestClient + ?Sized>(
    rest_client: &C,
    input: &str,
    external_input_parsers: &[ExternalInputParser],
) -> Result<InputType> {
    for parser in external_input_parsers {
        // Check regex
        let re = Regex::new(&parser.input_regex).context(format!(
            "Couldn't parse regex {} for provider {}",
            parser.input_regex, parser.provider_id
        ))?;
        if re.is_match(input).not() {
            continue;
        }

        // Build URL
        let urlsafe_input =
            percent_encoding::utf8_percent_encode(input, NON_ALPHANUMERIC).to_string();
        let parser_url = parser.parser_url.replacen("<input>", &urlsafe_input, 1);

        // Make request
        let (response, _) = match get_and_check_success(rest_client, &parser_url).await {
            Ok(response) => response,
            Err(e) => {
                error!("Request to external input parser {parser:?} failed: {e}");
                continue;
            }
        };

        // Try to parse as LnUrlRequestData
        if let Ok(lnurl_data) = serde_json::from_str::<LnUrlRequestData>(&response) {
            let domain = url::Url::parse(&parser_url)
                .ok()
                .and_then(|url| url.host_str().map(|s| s.to_string()))
                .unwrap_or_default();
            let input_type = lnurl_data.into();
            let input_type = match input_type {
                // Modify the LnUrlPay payload by adding the domain of the LNURL endpoint
                InputType::LnUrlPay { data, .. } => InputType::LnUrlPay {
                    data: LnUrlPayRequestData { domain, ..data },
                    bip353_address: None,
                },
                _ => input_type,
            };
            return Ok(input_type);
        }

        // Check other input types
        if let Ok(input_type) = parse_core(rest_client, &response).await {
            return Ok(input_type);
        }
    }

    Err(anyhow!("Unrecognized input type"))
}

/// Prepends the given prefix to the input, if the input doesn't already start with it
fn prepend_if_missing(prefix: &str, input: &str) -> String {
    match input.to_lowercase().starts_with(prefix) {
        true => input.into(),
        false => format!("{}{}", prefix, input.trim_start_matches(prefix)),
    }
}

/// Converts the LN Address to the corresponding LNURL-pay endpoint, as per LUD-16:
///
/// - https://<domain>/.well-known/lnurlp/<username> for clearnet domains
/// - http://<domain>/.well-known/lnurlp/<username> for onion domains
///
/// Valid characters for the username are `a-z0-9-_.`, however the function
/// tolerates capital letters by downcasing the address.
///
/// The result is a tuple of (domain, LNURL-pay endpoint, downcased lightning address)
fn ln_address_decode(ln_address: &str) -> Result<(String, String, String)> {
    if ln_address.contains('@') {
        let split = ln_address.split('@').collect::<Vec<&str>>();
        let user = split[0].to_lowercase();

        // BIP-353 addresses have a ₿ prefix. Some users will want to use it as
        // lnurl, so strip the prefix if it's there.
        let user = user
            .strip_prefix('₿')
            .map(|p| p.to_string())
            .unwrap_or(user);
        // It is safe to downcase the domains since they are case-insensitive.
        // https://www.rfc-editor.org/rfc/rfc3986#section-3.2.2
        let domain = split[1].to_lowercase();

        if !user
            .chars()
            .all(|c| c.is_alphanumeric() || ['-', '_', '.'].contains(&c))
        {
            return Err(anyhow!("Invalid username"));
        }

        let schema = match domain.ends_with(".onion") {
            true => "http://",
            false => "https://",
        };

        return Ok((
            domain.clone(),
            format!("{schema}{domain}/.well-known/lnurlp/{user}"),
            format!("{user}@{domain}"),
        ));
    }

    Err(anyhow!("Invalid LN address"))
}

/// Decodes the input to a human-readable http or https LNURL. Returns a tuple of (domain, url, lightning address if present).
///
/// It can handle three kinds of input:
///
/// - bech32-based (LUD-01), like LNURL1...
/// - LN addresses (LUD-16), like user@domain.com
/// - prefix-based (LUD-17), like lnurlp:// or lnurlp:
///
/// ## Validation notes
///
/// For bech32-encoded LNURLs, the only allowed schemes are http (for onion domains) and https (for clearnet domains).
///
/// LNURLs in all uppercase or all lowercase are valid, but mixed case ones are invalid.
///
/// For LN addresses, the username is limited to `a-z0-9-_.`, which is more restrictive than email addresses.
/// However the function tolerates capital letters by downcasing the address.
fn lnurl_decode(encoded: &str) -> LnUrlResult<(String, String, Option<String>)> {
    if let Ok((domain, url, ln_address)) = ln_address_decode(encoded) {
        return Ok((domain, url, Some(ln_address)));
    }

    match bech32::decode(encoded) {
        Ok((_hrp, payload, _variant)) => {
            let decoded = String::from_utf8(Vec::from_base32(&payload)?)?;

            let url = reqwest::Url::parse(&decoded)
                .map_err(|e| super::prelude::LnUrlError::InvalidUri(e.to_string()))?;
            let domain = url.domain().ok_or_else(|| {
                super::prelude::LnUrlError::invalid_uri("Could not determine domain")
            })?;

            if url.scheme() == "http" && !domain.ends_with(".onion") {
                return Err(super::prelude::LnUrlError::generic(
                    "HTTP scheme only allowed for onion domains",
                ));
            }
            if url.scheme() == "https" && domain.ends_with(".onion") {
                return Err(super::prelude::LnUrlError::generic(
                    "HTTPS scheme not allowed for onion domains",
                ));
            }

            Ok((domain.into(), decoded, None))
        }
        Err(_) => {
            let supported_prefixes = ["lnurlp", "lnurlw", "keyauth"];
            let mut encoded = encoded.to_string();

            // Treat prefix: and prefix:// the same, to cover both vendor implementations
            // https://github.com/lnbits/lnbits/pull/762#issue-1309702380
            for pref in supported_prefixes {
                let scheme_simple = &format!("{pref}:");
                let scheme_authority = &format!("{pref}://");
                if encoded.starts_with(scheme_simple) && !encoded.starts_with(scheme_authority) {
                    encoded = encoded.replacen(scheme_simple, scheme_authority, 1);
                    break;
                }
            }

            let url = reqwest::Url::parse(&encoded)
                .map_err(|e| super::prelude::LnUrlError::InvalidUri(e.to_string()))?;
            let domain = url.domain().ok_or_else(|| {
                super::prelude::LnUrlError::invalid_uri("Could not determine domain")
            })?;
            ensure_sdk!(
                supported_prefixes.contains(&url.scheme()),
                super::prelude::LnUrlError::generic("Invalid prefix scheme")
            );

            let scheme = url.scheme();
            let new_scheme = match domain.ends_with(".onion") {
                true => "http",
                false => "https",
            };

            Ok((domain.into(), encoded.replacen(scheme, new_scheme, 1), None))
        }
    }
}

async fn resolve_lnurl<C: RestClient + ?Sized>(
    rest_client: &C,
    domain: String,
    lnurl_endpoint: String,
    ln_address: Option<String>,
) -> Result<InputType> {
    // For LNURL-auth links, their type is already known if the link contains the login tag
    // No need to query the endpoint for details
    if lnurl_endpoint.contains("tag=login") {
        return Ok(InputType::LnUrlAuth {
            data: validate_request(domain, lnurl_endpoint)?,
        });
    }

    let (response, _) = rest_client.get(&lnurl_endpoint).await?;
    let lnurl_data: LnUrlRequestData =
        parse_json(&response).map_err(|_| anyhow!("Failed to parse response"))?;
    let temp = lnurl_data.into();
    let temp = match temp {
        // Modify the LnUrlPay payload by adding the domain of the LNURL endpoint
        InputType::LnUrlPay { data, .. } => InputType::LnUrlPay {
            data: LnUrlPayRequestData {
                domain,
                ln_address,
                ..data
            },
            bip353_address: None,
        },
        _ => temp,
    };
    Ok(temp)
}

/// Different kinds of inputs supported by [parse], including any relevant details extracted from the input
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum InputType {
    /// # Supported standards
    ///
    /// - plain on-chain BTC address
    /// - BIP21
    BitcoinAddress {
        address: BitcoinAddressData,
    },

    /// # Supported standards
    ///
    /// - plain on-chain liquid address
    /// - BIP21 on liquid/liquidtestnet/liquidregtest
    #[cfg(feature = "liquid")]
    LiquidAddress {
        address: LiquidAddressData,
    },

    /// Also covers URIs like `bitcoin:...&lightning=bolt11`. In this case, it returns the BOLT11
    /// and discards all other data.
    Bolt11 {
        invoice: LNInvoice,
    },
    #[cfg(feature = "liquid")]
    Bolt12Offer {
        offer: LNOffer,
        /// The BIP353 address from which this InputType was resolved
        bip353_address: Option<String>,
    },
    NodeId {
        node_id: String,
    },
    Url {
        url: String,
    },

    /// # Supported standards
    ///
    /// - LUD-01 LNURL bech32 encoding
    /// - LUD-06 `payRequest` spec
    /// - LUD-16 LN Address
    /// - LUD-17 Support for lnurlp prefix with non-bech32-encoded LNURL URLs
    LnUrlPay {
        data: LnUrlPayRequestData,
        /// The BIP353 address from which this InputType was resolved
        bip353_address: Option<String>,
    },

    /// # Supported standards
    ///
    /// - LUD-01 LNURL bech32 encoding
    /// - LUD-03 `withdrawRequest` spec
    /// - LUD-17 Support for lnurlw prefix with non-bech32-encoded LNURL URLs
    ///
    /// # Not supported (yet)
    ///
    /// - LUD-14 `balanceCheck`: reusable `withdrawRequest`s
    /// - LUD-19 Pay link discoverable from withdraw link
    LnUrlWithdraw {
        data: LnUrlWithdrawRequestData,
    },

    /// # Supported standards
    ///
    /// - LUD-01 LNURL bech32 encoding
    /// - LUD-04 `auth` base spec
    /// - LUD-17 Support for keyauth prefix with non-bech32-encoded LNURL URLs
    LnUrlAuth {
        data: LnUrlAuthRequestData,
    },

    /// Error returned by the LNURL endpoint
    LnUrlError {
        data: LnUrlErrorData,
    },
}

/// Generic struct containing the possible LNURL payloads returned when contacting a LNURL endpoint
// The uniffi bindings only supports enums with named fields.
// We use #[serde(flatten)] to map the JSON payload fields to the inner enum "data" field
// https://serde.rs/attr-flatten.html
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum LnUrlRequestData {
    PayRequest {
        #[serde(flatten)]
        data: LnUrlPayRequestData,
    },
    WithdrawRequest {
        #[serde(flatten)]
        data: LnUrlWithdrawRequestData,
    },
    #[serde(rename = "login")]
    AuthRequest {
        #[serde(flatten)]
        data: LnUrlAuthRequestData,
    },
    Error {
        #[serde(flatten)]
        data: LnUrlErrorData,
    },
}

impl From<LnUrlRequestData> for InputType {
    fn from(lnurl_data: LnUrlRequestData) -> Self {
        match lnurl_data {
            PayRequest { data } => Self::LnUrlPay {
                data,
                bip353_address: None,
            },
            WithdrawRequest { data } => Self::LnUrlWithdraw { data },
            AuthRequest { data } => Self::LnUrlAuth { data },
            Error { data } => Self::LnUrlError { data },
        }
    }
}

/// Wrapped in a [LnUrlPay], this is the result of [parse] when given a LNURL-pay endpoint.
///
/// It represents the endpoint's parameters for the LNURL workflow.
///
/// See <https://github.com/lnurl/luds/blob/luds/06.md>
#[derive(Clone, Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LnUrlPayRequestData {
    pub callback: String,
    /// The minimum amount, in millisats, that this LNURL-pay endpoint accepts
    pub min_sendable: u64,
    /// The maximum amount, in millisats, that this LNURL-pay endpoint accepts
    pub max_sendable: u64,
    /// As per LUD-06, `metadata` is a raw string (e.g. a json representation of the inner map).
    /// Use `metadata_vec()` to get the parsed items.
    #[serde(rename(deserialize = "metadata"))]
    pub metadata_str: String,
    /// The comment length accepted by this endpoint
    ///
    /// See <https://github.com/lnurl/luds/blob/luds/12.md>
    #[serde(default)]
    pub comment_allowed: u16,

    /// Indicates the domain of the LNURL-pay service, to be shown to the user when asking for
    /// payment input, as per LUD-06 spec.
    ///
    /// Note: this is not the domain of the callback, but the domain of the LNURL-pay endpoint.
    #[serde(skip)]
    pub domain: String,

    /// Value indicating whether the recipient supports Nostr Zaps through NIP-57.
    ///
    /// See <https://github.com/nostr-protocol/nips/blob/master/57.md>
    #[serde(default)]
    pub allows_nostr: bool,

    /// Optional recipient's lnurl provider's Nostr pubkey for NIP-57. If it exists it should be a
    /// valid BIP 340 public key in hex.
    ///
    /// See <https://github.com/nostr-protocol/nips/blob/master/57.md>
    /// See <https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki>
    pub nostr_pubkey: Option<String>,

    /// If sending to a LN Address, this will be filled.
    #[serde(skip)]
    pub ln_address: Option<String>,
}

impl LnUrlPayRequestData {
    /// The minimum amount, in sats, accepted by this LNURL-pay endpoint
    pub fn min_sendable_sats(&self) -> u64 {
        self.min_sendable / 1000
    }

    /// The maximum amount, in sats, accepted by this LNURL-pay endpoint
    pub fn max_sendable_sats(&self) -> u64 {
        self.max_sendable / 1000
    }

    /// Parsed metadata items. Use `metadata_str` to get the raw metadata string, as received from
    /// the LNURL endpoint.
    pub fn metadata_vec(&self) -> Result<Vec<MetadataItem>> {
        Ok(serde_json::from_str::<Vec<MetadataItem>>(
            &self.metadata_str,
        )?)
    }
}

impl LnUrlWithdrawRequestData {
    /// The minimum amount, in sats, accepted by this LNURL-withdraw endpoint
    pub fn min_withdrawable_sats(&self) -> u64 {
        self.min_withdrawable / 1000
    }

    /// The maximum amount, in sats, accepted by this LNURL-withdraw endpoint
    pub fn max_withdrawable_sats(&self) -> u64 {
        self.max_withdrawable / 1000
    }
}

/// Key-value pair in the [LnUrlPayRequestData], as returned by the LNURL-pay endpoint
#[derive(Deserialize, Debug)]
pub struct MetadataItem {
    pub key: String,
    pub value: String,
}

/// Wrapped in a [BitcoinAddress], this is the result of [parse] when given a plain or BIP-21 BTC address.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BitcoinAddressData {
    pub address: String,
    pub network: super::prelude::Network,
    pub amount_sat: Option<u64>,
    pub label: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug)]
pub enum URISerializationError {
    UnsupportedNetwork,
    AssetIdMissing,
    InvalidAddress,
}

impl BitcoinAddressData {
    /// Converts the structure to a BIP21 URI while also
    /// ensuring that all the fields are valid
    pub fn to_uri(&self) -> Result<String, URISerializationError> {
        self.address
            .parse::<bitcoin::Address>()
            .map_err(|_| URISerializationError::InvalidAddress)?;

        let mut optional_keys = HashMap::new();

        if let Some(amount_sat) = self.amount_sat {
            let amount_btc = amount_sat as f64 / 100_000_000.0;
            optional_keys.insert("amount", format!("{amount_btc:.8}"));
        }

        if let Some(message) = &self.message {
            optional_keys.insert("message", urlencoding::encode(message).to_string());
        }

        if let Some(label) = &self.label {
            optional_keys.insert("label", urlencoding::encode(label).to_string());
        }

        match optional_keys.is_empty() {
            true => Ok(self.address.clone()),
            false => {
                let scheme = "bitcoin";
                let suffix_str = optional_keys
                    .iter()
                    .map(|(key, value)| format!("{key}={value}"))
                    .collect::<Vec<String>>()
                    .join("&");
                Ok(format!("{scheme}:{}?{suffix_str}", self.address))
            }
        }
    }
}

impl From<Uri<'_>> for BitcoinAddressData {
    fn from(uri: Uri) -> Self {
        BitcoinAddressData {
            address: uri.address.to_string(),
            network: uri.address.network.into(),
            amount_sat: uri.amount.map(|a| a.to_sat()),
            label: uri.label.map(|label| label.try_into().unwrap()),
            message: uri.message.map(|msg| msg.try_into().unwrap()),
        }
    }
}

/// Configuration for an external input parser
#[derive(Debug, Clone, Serialize)]
pub struct ExternalInputParser {
    /// An arbitrary parser provider id
    pub provider_id: String,
    /// The external parser will be used when an input conforms to this regex
    pub input_regex: String,
    /// The URL of the parser containing a placeholder `<input>` that will be replaced with the
    /// input to be parsed. The input is sanitized using percent encoding.
    pub parser_url: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_bip353_parsing() -> Result<()> {
        // Mock DNS resolver
        let mut mock_resolver = dns_resolver::MockDnsResolver::new();
        mock_resolver
            .expect_txt_lookup_with_dnssec()
            .with(eq("user._bitcoin-payment.example.com"))
            .returning(|_| {
                Ok((
                    vec!["bitcoin:?lnurl=LNURL1...".to_string()],
                    Some(dns_resolver::DnssecProof {
                        authentication_chain: vec![],
                    }),
                ))
            });

        // Test valid BIP353 address
        let input = "user@example.com";
        let result = bip353_parse(input).await;
        assert!(result.is_some());

        // Test invalid BIP353 address (no DNSSEC)
        mock_resolver
            .expect_txt_lookup_with_dnssec()
            .with(eq("invalid._bitcoin-payment.example.com"))
            .returning(|_| Ok((vec![], None)));

        let input = "invalid@example.com";
        let result = bip353_parse(input).await;
        assert!(result.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_parse_bitcoin_addresses() -> Result<()> {
        let rest_client: Arc<dyn RestClient> = Arc::new(ReqwestRestClient::new()?);

        // Test plain BTC address
        let input = "1andreas3batLhQa2FawWjeyjCqyBzypd";
        assert!(matches!(
            parse_with_rest_client(rest_client.as_ref(), input, None).await?,
            InputType::BitcoinAddress { address: _ }
        ));

        // Test BIP21 URI with amount
        let input = "bitcoin:1andreas3batLhQa2FawWjeyjCqyBzypd?amount=0.00002000";
        assert!(matches!(
            parse_with_rest_client(rest_client.as_ref(), input, None).await?,
            InputType::BitcoinAddress { address: _ }
        ));

        // Test BIP21 URI with amount and label
        let input = "bitcoin:1andreas3batLhQa2FawWjeyjCqyBzypd?amount=0.00002000&label=Hello";
        assert!(matches!(
            parse_with_rest_client(rest_client.as_ref(), input, None).await?,
            InputType::BitcoinAddress { address: _ }
        ));

        // Test BIP21 URI with amount, label and message
        let input = "bitcoin:1andreas3batLhQa2FawWjeyjCqyBzypd?amount=0.00002000&label=Hello&message=Msg";
        assert!(matches!(
            parse_with_rest_client(rest_client.as_ref(), input, None).await?,
            InputType::BitcoinAddress { address: _ }
        ));

        Ok(())
    }

    #[tokio::test]
    async fn test_parse_bolt11() -> Result<()> {
        let rest_client: Arc<dyn RestClient> = Arc::new(ReqwestRestClient::new()?);
        let invoice = "lnbc110n1p38q3gtpp5ypz09jrd8p993snjwnm68cph4ftwp22le34xd4r8ftspwshxhmnsdqqxqyjw5qcqpxsp5htlg8ydpywvsa7h3u4hdn77ehs4z4e844em0apjyvmqfkzqhhd2q9qgsqqqyssqszpxzxt9uuqzymr7zxcdccj5g69s8q7zzjs7sgxn9ejhnvdh6gqjcy22mss2yexunagm5r2gqczh8k24cwrqml3njskm548aruhpwssq9nvrvz";

        // Test plain BOLT11 invoice
        assert!(matches!(
            parse_with_rest_client(rest_client.as_ref(), invoice, None).await?,
            InputType::Bolt11 { invoice: _ }
        ));

        // Test BOLT11 with lightning: prefix
        assert!(matches!(
            parse_with_rest_client(rest_client.as_ref(), &format!("lightning:{}", invoice), None).await?,
            InputType::Bolt11 { invoice: _ }
        ));

        // Test BIP21 with LN fallback
        let btc_address = "1andreas3batLhQa2FawWjeyjCqyBzypd";
        assert!(matches!(
            parse_with_rest_client(
                rest_client.as_ref(),
                &format!("bitcoin:{}?lightning={}", btc_address, invoice),
                None
            )
            .await?,
            InputType::Bolt11 { invoice: _ }
        ));

        Ok(())
    }

    #[tokio::test]
    async fn test_parse_web_urls() -> Result<()> {
        let rest_client: Arc<dyn RestClient> = Arc::new(ReqwestRestClient::new()?);

        // Test simple URL
        assert!(matches!(
            parse_with_rest_client(rest_client.as_ref(), "https://breez.technology", None).await?,
            InputType::Url { url: _ }
        ));

        // Test URL with path
        assert!(matches!(
            parse_with_rest_client(rest_client.as_ref(), "https://breez.technology/test-path", None).await?,
            InputType::Url { url: _ }
        ));

        // Test URL with query params
        assert!(matches!(
            parse_with_rest_client(rest_client.as_ref(), "https://breez.technology/test-path?arg=val", None).await?,
            InputType::Url { url: _ }
        ));

        Ok(())
    }

    #[tokio::test]
    async fn test_parse_node_id() -> Result<()> {
        let rest_client: Arc<dyn RestClient> = Arc::new(ReqwestRestClient::new()?);
        let node_id = "03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f";

        // Test plain node ID
        assert!(matches!(
            parse_with_rest_client(rest_client.as_ref(), node_id, None).await?,
            InputType::NodeId { node_id: _ }
        ));

        // Test node URI
        assert!(matches!(
            parse_with_rest_client(rest_client.as_ref(), &format!("{}@example.com", node_id), None).await?,
            InputType::NodeId { node_id: _ }
        ));

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bip353Metadata {
    pub label: Option<String>,
    pub message: Option<String>,
    pub amount: Option<u64>,
}
