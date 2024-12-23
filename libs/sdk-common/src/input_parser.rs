use std::collections::HashMap;
use std::ops::Not;
use std::str::FromStr;

use ::bip21::Uri;
use anyhow::{anyhow, Context, Result};
use bitcoin::bech32;
use bitcoin::bech32::FromBase32;
use hickory_resolver::config::{ResolverConfig, ResolverOpts};
use hickory_resolver::name_server::{GenericConnector, TokioRuntimeProvider};
use hickory_resolver::AsyncResolver;
use hickory_resolver::TokioAsyncResolver;
use lazy_static::lazy_static;
use log::{debug, error};
use percent_encoding::NON_ALPHANUMERIC;
use regex::Regex;
use serde::{Deserialize, Serialize};
use LnUrlRequestData::*;

use crate::prelude::*;

const USER_BITCOIN_PAYMENT_PREFIX: &str = "user._bitcoin-payment";
const BOLT12_PREFIX: &str = "lno=";
const LNURL_PAY_PREFIX: &str = "lnurl=";
const BIP353_PREFIX: &str = "bitcoin:";

lazy_static! {
    static ref DNS_RESOLVER: TokioAsyncResolver = {
        let mut opts = ResolverOpts::default();
        opts.validate = true;

        TokioAsyncResolver::tokio(ResolverConfig::default(), opts)
    };
}

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
///     assert!(matches!( parse(lnurl_pay_url, None).await, Ok(LnUrlPay{data: _}) ));
///     // assert!(matches!( parse("lnurlp://domain.com/lnurl-pay?key=val").await, Ok(LnUrlPay{data: _}) ));
///     // assert!(matches!( parse("lightning@address.com").await, Ok(LnUrlPay{data: _}) ));
///
///     if let Ok(LnUrlPay{data: pd}) = parse(lnurl_pay_url,None).await {
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
///     // assert!(matches!( parse("lnurlw://domain.com/lnurl-withdraw?key=val").await, Ok(LnUrlWithdraw{data: _} ));
///
///     if let Ok(LnUrlWithdraw{data: wd}) = parse(lnurl_withdraw_url,None).await {
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
///     // assert!(matches!( parse("keyauth://domain.com/auth?key=val").await, Ok(LnUrlAuth{data: _}) ));
///
///     if let Ok(LnUrlAuth{data: ad}) = parse(lnurl_auth_url,None).await {
///         assert_eq!(ad.k1, "1a855505699c3e01be41bddd32007bfcc5ff93505dec0cbca64b4b8ff590b822");
///     }
/// }
/// ```
///
/// ## External input parsing
///
/// ```no_run
/// use sdk_common::prelude::{InputType::*, parse, ExternalInputParser};
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
    let input = input.trim();

    // Try to parse the destination as a bip353 address.
    let input_parsed = match bip353_parse(input, &DNS_RESOLVER).await {
        Some(value) => value,
        None => input.to_string(),
    };

    let input = input_parsed.as_str();

    if let Ok(input_type) = parse_core(input).await {
        return Ok(input_type);
    }

    if let Some(external_input_parsers) = external_input_parsers {
        return parse_external(input, external_input_parsers).await;
    }

    Err(anyhow!("Unrecognized input type"))
}

async fn bip353_parse(
    input: &str,
    dns_resolver: &AsyncResolver<GenericConnector<TokioRuntimeProvider>>,
) -> Option<String> {
    if let Some((local_part, domain)) = input.split_once('@') {
        let dns_name = format!("{}.{}.{}", local_part, USER_BITCOIN_PAYMENT_PREFIX, domain);

        // Query for TXT records of a domain
        let txt_data = match dns_resolver.txt_lookup(dns_name).await {
            Ok(records) => records
                .iter()
                .flat_map(|record| record.to_string().into_bytes())
                .collect::<Vec<u8>>(),
            Err(e) => {
                debug!("No BIP353 TXT records found: {}", e);
                return None;
            }
        };

        // Decode TXT data
        match String::from_utf8(txt_data) {
            Ok(decoded) => {
                if !decoded.to_lowercase().starts_with(BIP353_PREFIX) {
                    error!(
                        "Invalid decoded TXT data (doesn't begin with: {})",
                        BIP353_PREFIX
                    );

                    return None;
                }

                if let Some((_, bolt12_address)) =
                    decoded.split_once(&format!("{}?{}", BIP353_PREFIX, BOLT12_PREFIX))
                {
                    return Some(bolt12_address.to_string());
                }

                if let Some((_, lnurl)) =
                    decoded.split_once(&format!("{}?{}", BIP353_PREFIX, LNURL_PAY_PREFIX))
                {
                    return Some(lnurl.to_string());
                }
            }
            Err(e) => {
                error!("Failed to decode TXT data: {}", e);
            }
        }
    }

    None
}

/// Core parse implementation
async fn parse_core(input: &str) -> Result<InputType> {
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
                    return resolve_lnurl(domain, lnurl_endpoint, ln_address).await;
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
        return resolve_lnurl(domain, lnurl_endpoint, ln_address).await;
    }

    Err(anyhow!("Unrecognized input type"))
}

/// Parse input using provided external parsers.
async fn parse_external(
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
        let parsed_value = match request_external_parsing(&parser_url).await {
            Ok(t) => t,
            Err(e) => {
                error!("Request to external input parser {parser:?} failed: {e}");
                continue;
            }
        };

        // Try to parse as LnUrlRequestData
        if let Ok(lnurl_data) = serde_json::from_str::<LnUrlRequestData>(&parsed_value) {
            let domain = url::Url::parse(&parser_url)
                .ok()
                .and_then(|url| url.host_str().map(|s| s.to_string()))
                .unwrap_or_default();
            let input_type = lnurl_data.into();
            let input_type = match input_type {
                // Modify the LnUrlPay payload by adding the domain of the LNURL endpoint
                InputType::LnUrlPay { data } => InputType::LnUrlPay {
                    data: LnUrlPayRequestData { domain, ..data },
                },
                _ => input_type,
            };
            return Ok(input_type);
        }

        // Check other input types
        if let Ok(input_type) = parse_core(&parsed_value).await {
            return Ok(input_type);
        }
    }

    Err(anyhow!("Unrecognized input type"))
}

async fn request_external_parsing(url: &str) -> reqwest::Result<String> {
    let response = reqwest::get(url).await?.error_for_status()?;
    response.text().await
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

async fn resolve_lnurl(
    domain: String,
    mut lnurl_endpoint: String,
    ln_address: Option<String>,
) -> Result<InputType> {
    // For LNURL-auth links, their type is already known if the link contains the login tag
    // No need to query the endpoint for details
    if lnurl_endpoint.contains("tag=login") {
        return Ok(InputType::LnUrlAuth {
            data: validate_request(domain, lnurl_endpoint)?,
        });
    }

    lnurl_endpoint = maybe_replace_host_with_mockito_test_host(lnurl_endpoint)?;
    let lnurl_data: LnUrlRequestData = get_parse_and_log_response(&lnurl_endpoint, false)
        .await
        .map_err(|_| anyhow!("Failed to parse response"))?;
    let temp = lnurl_data.into();
    let temp = match temp {
        // Modify the LnUrlPay payload by adding the domain of the LNURL endpoint
        InputType::LnUrlPay { data } => InputType::LnUrlPay {
            data: LnUrlPayRequestData {
                domain,
                ln_address,
                ..data
            },
        },
        _ => temp,
    };
    Ok(temp)
}

/// Different kinds of inputs supported by [parse], including any relevant details extracted from the input
#[derive(Clone, Debug, Serialize)]
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
    /// - BIP21 on liquid/liquidtestnet
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
            PayRequest { data } => Self::LnUrlPay { data },
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
#[derive(Clone, Debug, Serialize)]
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
pub(crate) mod tests {
    use std::sync::Mutex;

    use anyhow::{anyhow, Result};
    use bitcoin::bech32;
    use bitcoin::bech32::{ToBase32, Variant};
    use bitcoin::secp256k1::{PublicKey, Secp256k1, SecretKey};
    use mockito::{Mock, Server};
    use once_cell::sync::Lazy;

    use crate::input_parser::*;

    /// Mock server used in tests. As the server is shared between tests,
    /// we should not mock the same url twice with two different outputs,
    /// one way to do so is to add a random string that will be a differentiator
    /// in the URL.
    pub(crate) static MOCK_HTTP_SERVER: Lazy<Mutex<Server>> = Lazy::new(|| {
        let opts = mockito::ServerOpts {
            host: "127.0.0.1",
            port: 8080,
            ..Default::default()
        };
        let server = Server::new_with_opts(opts);
        Mutex::new(server)
    });

    #[tokio::test]
    async fn test_generic_invalid_input() -> Result<(), Box<dyn std::error::Error>> {
        assert!(parse("invalid_input", None).await.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_trim_input() -> Result<()> {
        for address in [
            r#"1andreas3batLhQa2FawWjeyjCqyBzypd"#,
            r#"1andreas3batLhQa2FawWjeyjCqyBzypd "#,
            r#"1andreas3batLhQa2FawWjeyjCqyBzypd
            "#,
            r#"
            1andreas3batLhQa2FawWjeyjCqyBzypd
            "#,
            r#" 1andreas3batLhQa2FawWjeyjCqyBzypd
            "#,
        ] {
            assert!(matches!(
                parse(address, None).await?,
                InputType::BitcoinAddress { address: _ }
            ));
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_bitcoin_address() -> Result<()> {
        for address in [
            "1andreas3batLhQa2FawWjeyjCqyBzypd",
            "12c6DSiU4Rq3P4ZxziKxzrL5LmMBrzjrJX",
            "bc1qxhmdufsvnuaaaer4ynz88fspdsxq2h9e9cetdj",
            "3CJ7cNxChpcUykQztFSqKFrMVQDN4zTTsp",
        ] {
            assert!(matches!(
                parse(address, None).await?,
                InputType::BitcoinAddress { address: _ }
            ));
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_bitcoin_address_bip21() -> Result<()> {
        // Addresses from https://github.com/Kixunil/bip21/blob/master/src/lib.rs

        // Valid address with the `bitcoin:` prefix
        assert!(parse("bitcoin:1andreas3batLhQa2FawWjeyjCqyBzypd", None)
            .await
            .is_ok());
        assert!(parse("bitcoin:testinvalidaddress", None).await.is_err());

        let addr = "1andreas3batLhQa2FawWjeyjCqyBzypd";

        // Address with amount
        let addr_1 = format!("bitcoin:{addr}?amount=0.00002000");
        match parse(&addr_1, None).await? {
            InputType::BitcoinAddress {
                address: addr_with_amount_parsed,
            } => {
                assert_eq!(addr_with_amount_parsed.address, addr);
                assert_eq!(addr_with_amount_parsed.network, Network::Bitcoin);
                assert_eq!(addr_with_amount_parsed.amount_sat, Some(2000));
                assert_eq!(addr_with_amount_parsed.label, None);
                assert_eq!(addr_with_amount_parsed.message, None);
            }
            _ => return Err(anyhow!("Invalid type parsed")),
        }

        // Address with amount and label
        let label = "test-label";
        let addr_2 = format!("bitcoin:{addr}?amount=0.00002000&label={label}");
        match parse(&addr_2, None).await? {
            InputType::BitcoinAddress {
                address: addr_with_amount_parsed,
            } => {
                assert_eq!(addr_with_amount_parsed.address, addr);
                assert_eq!(addr_with_amount_parsed.network, Network::Bitcoin);
                assert_eq!(addr_with_amount_parsed.amount_sat, Some(2000));
                assert_eq!(addr_with_amount_parsed.label, Some(label.into()));
                assert_eq!(addr_with_amount_parsed.message, None);
            }
            _ => return Err(anyhow!("Invalid type parsed")),
        }

        // Address with amount, label and message
        let message = "test-message";
        let addr_3 = format!("bitcoin:{addr}?amount=0.00002000&label={label}&message={message}");
        match parse(&addr_3, None).await? {
            InputType::BitcoinAddress {
                address: addr_with_amount_parsed,
            } => {
                assert_eq!(addr_with_amount_parsed.address, addr);
                assert_eq!(addr_with_amount_parsed.network, Network::Bitcoin);
                assert_eq!(addr_with_amount_parsed.amount_sat, Some(2000));
                assert_eq!(addr_with_amount_parsed.label, Some(label.into()));
                assert_eq!(addr_with_amount_parsed.message, Some(message.into()));
            }
            _ => return Err(anyhow!("Invalid type parsed")),
        }

        Ok(())
    }

    /// BIP21 amounts which can lead to rounding errors.
    /// The format is: (sat amount, BIP21 BTC amount)
    pub(crate) fn get_bip21_rounding_test_vectors() -> Vec<(u64, f64)> {
        vec![
            (999, 0.0000_0999),
            (1_000, 0.0000_1000),
            (59_810, 0.0005_9810),
        ]
    }

    #[tokio::test]
    async fn test_bitcoin_address_bip21_rounding() -> Result<()> {
        for (amount_sat, amount_btc) in get_bip21_rounding_test_vectors() {
            let addr = format!("bitcoin:1andreas3batLhQa2FawWjeyjCqyBzypd?amount={amount_btc}");

            match parse(&addr, None).await? {
                InputType::BitcoinAddress {
                    address: addr_with_amount_parsed,
                } => {
                    assert_eq!(addr_with_amount_parsed.amount_sat, Some(amount_sat));
                }
                _ => return Err(anyhow!("Invalid type parsed")),
            }
        }

        Ok(())
    }

    #[tokio::test]
    #[cfg(feature = "liquid")]
    async fn test_liquid_address() -> Result<()> {
        assert!(parse("tlq1qqw5ur50rnvcx33vmljjtnez3hrtl6n7vs44tdj2c9fmnxrrgzgwnhw6jtpn8cljkmlr8tgfw9hemrr5y8u2nu024hhak3tpdk", None)
            .await
            .is_ok());
        assert!(parse("liquidnetwork:tlq1qqw5ur50rnvcx33vmljjtnez3hrtl6n7vs44tdj2c9fmnxrrgzgwnhw6jtpn8cljkmlr8tgfw9hemrr5y8u2nu024hhak3tpdk", None)
            .await
            .is_ok());
        assert!(parse("wrong-net:tlq1qqw5ur50rnvcx33vmljjtnez3hrtl6n7vs44tdj2c9fmnxrrgzgwnhw6jtpn8cljkmlr8tgfw9hemrr5y8u2nu024hhak3tpdk", None).await.is_err());
        assert!(parse("liquidnetwork:testinvalidaddress", None)
            .await
            .is_err());

        let address: elements::Address = "tlq1qqw5ur50rnvcx33vmljjtnez3hrtl6n7vs44tdj2c9fmnxrrgzgwnhw6jtpn8cljkmlr8tgfw9hemrr5y8u2nu024hhak3tpdk".parse()?;
        let amount_btc = 0.00001; // 1000 sats
        let label = "label";
        let message = "this%20is%20a%20message";
        let asset_id = elements::issuance::AssetId::LIQUID_BTC.to_string();
        let output = parse(&format!(
                    "liquidnetwork:{}?amount={amount_btc}&assetid={asset_id}&label={label}&message={message}",
                    address
                ),
                           None)
        .await?;

        if let InputType::LiquidAddress {
            address: liquid_address_data,
        } = output
        {
            assert_eq!(Network::Bitcoin, liquid_address_data.network);
            assert_eq!(address.to_string(), liquid_address_data.address.to_string());
            assert_eq!(
                Some((amount_btc * 100_000_000.0) as u64),
                liquid_address_data.amount_sat
            );
            assert_eq!(Some(label.to_string()), liquid_address_data.label);
            assert_eq!(
                Some(urlencoding::decode(message).unwrap().into_owned()),
                liquid_address_data.message
            );
        } else {
            panic!("Invalid input type received");
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_bolt11() -> Result<()> {
        let bolt11 = "lnbc110n1p38q3gtpp5ypz09jrd8p993snjwnm68cph4ftwp22le34xd4r8ftspwshxhmnsdqqxqyjw5qcqpxsp5htlg8ydpywvsa7h3u4hdn77ehs4z4e844em0apjyvmqfkzqhhd2q9qgsqqqyssqszpxzxt9uuqzymr7zxcdccj5g69s8q7zzjs7sgxn9ejhnvdh6gqjcy22mss2yexunagm5r2gqczh8k24cwrqml3njskm548aruhpwssq9nvrvz";

        // Invoice without prefix
        assert!(matches!(
            parse(bolt11, None).await?,
            InputType::Bolt11 { invoice: _invoice }
        ));

        // Invoice with prefix
        let invoice_with_prefix = format!("lightning:{bolt11}");
        assert!(matches!(
            parse(&invoice_with_prefix, None).await?,
            InputType::Bolt11 { invoice: _invoice }
        ));

        Ok(())
    }

    #[tokio::test]
    async fn test_capitalized_bolt11() -> Result<()> {
        let bolt11 = "LNBC110N1P38Q3GTPP5YPZ09JRD8P993SNJWNM68CPH4FTWP22LE34XD4R8FTSPWSHXHMNSDQQXQYJW5QCQPXSP5HTLG8YDPYWVSA7H3U4HDN77EHS4Z4E844EM0APJYVMQFKZQHHD2Q9QGSQQQYSSQSZPXZXT9UUQZYMR7ZXCDCCJ5G69S8Q7ZZJS7SGXN9EJHNVDH6GQJCY22MSS2YEXUNAGM5R2GQCZH8K24CWRQML3NJSKM548ARUHPWSSQ9NVRVZ";

        // Invoice without prefix
        assert!(matches!(
            parse(bolt11, None).await?,
            InputType::Bolt11 { invoice: _invoice }
        ));

        // Invoice with prefix
        let invoice_with_prefix = format!("LIGHTNING:{bolt11}");
        assert!(matches!(
            parse(&invoice_with_prefix, None).await?,
            InputType::Bolt11 { invoice: _invoice }
        ));

        Ok(())
    }

    #[tokio::test]
    async fn test_bolt11_with_fallback_bitcoin_address() -> Result<()> {
        let addr = "1andreas3batLhQa2FawWjeyjCqyBzypd";
        let bolt11 = "lnbc110n1p38q3gtpp5ypz09jrd8p993snjwnm68cph4ftwp22le34xd4r8ftspwshxhmnsdqqxqyjw5qcqpxsp5htlg8ydpywvsa7h3u4hdn77ehs4z4e844em0apjyvmqfkzqhhd2q9qgsqqqyssqszpxzxt9uuqzymr7zxcdccj5g69s8q7zzjs7sgxn9ejhnvdh6gqjcy22mss2yexunagm5r2gqczh8k24cwrqml3njskm548aruhpwssq9nvrvz";

        // Address and invoice
        // BOLT11 is the first URI arg (preceded by '?')
        let addr_1 = format!("bitcoin:{addr}?lightning={bolt11}");
        assert!(matches!(
            parse(&addr_1, None).await?,
            InputType::Bolt11 { invoice: _invoice }
        ));

        // Address, amount and invoice
        // BOLT11 is not the first URI arg (preceded by '&')
        let addr_2 = format!("bitcoin:{addr}?amount=0.00002000&lightning={bolt11}");
        assert!(matches!(
            parse(&addr_2, None).await?,
            InputType::Bolt11 { invoice: _invoice }
        ));

        Ok(())
    }

    #[tokio::test]
    async fn test_url() -> Result<()> {
        assert!(matches!(
            parse("https://breez.technology", None).await?,
            InputType::Url { url: _url }
        ));
        assert!(matches!(
            parse("https://breez.technology/", None).await?,
            InputType::Url { url: _url }
        ));
        assert!(matches!(
            parse("https://breez.technology/test-path", None).await?,
            InputType::Url { url: _url }
        ));
        assert!(matches!(
            parse(
                "https://breez.technology/test-path?arg1=val1&arg2=val2",
                None
            )
            .await?,
            InputType::Url { url: _url }
        ));
        // `lightning` query param is not an LNURL.
        assert!(matches!(
            parse("https://breez.technology?lightning=nonsense", None).await?,
            InputType::Url { url: _url }
        ));

        Ok(())
    }

    #[tokio::test]
    async fn test_node_id() -> Result<()> {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&[0xab; 32])?;
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);

        match parse(&public_key.to_string(), None).await? {
            InputType::NodeId { node_id } => {
                assert_eq!(node_id, public_key.to_string());
            }
            _ => return Err(anyhow!("Unexpected type")),
        }

        // Other formats and sizes
        assert!(parse(
            "012345678901234567890123456789012345678901234567890123456789mnop",
            None
        )
        .await
        .is_err());
        assert!(parse("0123456789", None).await.is_err());
        assert!(parse("abcdefghij", None).await.is_err());

        // Plain Node ID
        assert!(parse(
            "03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f",
            None
        )
        .await
        .is_ok());
        // Plain Node ID (66 hex chars) with @ separator and any string afterwards
        assert!(parse(
            "03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f@",
            None
        )
        .await
        .is_ok());
        assert!(parse(
            "03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f@sdfsffs",
            None
        )
        .await
        .is_ok());
        assert!(parse(
            "03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f@1.2.3.4:1234",
            None
        )
        .await
        .is_ok());

        // Invalid Node ID (66 chars ending in non-hex-chars) with @ separator and any string afterwards -> invalid
        assert!(parse(
            "03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3zzz@",
            None
        )
        .await
        .is_err());
        assert!(parse(
            "03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3zzz@sdfsffs",
            None
        )
        .await
        .is_err());
        assert!(parse(
            "03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3zzz@1.2.3.4:1234",
            None
        )
        .await
        .is_err());

        Ok(())
    }

    #[test]
    fn test_lnurl_pay_lud_01() -> Result<()> {
        // Covers cases in LUD-01: Base LNURL encoding and decoding
        // https://github.com/lnurl/luds/blob/luds/01.md

        // HTTPS allowed with clearnet domains
        assert_eq!(
            lnurl_decode(&bech32::encode(
                "LNURL",
                "https://domain.com".to_base32(),
                Variant::Bech32
            )?)?,
            ("domain.com".into(), "https://domain.com".into(), None)
        );

        // HTTP not allowed with clearnet domains
        assert!(lnurl_decode(&bech32::encode(
            "LNURL",
            "http://domain.com".to_base32(),
            Variant::Bech32
        )?)
        .is_err());

        // HTTP allowed with onion domains
        assert_eq!(
            lnurl_decode(&bech32::encode(
                "LNURL",
                "http://3fdsf.onion".to_base32(),
                Variant::Bech32
            )?)?,
            ("3fdsf.onion".into(), "http://3fdsf.onion".into(), None)
        );

        // HTTPS not allowed with onion domains
        assert!(lnurl_decode(&bech32::encode(
            "LNURL",
            "https://3fdsf.onion".to_base32(),
            Variant::Bech32
        )?)
        .is_err());

        let decoded_url = "https://service.com/api?q=3fc3645b439ce8e7f2553a69e5267081d96dcd340693afabe04be7b0ccd178df";
        let lnurl_raw = "LNURL1DP68GURN8GHJ7UM9WFMXJCM99E3K7MF0V9CXJ0M385EKVCENXC6R2C35XVUKXEFCV5MKVV34X5EKZD3EV56NYD3HXQURZEPEXEJXXEPNXSCRVWFNV9NXZCN9XQ6XYEFHVGCXXCMYXYMNSERXFQ5FNS";

        assert_eq!(
            lnurl_decode(lnurl_raw)?,
            ("service.com".into(), decoded_url.into(), None)
        );

        // Uppercase and lowercase allowed, but mixed case is invalid
        assert!(lnurl_decode(&lnurl_raw.to_uppercase()).is_ok());
        assert!(lnurl_decode(&lnurl_raw.to_lowercase()).is_ok());
        assert!(lnurl_decode(&format!(
            "{}{}",
            lnurl_raw[..5].to_uppercase(),
            lnurl_raw[5..].to_lowercase()
        ))
        .is_err());

        Ok(())
    }

    fn mock_lnurl_withdraw_endpoint(path: &str, return_lnurl_error: Option<String>) -> Mock {
        let expected_lnurl_withdraw_data = r#"
{
    "tag":"withdrawRequest",
    "callback":"https://localhost/lnurl-withdraw/callback/e464f841c44dbdd86cee4f09f4ccd3ced58d2e24f148730ec192748317b74538",
    "k1":"37b4c919f871c090830cc47b92a544a30097f03430bc39670b8ec0da89f01a81",
    "minWithdrawable":3000,
    "maxWithdrawable":12000,
    "defaultDescription":"sample withdraw"
}
        "#.replace('\n', "");

        let (response_body, status) = match &return_lnurl_error {
            None => (expected_lnurl_withdraw_data, 200),
            Some(err_reason) => (
                ["{\"status\": \"ERROR\", \"reason\": \"", err_reason, "\"}"].join(""),
                400,
            ),
        };

        let mut server = MOCK_HTTP_SERVER.lock().unwrap();
        server
            .mock("GET", path)
            .with_body(response_body)
            .with_status(status)
            .create()
    }

    #[tokio::test]
    async fn test_lnurl_withdraw_lud_03() -> Result<(), Box<dyn std::error::Error>> {
        // Covers cases in LUD-03: withdrawRequest base spec
        // https://github.com/lnurl/luds/blob/luds/03.md

        let path = "/lnurl-withdraw?session=bc893fafeb9819046781b47d68fdcf88fa39a28898784c183b42b7ac13820d81";
        let _m = mock_lnurl_withdraw_endpoint(path, None);

        let lnurl_withdraw_encoded = "lnurl1dp68gurn8ghj7mr0vdskc6r0wd6z7mrww4exctthd96xserjv9mn7um9wdekjmmw843xxwpexdnxzen9vgunsvfexq6rvdecx93rgdmyxcuxverrvcursenpxvukzv3c8qunsdecx33nzwpnvg6ryc3hv93nzvecxgcxgwp3h33lxk";
        assert_eq!(
            lnurl_decode(lnurl_withdraw_encoded)?,
            ("localhost".into(), format!("https://localhost{path}"), None,)
        );

        if let InputType::LnUrlWithdraw { data: wd } = parse(lnurl_withdraw_encoded, None).await? {
            assert_eq!(wd.callback, "https://localhost/lnurl-withdraw/callback/e464f841c44dbdd86cee4f09f4ccd3ced58d2e24f148730ec192748317b74538");
            assert_eq!(
                wd.k1,
                "37b4c919f871c090830cc47b92a544a30097f03430bc39670b8ec0da89f01a81"
            );
            assert_eq!(wd.min_withdrawable, 3000);
            assert_eq!(wd.max_withdrawable, 12000);
            assert_eq!(wd.default_description, "sample withdraw");
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_lnurl_withdraw_in_url() -> Result<(), Box<dyn std::error::Error>> {
        let path = "/lnurl-withdraw?session=bc893fafeb9819046781b47d68fdcf88fa39a28898784c183b42b7ac13820d81";
        let _m = mock_lnurl_withdraw_endpoint(path, None);

        let lnurl_withdraw_encoded = "lnurl1dp68gurn8ghj7mr0vdskc6r0wd6z7mrww4exctthd96xserjv9mn7um9wdekjmmw843xxwpexdnxzen9vgunsvfexq6rvdecx93rgdmyxcuxverrvcursenpxvukzv3c8qunsdecx33nzwpnvg6ryc3hv93nzvecxgcxgwp3h33lxk";
        assert_eq!(
            lnurl_decode(lnurl_withdraw_encoded)?,
            ("localhost".into(), format!("https://localhost{path}"), None,)
        );
        let url = format!("https://bitcoin.org?lightning={lnurl_withdraw_encoded}");

        if let InputType::LnUrlWithdraw { data: wd } = parse(&url, None).await? {
            assert_eq!(wd.callback, "https://localhost/lnurl-withdraw/callback/e464f841c44dbdd86cee4f09f4ccd3ced58d2e24f148730ec192748317b74538");
            assert_eq!(
                wd.k1,
                "37b4c919f871c090830cc47b92a544a30097f03430bc39670b8ec0da89f01a81"
            );
            assert_eq!(wd.min_withdrawable, 3000);
            assert_eq!(wd.max_withdrawable, 12000);
            assert_eq!(wd.default_description, "sample withdraw");
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_lnurl_auth_lud_04() -> Result<()> {
        // Covers cases in LUD-04: `auth` base spec
        // https://github.com/lnurl/luds/blob/luds/04.md

        // No action specified
        let decoded_url = "https://localhost/lnurl-login?tag=login&k1=1a855505699c3e01be41bddd32007bfcc5ff93505dec0cbca64b4b8ff590b822";
        let lnurl_auth_encoded = "lnurl1dp68gurn8ghj7mr0vdskc6r0wd6z7mrww4excttvdankjm3lw3skw0tvdankjm3xdvcn6vtp8q6n2dfsx5mrjwtrxdjnqvtzv56rzcnyv3jrxv3sxqmkyenrvv6kve3exv6nqdtyv43nqcmzvdsnvdrzx33rsenxx5unqc3cxgeqgntfgu";
        assert_eq!(
            lnurl_decode(lnurl_auth_encoded)?,
            ("localhost".into(), decoded_url.into(), None)
        );

        if let InputType::LnUrlAuth { data: ad } = parse(lnurl_auth_encoded, None).await? {
            assert_eq!(
                ad.k1,
                "1a855505699c3e01be41bddd32007bfcc5ff93505dec0cbca64b4b8ff590b822"
            );
            assert_eq!(ad.domain, "localhost".to_string());
            assert_eq!(ad.action, None);
        }

        // Action = register
        let _decoded_url = "https://localhost/lnurl-login?tag=login&k1=1a855505699c3e01be41bddd32007bfcc5ff93505dec0cbca64b4b8ff590b822&action=register";
        let lnurl_auth_encoded = "lnurl1dp68gurn8ghj7mr0vdskc6r0wd6z7mrww4excttvdankjm3lw3skw0tvdankjm3xdvcn6vtp8q6n2dfsx5mrjwtrxdjnqvtzv56rzcnyv3jrxv3sxqmkyenrvv6kve3exv6nqdtyv43nqcmzvdsnvdrzx33rsenxx5unqc3cxgezvctrw35k7m3awfjkw6tnw3jhys2umys";
        if let InputType::LnUrlAuth { data: ad } = parse(lnurl_auth_encoded, None).await? {
            assert_eq!(
                ad.k1,
                "1a855505699c3e01be41bddd32007bfcc5ff93505dec0cbca64b4b8ff590b822"
            );
            assert_eq!(ad.domain, "localhost".to_string());
            assert_eq!(ad.action, Some("register".into()));
        }

        // Action = login
        let _decoded_url = "https://localhost/lnurl-login?tag=login&k1=1a855505699c3e01be41bddd32007bfcc5ff93505dec0cbca64b4b8ff590b822&action=login";
        let lnurl_auth_encoded = "lnurl1dp68gurn8ghj7mr0vdskc6r0wd6z7mrww4excttvdankjm3lw3skw0tvdankjm3xdvcn6vtp8q6n2dfsx5mrjwtrxdjnqvtzv56rzcnyv3jrxv3sxqmkyenrvv6kve3exv6nqdtyv43nqcmzvdsnvdrzx33rsenxx5unqc3cxgezvctrw35k7m3ad3hkw6tw2acjtx";
        if let InputType::LnUrlAuth { data: ad } = parse(lnurl_auth_encoded, None).await? {
            assert_eq!(
                ad.k1,
                "1a855505699c3e01be41bddd32007bfcc5ff93505dec0cbca64b4b8ff590b822"
            );
            assert_eq!(ad.domain, "localhost".to_string());
            assert_eq!(ad.action, Some("login".into()));
        }

        // Action = link
        let _decoded_url = "https://localhost/lnurl-login?tag=login&k1=1a855505699c3e01be41bddd32007bfcc5ff93505dec0cbca64b4b8ff590b822&action=link";
        let lnurl_auth_encoded = "lnurl1dp68gurn8ghj7mr0vdskc6r0wd6z7mrww4excttvdankjm3lw3skw0tvdankjm3xdvcn6vtp8q6n2dfsx5mrjwtrxdjnqvtzv56rzcnyv3jrxv3sxqmkyenrvv6kve3exv6nqdtyv43nqcmzvdsnvdrzx33rsenxx5unqc3cxgezvctrw35k7m3ad35ku6cc8mvs6";
        if let InputType::LnUrlAuth { data: ad } = parse(lnurl_auth_encoded, None).await? {
            assert_eq!(
                ad.k1,
                "1a855505699c3e01be41bddd32007bfcc5ff93505dec0cbca64b4b8ff590b822"
            );
            assert_eq!(ad.domain, "localhost".to_string());
            assert_eq!(ad.action, Some("link".into()));
        }

        // Action = auth
        let _decoded_url = "https://localhost/lnurl-login?tag=login&k1=1a855505699c3e01be41bddd32007bfcc5ff93505dec0cbca64b4b8ff590b822&action=auth";
        let lnurl_auth_encoded = "lnurl1dp68gurn8ghj7mr0vdskc6r0wd6z7mrww4excttvdankjm3lw3skw0tvdankjm3xdvcn6vtp8q6n2dfsx5mrjwtrxdjnqvtzv56rzcnyv3jrxv3sxqmkyenrvv6kve3exv6nqdtyv43nqcmzvdsnvdrzx33rsenxx5unqc3cxgezvctrw35k7m3av96hg6qmg6zgu";
        if let InputType::LnUrlAuth { data: ad } = parse(lnurl_auth_encoded, None).await? {
            assert_eq!(
                ad.k1,
                "1a855505699c3e01be41bddd32007bfcc5ff93505dec0cbca64b4b8ff590b822"
            );
            assert_eq!(ad.domain, "localhost".to_string());
            assert_eq!(ad.action, Some("auth".into()));
        }

        // Action = another, invalid type
        let _decoded_url = "https://localhost/lnurl-login?tag=login&k1=1a855505699c3e01be41bddd32007bfcc5ff93505dec0cbca64b4b8ff590b822&action=invalid";
        let lnurl_auth_encoded = "lnurl1dp68gurn8ghj7mr0vdskc6r0wd6z7mrww4excttvdankjm3lw3skw0tvdankjm3xdvcn6vtp8q6n2dfsx5mrjwtrxdjnqvtzv56rzcnyv3jrxv3sxqmkyenrvv6kve3exv6nqdtyv43nqcmzvdsnvdrzx33rsenxx5unqc3cxgezvctrw35k7m3ad9h8vctvd9jq2s4vfw";
        assert!(parse(lnurl_auth_encoded, None).await.is_err());

        Ok(())
    }

    fn mock_lnurl_pay_endpoint(path: &str, return_lnurl_error: Option<String>) -> Mock {
        let expected_lnurl_pay_data = r#"
{
    "callback":"https://localhost/lnurl-pay/callback/db945b624265fc7f5a8d77f269f7589d789a771bdfd20e91a3cf6f50382a98d7",
    "tag":"payRequest",
    "maxSendable":16000,
    "minSendable":4000,
    "metadata":"[
        [\"text/plain\",\"WRhtV\"],
        [\"text/long-desc\",\"MBTrTiLCFS\"],
        [\"image/png;base64\",\"iVBORw0KGgoAAAANSUhEUgAAASwAAAEsCAYAAAB5fY51AAATOElEQVR4nO3dz4slVxXA8fIHiEhCjBrcCHEEXbiLkiwd/LFxChmQWUVlpqfrdmcxweAk9r09cUrQlWQpbgXBv8CdwrhRJqn7umfEaEgQGVGzUEwkIu6ei6TGmvH16/ej6p5z7v1+4Ozfq3vqO5dMZ7qqAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAgHe4WbjuutBKfw4AWMrNwnUXw9zFMCdaANS6J1ZEC4BWC2NFtABoszRWRAuAFivFimgBkLZWrIgWACkbxYpoAUhtq1gRLQCpjBIrogVU1ZM32webma9dDM+7LrR3J4bnm5mvn7zZPij9GS0bNVZEaxTsvDEu+iea6F9w0d9a5QVpunDcRP/C7uzgM9Kf3ZJJYkW0NsLOG7PzynMPNDFcaTr/2+1eFH/kon/q67evfkD6O2k2aayI1krYeYPO3mjf67rwjIv+zZFfmL+5zu+18/bd0t9RmySxIlonYueNuvTS4cfe/tNhuhem6cKvXGw/LP1dtUgaK6L1f9h5o/aODj/rov9Hihemif4vzS3/SenvLE0kVkTrLnbeKBfDYxNch0+bv7p47RPS312KaKyIFjtv1U53cMZ1/u8yL42/s3/76iPSzyA1FbEqOFrsvFGXX24fdtH/UfKFaaKP0s8hJVWxKjBa7LxhTfQ3xF+WGOYu+h9LP4sUVMaqsGix80a56J+WP7T/ze7s4PPSz2RKqmNVSLTYeaMuHfmPuBjekj6w4TTRvyb9XKZiIlaZR4udN6yJ/gfSh7Vo9mb+kvSzGZupWGUcLXbeqJ1XnnvAdf7f0gd1wrwq/XzGZDJWGUaLnTesmYWLCg5p2Twm/YzGYDpWmUWLnTfMxfAzBQd04ux24XvSz2hbWcQqo2ix80ZdmF94j4v+P9IHtHz8TenntI2sYtWP4Wix84Zd7g4flz+c00f6OW0qy1j1YzRa7LxhTRd2pA9mlWluffvT0s9qXVnHqh+D0WLnDbPyUjWd/4r0s1qHlec6yhiLlpWzsbbzSTTRf1f6YFaZvdmhk35Wq7LyQow6hqLFzhvWRP8d6YNZZZoYvPSzWkWRserHSLTYecPcLDwrfTArzrekn9Vpio5VPwaixc4b1sTDfQUHs8rsSj+rZYjVYJRHi503bLfzX1ZwMKdO0x18UfpZnYRYLRjF0WLnDds/PnhU+mBWmYsvPftR6We1CLFaMkqjxc4b5zr/uvThLF98/wfpZ7QIsVrl7HRGi503zHXhJ+IHtGSaGH4k/YzuR6zWefn0RYudN8xFf176gJbN3lH4gvQzGiJWG4yyaLHzxrku/FP6kE5Y9D9JP5shYrXVWbbS5zfEzhvmutCKH9TC8U9LP5sesRrlZWylz7HHzht28bh9SOCXSJ623Gr+pCFWo55rK32eVcXOm7c3O3TiB3bP+PPSz6SqiNVEL2Yrfa5Vxc6b57rwC/lDC/Mm+p9KP4uqIlaTjpJosfOGvfNbcO+IHlwXji/8+pn3Sz8LYpVgFESLnTdupzs408Twhszh+Tv7t68+Iv0MiFXCURAtdt64y93h4030/0p8eH/e6Q7OSH93YiUwCqJV8s5nwUX/RLq/RfF3dm9f+7j4dyZWcqMgWiXufFb2jw8ebWL43ZQH13T+50/95uCD0t+VWCkYBdEqaeezdOW1K+9rYvAuhrfGXU7/ejMLF6t59S7p70isFI2CaJWw89m7/HL7sJv5b7oYXt3u4PzNvVn4mvT36RErhaMgWlWV784Xpznyn2ti+KGL/verHFjThRdd57+/0137lPRnHyJWikdJtHq57HzxvvGi/1DTHX7VzcJ114X27sx82O3Cl7T+fAmxMjDKotWzuvMwilgZGqXRApIgVgaHaKFExMrwEC2UhFhlMEQLJSBWGQ3RQs6IVYZDtJAjYpXxEC3khFgVMEQLOSBWBQ3RgmXEqsAhWrDIdaGt63rOlDdEC6b0v2dO+sVhhILFTQtWDH8ppvSLwwgGi2hBu/t/g6/0i8MIB4toQatFv25c+sVhFASLaEGbRbEiWOUOf3sItU6KFcEqd/iRB6i0LFYEq9zh57SgzmmxIljlDj9cClVWiRXBKnf4iXiosWqsCFa5w//GAxXWiRXBKnfW2RGihUmsGyuCVe6suydEC6PaJFYEq9zZZFeIFkaxaawIVrmz6b4QLWxlm1gRrHJnm50hWtjItrEiWOXOtntDtLCWMWJFsMqdMXaHaGElY8WKYJU7Y+0P0cJSY8aKYJU7Y+4Q0cJCY8eKYJU7Y+8R0cI9pogVwSp3ptglooWqqqaLFcEqd6baJ6JVuCljRbDKnSl3imgVaupYEaxyZ+q9IlqFSRGrhME6K/Uc67q29Mtif1nX9dksgkW0ypEqVgmDdUPiOZ4/f/6huq7fUBCilULVf+5sgkW08pcyVgmDNa8Fblm1/tvVPaEafO58gkW08pU6VomDlfSWpfx2tTBUveyCRbTyIxGrxMGaL3tJx1brvF0tDdXgs+cXLKKVD6lYCQQryS1L4e1qpVD1sg0W0bJPMlYCwZqv8+JuqtZzu1orVIPPn2+wiJZd0rESCtaktywlt6uNQtXLPlhEyx4NsRIK1nybl/k0teztaqtQDb5D/sEiWnZoiZVgsCa5ZQnerkYJVa+YYBEt/TTFSjBY8zFf8F6d/nY1aqgG36OcYBEtvbTFSjhYo96yEt+uJglVr7hgES19NMZKOFjzMV/6Os3tatJQDb5LecEiWnpojZWCYI1yy0pwu0oSql6xwSJa8jTHSkGw5mOEoJ7udpU0VIPvU26wiJYc7bFSEqytblkT3a5EQtUrPlhEKz0LsVISrPk2cainuV29Udf19fPnzz804kqs850IFtFKx0qsFAVro1tWgv92JRIugkW0krEUK0XBmteb/T93qX7uKmm4CBbRSsJarJQFa61bltBPtScJF8EiWpOzGCtlwZrX6/0TLJL/z+Ck4SJYRGtSVmOlMFgr3bKU/IsMk4WLYBGtyViOlcJgzevV/kVOLf/e1SThIlhEaxLWY6U0WEtvWYpuV5OFi2ARrdHlECulwZrXy39Bg7bb1ejhIlhEa1S5xEpxsBbespTfrkYLF8EiWqPJKVaKgzWvF/++Pgu3q63DRbCI1ihyi5XyYN1zyzJ4u9o4XASLaG0tx1gpD9a8vvfXt1u9Xa0dLoJFtLaSa6wMBOtGVWVzu1o5XASLaG0s51gZCNa8ruuzdV63q1PDRbCI1kZyj5WRYN2o87xdnRgugkW01lZCrIwEiyFYRGuZUmJFsMod6b0jWiMpKVYEq9yR3juiNYLSYkWwyh3pvSNaWyoxVgSr3JHeO6K1hVJjRbDKHem9I1pbIFhMaSO9dwRrS6VGS/rFYQgWsdpQidGSfnEYgkWstlBatKRfHIZgEastlRQt6ReHIVjEagSlREv6xWEIFrEaSQnRSvSCtOfOnXtT+iVNMe98z19Kf47ig1VarHq5RyvFy1FVd/9NqxLC1dZv/5M40p+j3GCVGqteztFKFaxezuE6d+7cm4N/00r1LUt674jVxHKNVupg9TINV9t/v1r5LUt674hVAjlGSypYvVzCNbxd9WrFtyzpvSNWieQWLelg9TIIV3v/d6oV37Kk945YJZRTtLQEq2cxXItuV71a6S1Leu+IVWK5REtbsHrGwtWe9D1qpbcs6b0jVgJyiJbWYPW0h2vZ7apXK7xlSe8dsRJiPVrag9VTHK72tM9eK7xlSe8dsRJkOVpWgtXTFK5Vble9WtktS3rviJUwq9GyFqyeknC1q37eWtktS3rviJUCFqNlNVg9qXCtc7vq1YpuWdJ7R6yUsBYt68HqCYSrXfcz1opuWdJ7R6wUsRStXILVSxGuTW5XvVrJLUt674iVMlailVuwehOHq930c9VKblnSe0esFLIQrVyDVVV343BjzO+yze1q8LnEb1nSe0eslNIerRyDNUWoBtOO9PkIFrHSSXO0cgrWxKEa5XY1+KyityzpvSNWymmNVg7BmjpUg2lH/swEi1jppTFaloOVMFSj3q4Gn1/sliW9d8TKCG3RshislKEaTDvR9yBYxEo3TdGyFCyhUE1yuxp8J5FblvTeEStjtETLQrCkQjWYdoQjX/bdygwWsbJFQ7Q0B0tBqCa9XQ2+Z/JblvTeESujpKOlMVgaQjWYdoJjX/R9ywkWsbJNMlqagqUsVEluV4PvnvSWRaywFaloaQiWtlANpk1w9MNnkHewiFVeJKIlGSzFoUp6uxo8j2S3LGKFUaSOlkSwNIdqMG3qs68T3rKIFUaTMlopg2UkVCK3q8EzSnLLIlYYVapoJYqAiVANppU69zrRLYtYYXQpoqUgDozAECtMYupoSb84TIbBIlZlmzJa0i8Ok1mwiBWqarpoSb84TEbBIlYYmiJa0i8Ok0mwiBUWGTta0i8Ok0GwiBWWGTNa0i8OYzxYxAqrGCta0i8OYzhYxArrGCNa0i8OYzRYxAqb2DZa0i8OYzBYxArb2CZa0i8OYyxYxApj2DRa0i8OYyhYxApj2iRa0i8OYyRYxApTWDda0i8OYyBYxApTWida0i8OozxYxAoprBot6ReHURwsYoWUVomW9IvDKA0WsYKE06Il/eIwCoNFrCBpWbSkXxxGWbCIFTQ4KVrSLw6jKFjECposipb0i8MoCRaxgkb3R0v6xWEUBItYQbNhtKRfHEY4WMQKFvTRkn5xGMFgEStY4rrQSr84jFCwiBUsSvUbphlFQ6xgGdEqaIgVckC0ChhihZwQrYyHWCFHRCvDIVbIGdHKaIgVSkC0MhhihZIQLcNDrFAiomVwiBVKRrQMDbHCmJ682T7YzHztYnjedaG9OzE838x8/eTN9kHpz7gI0TIwSmNldeeL5aJ/oon+BRf9rVUWr+nCcRP9C7uzg89If/YhoqV4lMUql50vxs4rzz3QxHCl6fxvt1tEf+Sif+rrt69+QPo7VRXRUjlKYpXrzmft7I32va4Lz7jo3xx5Mf/mOr/Xztt3S39HoqVoFMSqhJ3P0qWXDj/29p8O0y1o04Vfudh+WPq7Ei0FoyBWJe18VvaODj/rov9HikVtov9Lc8t/Uvo7Ey3BURCrEnc+Cy6Gxya4Dp82f3Xx2ifEvzvRSj8KYlXyzpu20x2ccZ3/u8zy+jv7t68+Iv0MiFbCURArdt6oyy+3D7vo/yi5wE30Ufo5VBXRSjIKYsXOG9ZEf0N8iWOYu+h/LP0sqopoTToKYlVV7LxZLvqn5Q/tf7M7O/i89DOpKqI1ySiJFTtv1KUj/xEXw1vSBzacJvrXpJ9Lj2iNOEpixc4b1kT/A+nDWjR7M39J+tn0iNYIoyRWVcXOm7XzynMPuM7/W/qgTphXpZ/PENHaYhTFip03rJmFiwoOadk8Jv2MhojWBqMoVlXFzpvmYviZggM6cXa78D3pZ3Q/orXGKItVVbHzZl2YX3iPi/4/0ge0fPxN6ee0CNFaYRTGip037HJ3+Lj84Zw+0s/pJERrySiMVVWx86Y1XdiRPphVprn17U9LP6uTEK0FozRWVcXOm+Zm4br0wax0eJ3/ivSzWoZoDUZxrKqKnTetif670gezyuzNDp30szoN0QrqY1VV7LxpTfTfkT6YVaaJwUs/q1UUHS0Dsaoqdt40NwvPSh/MivMt6We1qiKjZSRWVcXOm9bEw30FB7PK7Eo/q3UUFS1Dsaoqdt603c5/WcHBnDpNd/BF6We1riKiZSxWVcXOm7Z/fPCo9MGsMhdfevaj0s9qE1lHy2CsqoqdN891/nXpw1n+Yvg/SD+jbWQZLaOx6rHzhrku/ET8gJZME8OPpJ/RtrKKlvFYVRU7b5qL/rz0AS2bvaPwBelnNIYsopVBrKqKnTfPdeGf0od0wgvyJ+lnMybT0cokVj123jC9L5J/WvrZjE3vsy4nVlWl+Rzy2/nRXTxuHxL4JZKnvSTZ/kmj92UpI1ZVxc6btzc7dOIHds/489LPZEomopVprHrsvHGuC7+QP7Qwb6L/qfSzSEF1tDKPVY+dN+yd34J7R/TgunB84dfPvF/6WaSiMlqFxKqq2HnzdrqDM00Mb8gcnr+zf/vqI9LPIDVV0SooVj123rjL3eHjTfT/Snx4f97pDs5If3cpKqJVYKx67LxxLvon0v0tir+ze/vax6W/szTRaBUcqx47b9z+8cGjTQy/m/Lgms7//KnfHHxQ+rtqIRItYnUXO2/cldeuvK+JwbsY3hr3JfGvN7NwsZpX75L+jtokjRax+j/sfAYuv9w+7Gb+my6GV7c7OH9zbxa+Jv19tEsSLWK1FDufiebIf66J4Ycu+t+vcmBNF150nf/+TnftU9Kf3ZJJo0Ws1sLOZ+IbL/oPNd3hV90sXHddaO/OzIfdLnyJny/ZziTRIlZbYeeBJUaNFrECMLVRokWsAKSyVbSIFYDUNooWsQIgZa1oESsA0laKFrECoMXSaBErANosjBaxAqDVPdEiVgC063/aWvpzAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQI//AplAdntdLBX1AAAAAElFTkSuQmCC\"]
    ]",
    "commentAllowed":0,
    "payerData":{
        "name":{"mandatory":false},
        "pubkey":{"mandatory":false},
        "identifier":{"mandatory":false},
        "email":{"mandatory":false},
        "auth":{"mandatory":false,"k1":"18ec6d5b96db6f219baed2f188aee7359fcf5bea11bb7d5b47157519474c2222"}
    }
}
        "#.replace('\n', "");

        let response_body = match return_lnurl_error {
            None => expected_lnurl_pay_data,
            Some(err_reason) => {
                ["{\"status\": \"ERROR\", \"reason\": \"", &err_reason, "\"}"].join("")
            }
        };

        let mut server = MOCK_HTTP_SERVER.lock().unwrap();
        server.mock("GET", path).with_body(response_body).create()
    }

    fn mock_lnurl_ln_address_endpoint(
        ln_address: &str,
        return_lnurl_error: Option<String>,
    ) -> Result<Mock> {
        let (_domain, lnurl_pay_url, _ln_address) = ln_address_decode(ln_address)?;
        let url = reqwest::Url::parse(&lnurl_pay_url)?;
        let path = url.path();

        let expected_lnurl_pay_data = r#"
{
    "callback":"https://localhost/lnurl-pay/callback/db945b624265fc7f5a8d77f269f7589d789a771bdfd20e91a3cf6f50382a98d7",
    "tag":"payRequest",
    "maxSendable":16000,
    "minSendable":4000,
    "metadata":"[
        [\"text/plain\",\"WRhtV\"],
        [\"text/long-desc\",\"MBTrTiLCFS\"],
        [\"image/png;base64\",\"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\"]
    ]",
    "commentAllowed":0,
    "payerData":{
        "name":{"mandatory":false},
        "pubkey":{"mandatory":false},
        "identifier":{"mandatory":false},
        "email":{"mandatory":false},
        "auth":{"mandatory":false,"k1":"18ec6d5b96db6f219baed2f188aee7359fcf5bea11bb7d5b47157519474c2222"}
    }
}
        "#.replace('\n', "");

        let response_body = match return_lnurl_error {
            None => expected_lnurl_pay_data,
            Some(err_reason) => {
                ["{\"status\": \"ERROR\", \"reason\": \"", &err_reason, "\"}"].join("")
            }
        };

        let mut server = MOCK_HTTP_SERVER.lock().unwrap();
        Ok(server.mock("GET", path).with_body(response_body).create())
    }

    #[tokio::test]
    async fn test_lnurl_pay_lud_06() -> Result<(), Box<dyn std::error::Error>> {
        // Covers cases in LUD-06: payRequest base spec
        // https://github.com/lnurl/luds/blob/luds/06.md

        let path =
            "/lnurl-pay?session=db945b624265fc7f5a8d77f269f7589d789a771bdfd20e91a3cf6f50382a98d7";
        let _m = mock_lnurl_pay_endpoint(path, None);

        let lnurl_pay_encoded = "lnurl1dp68gurn8ghj7mr0vdskc6r0wd6z7mrww4excttsv9un7um9wdekjmmw84jxywf5x43rvv35xgmr2enrxanr2cfcvsmnwe3jxcukvde48qukgdec89snwde3vfjxvepjxpjnjvtpxd3kvdnxx5crxwpjvyunsephsz36jf";
        assert_eq!(
            lnurl_decode(lnurl_pay_encoded)?,
            ("localhost".into(), format!("https://localhost{path}"), None)
        );

        if let InputType::LnUrlPay { data: pd } = parse(lnurl_pay_encoded, None).await? {
            assert_eq!(pd.callback, "https://localhost/lnurl-pay/callback/db945b624265fc7f5a8d77f269f7589d789a771bdfd20e91a3cf6f50382a98d7");
            assert_eq!(pd.max_sendable, 16000);
            assert_eq!(pd.min_sendable, 4000);
            assert_eq!(pd.comment_allowed, 0);
            assert_eq!(pd.domain, "localhost");

            assert_eq!(pd.metadata_vec()?.len(), 3);
            assert_eq!(
                pd.metadata_vec()?.first().ok_or("Key not found")?.key,
                "text/plain"
            );
            assert_eq!(
                pd.metadata_vec()?.first().ok_or("Key not found")?.value,
                "WRhtV"
            );
            assert_eq!(
                pd.metadata_vec()?.get(1).ok_or("Key not found")?.key,
                "text/long-desc"
            );
            assert_eq!(
                pd.metadata_vec()?.get(1).ok_or("Key not found")?.value,
                "MBTrTiLCFS"
            );
            assert_eq!(
                pd.metadata_vec()?.get(2).ok_or("Key not found")?.key,
                "image/png;base64"
            );
        }

        for lnurl_pay in [
            lnurl_pay_encoded,
            lnurl_pay_encoded.to_uppercase().as_str(),
            format!("lightning:{}", lnurl_pay_encoded).as_str(),
            format!("lightning:{}", lnurl_pay_encoded.to_uppercase()).as_str(),
            format!("LIGHTNING:{}", lnurl_pay_encoded).as_str(),
            format!("LIGHTNING:{}", lnurl_pay_encoded.to_uppercase()).as_str(),
        ] {
            assert!(matches!(
                parse(lnurl_pay, None).await?,
                InputType::LnUrlPay { data: _ }
            ));
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_lnurl_pay_lud_16_ln_address() -> Result<(), Box<dyn std::error::Error>> {
        // Covers cases in LUD-16: Paying to static internet identifiers (LN Address)
        // https://github.com/lnurl/luds/blob/luds/16.md

        let ln_address = "user@domain.net";
        let _m = mock_lnurl_ln_address_endpoint(ln_address, None)?;

        if let InputType::LnUrlPay { data: pd } = parse(ln_address, None).await? {
            assert_eq!(pd.callback, "https://localhost/lnurl-pay/callback/db945b624265fc7f5a8d77f269f7589d789a771bdfd20e91a3cf6f50382a98d7");
            assert_eq!(pd.max_sendable, 16000);
            assert_eq!(pd.min_sendable, 4000);
            assert_eq!(pd.comment_allowed, 0);
            assert_eq!(pd.domain, "domain.net");
            assert_eq!(pd.ln_address, Some(ln_address.to_string()));

            assert_eq!(pd.metadata_vec()?.len(), 3);
            assert_eq!(
                pd.metadata_vec()?.first().ok_or("Key not found")?.key,
                "text/plain"
            );
            assert_eq!(
                pd.metadata_vec()?.first().ok_or("Key not found")?.value,
                "WRhtV"
            );
            assert_eq!(
                pd.metadata_vec()?.get(1).ok_or("Key not found")?.key,
                "text/long-desc"
            );
            assert_eq!(
                pd.metadata_vec()?.get(1).ok_or("Key not found")?.value,
                "MBTrTiLCFS"
            );
            assert_eq!(
                pd.metadata_vec()?.get(2).ok_or("Key not found")?.key,
                "image/png;base64"
            );
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_lnurl_pay_lud_16_ln_address_with_prefix() -> Result<(), Box<dyn std::error::Error>>
    {
        // Covers cases in LUD-16, with BIP-353 prefix.

        let ln_address = "₿user@domain.net";
        let server_ln_address = "user@domain.net";
        let _m = mock_lnurl_ln_address_endpoint(server_ln_address, None)?;

        if let InputType::LnUrlPay { data: pd } = parse(ln_address, None).await? {
            assert_eq!(pd.callback, "https://localhost/lnurl-pay/callback/db945b624265fc7f5a8d77f269f7589d789a771bdfd20e91a3cf6f50382a98d7");
            assert_eq!(pd.max_sendable, 16000);
            assert_eq!(pd.min_sendable, 4000);
            assert_eq!(pd.comment_allowed, 0);
            assert_eq!(pd.domain, "domain.net");
            assert_eq!(pd.ln_address, Some(server_ln_address.to_string()));
        } else {
            panic!("input was not ln address")
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_lnurl_pay_lud_16_ln_address_error() -> Result<()> {
        // Covers cases in LUD-16: Paying to static internet identifiers (LN Address)
        // https://github.com/lnurl/luds/blob/luds/16.md

        let ln_address = "error@domain.com";
        let expected_err = "Error msg from LNURL endpoint found via LN Address";
        let _m = mock_lnurl_ln_address_endpoint(ln_address, Some(expected_err.to_string()))?;

        if let InputType::LnUrlError { data: msg } = parse(ln_address, None).await? {
            assert_eq!(msg.reason, expected_err);
            return Ok(());
        }

        Err(anyhow!("Unrecognized input type"))
    }

    #[test]
    fn test_ln_address_lud_16_decode() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            lnurl_decode("user@domain.onion")?,
            (
                "domain.onion".into(),
                "http://domain.onion/.well-known/lnurlp/user".into(),
                Some("user@domain.onion".into()),
            )
        );
        assert_eq!(
            lnurl_decode("user@domain.com")?,
            (
                "domain.com".into(),
                "https://domain.com/.well-known/lnurlp/user".into(),
                Some("user@domain.com".into()),
            )
        );
        assert_eq!(
            lnurl_decode("user@domain.net")?,
            (
                "domain.net".into(),
                "https://domain.net/.well-known/lnurlp/user".into(),
                Some("user@domain.net".into()),
            )
        );
        assert_eq!(
            lnurl_decode("User@domain.com")?,
            (
                "domain.com".into(),
                "https://domain.com/.well-known/lnurlp/user".into(),
                Some("user@domain.com".into()),
            )
        );
        assert_eq!(
            lnurl_decode("ODELL@DOMAIN.COM")?,
            (
                "domain.com".into(),
                "https://domain.com/.well-known/lnurlp/odell".into(),
                Some("odell@domain.com".into()),
            )
        );
        assert!(ln_address_decode("invalid_ln_address").is_err());

        // Valid chars are a-z0-9-_.
        assert!(lnurl_decode("user.testy_test1@domain.com").is_ok());
        assert!(lnurl_decode("user+1@domain.com").is_err());

        Ok(())
    }

    #[test]
    fn test_lnurl_lud_17_prefixes() -> Result<(), Box<dyn std::error::Error>> {
        // Covers cases in LUD-17: Protocol schemes and raw (non bech32-encoded) URLs
        // https://github.com/lnurl/luds/blob/luds/17.md

        // Variant-specific prefix replaces https for clearnet and http for onion

        // For onion addresses, the prefix maps to an equivalent HTTP URL
        assert_eq!(
            lnurl_decode("lnurlp://asfddf2dsf3f.onion")?,
            (
                "asfddf2dsf3f.onion".into(),
                "http://asfddf2dsf3f.onion".into(),
                None,
            )
        );
        assert_eq!(
            lnurl_decode("lnurlp://asfddf2dsf3flnurlp.onion")?,
            (
                "asfddf2dsf3flnurlp.onion".into(),
                "http://asfddf2dsf3flnurlp.onion".into(),
                None,
            )
        );
        assert_eq!(
            lnurl_decode("lnurlw://asfddf2dsf3f.onion")?,
            (
                "asfddf2dsf3f.onion".into(),
                "http://asfddf2dsf3f.onion".into(),
                None,
            )
        );
        assert_eq!(
            lnurl_decode("keyauth://asfddf2dsf3f.onion")?,
            (
                "asfddf2dsf3f.onion".into(),
                "http://asfddf2dsf3f.onion".into(),
                None,
            )
        );

        // For non-onion addresses, the prefix maps to an equivalent HTTPS URL
        assert_eq!(
            lnurl_decode("lnurlp://domain.com")?,
            ("domain.com".into(), "https://domain.com".into(), None)
        );
        assert_eq!(
            lnurl_decode("lnurlp://lnurlp.com")?,
            ("lnurlp.com".into(), "https://lnurlp.com".into(), None)
        );
        assert_eq!(
            lnurl_decode("lnurlw://domain.com")?,
            ("domain.com".into(), "https://domain.com".into(), None)
        );
        assert_eq!(
            lnurl_decode("lnurlw://lnurlw.com")?,
            ("lnurlw.com".into(), "https://lnurlw.com".into(), None)
        );
        assert_eq!(
            lnurl_decode("keyauth://domain.com")?,
            ("domain.com".into(), "https://domain.com".into(), None)
        );
        assert_eq!(
            lnurl_decode("keyauth://keyauth.com")?,
            ("keyauth.com".into(), "https://keyauth.com".into(), None)
        );

        // Same as above, but prefix: approach instead of prefix://
        assert_eq!(
            lnurl_decode("lnurlp:asfddf2dsf3f.onion")?,
            (
                "asfddf2dsf3f.onion".into(),
                "http://asfddf2dsf3f.onion".into(),
                None
            )
        );
        assert_eq!(
            lnurl_decode("lnurlw:asfddf2dsf3f.onion")?,
            (
                "asfddf2dsf3f.onion".into(),
                "http://asfddf2dsf3f.onion".into(),
                None
            )
        );
        assert_eq!(
            lnurl_decode("keyauth:asfddf2dsf3f.onion")?,
            (
                "asfddf2dsf3f.onion".into(),
                "http://asfddf2dsf3f.onion".into(),
                None
            )
        );

        assert_eq!(
            lnurl_decode("lnurlp:domain.com")?,
            ("domain.com".into(), "https://domain.com".into(), None)
        );
        assert_eq!(
            lnurl_decode("lnurlp:domain.com/lnurlp:lol")?,
            (
                "domain.com".into(),
                "https://domain.com/lnurlp:lol".into(),
                None
            )
        );
        assert_eq!(
            lnurl_decode("lnurlw:domain.com")?,
            ("domain.com".into(), "https://domain.com".into(), None)
        );
        assert_eq!(
            lnurl_decode("keyauth:domain.com")?,
            ("domain.com".into(), "https://domain.com".into(), None)
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_lnurl_pay_lud_17() -> Result<(), Box<dyn std::error::Error>> {
        let pay_path =
            "/lnurl-pay?session=db945b624265fc7f5a8d77f269f7589d789a771bdfd20e91a3cf6f50382a98d7";
        let _m = mock_lnurl_pay_endpoint(pay_path, None);

        let lnurl_pay_url = format!("lnurlp://localhost{pay_path}");
        if let InputType::LnUrlPay { data: pd } = parse(&lnurl_pay_url, None).await? {
            assert_eq!(pd.callback, "https://localhost/lnurl-pay/callback/db945b624265fc7f5a8d77f269f7589d789a771bdfd20e91a3cf6f50382a98d7");
            assert_eq!(pd.max_sendable, 16000);
            assert_eq!(pd.min_sendable, 4000);
            assert_eq!(pd.comment_allowed, 0);
            assert_eq!(pd.domain, "localhost");

            assert_eq!(pd.metadata_vec()?.len(), 3);
            assert_eq!(
                pd.metadata_vec()?.first().ok_or("Key not found")?.key,
                "text/plain"
            );
            assert_eq!(
                pd.metadata_vec()?.first().ok_or("Key not found")?.value,
                "WRhtV"
            );
            assert_eq!(
                pd.metadata_vec()?.get(1).ok_or("Key not found")?.key,
                "text/long-desc"
            );
            assert_eq!(
                pd.metadata_vec()?.get(1).ok_or("Key not found")?.value,
                "MBTrTiLCFS"
            );
            assert_eq!(
                pd.metadata_vec()?.get(2).ok_or("Key not found")?.key,
                "image/png;base64"
            );
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_lnurl_withdraw_lud_17() -> Result<(), Box<dyn std::error::Error>> {
        let withdraw_path = "/lnurl-withdraw?session=e464f841c44dbdd86cee4f09f4ccd3ced58d2e24f148730ec192748317b74538";
        let _m = mock_lnurl_withdraw_endpoint(withdraw_path, None);

        if let InputType::LnUrlWithdraw { data: wd } =
            parse(&format!("lnurlw://localhost{withdraw_path}"), None).await?
        {
            assert_eq!(wd.callback, "https://localhost/lnurl-withdraw/callback/e464f841c44dbdd86cee4f09f4ccd3ced58d2e24f148730ec192748317b74538");
            assert_eq!(
                wd.k1,
                "37b4c919f871c090830cc47b92a544a30097f03430bc39670b8ec0da89f01a81"
            );
            assert_eq!(wd.min_withdrawable, 3000);
            assert_eq!(wd.max_withdrawable, 12000);
            assert_eq!(wd.default_description, "sample withdraw");
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_lnurl_auth_lud_17() -> Result<()> {
        let auth_path = "/lnurl-login?tag=login&k1=1a855505699c3e01be41bddd32007bfcc5ff93505dec0cbca64b4b8ff590b822";

        if let InputType::LnUrlAuth { data: ad } =
            parse(&format!("keyauth://localhost{auth_path}"), None).await?
        {
            assert_eq!(
                ad.k1,
                "1a855505699c3e01be41bddd32007bfcc5ff93505dec0cbca64b4b8ff590b822"
            );
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_lnurl_pay_lud_17_error() -> Result<()> {
        let pay_path = "/lnurl-pay?session=paylud17error";
        let expected_error_msg = "test pay error";
        let _m = mock_lnurl_pay_endpoint(pay_path, Some(expected_error_msg.to_string()));

        if let InputType::LnUrlError { data: msg } =
            parse(&format!("lnurlp://localhost{pay_path}"), None).await?
        {
            assert_eq!(msg.reason, expected_error_msg);
            return Ok(());
        }

        Err(anyhow!("Unrecognized input type"))
    }

    #[tokio::test]
    async fn test_lnurl_withdraw_lud_17_error() -> Result<()> {
        let withdraw_path = "/lnurl-withdraw?session=withdrawlud17error";
        let expected_error_msg = "test withdraw error";
        let _m = mock_lnurl_withdraw_endpoint(withdraw_path, Some(expected_error_msg.to_string()));

        if let InputType::LnUrlError { data: msg } =
            parse(&format!("lnurlw://localhost{withdraw_path}"), None).await?
        {
            assert_eq!(msg.reason, expected_error_msg);
            return Ok(());
        }

        Err(anyhow!("Unrecognized input type"))
    }

    fn mock_external_parser(path: &str, response: &str, status: usize) -> Mock {
        let mut server = MOCK_HTTP_SERVER.lock().unwrap();
        server
            .mock("GET", path)
            .with_body(response)
            .with_status(status)
            .create()
    }

    #[tokio::test]
    async fn test_external_parsing_lnurlp_first_response() -> Result<(), Box<dyn std::error::Error>>
    {
        let input = "123provider.domain32/1";
        let path = format!(
            "/{}",
            percent_encoding::utf8_percent_encode(input, NON_ALPHANUMERIC)
        );
        let response = r#"
        {
            "callback":"callback_url",
            "minSendable":57000,
            "maxSendable":57000,
            "metadata":"[[\"text/plain\", \"External payment\"]]","tag":"payRequest"
        }
        "#;
        let _m = mock_external_parser(&path, response, 200);

        let parsers = vec![ExternalInputParser {
            provider_id: "id".to_string(),
            input_regex: "(.*)(provider.domain)(.*)".to_string(),
            parser_url: "http://127.0.0.1:8080/<input>".to_string(),
        }];

        let input_type = parse(input, Some(&parsers)).await?;
        if let InputType::LnUrlPay { data } = input_type {
            assert_eq!(data.callback, "callback_url");
            assert_eq!(data.max_sendable, 57000);
            assert_eq!(data.min_sendable, 57000);
            assert_eq!(data.comment_allowed, 0);
            assert_eq!(data.domain, "127.0.0.1");

            assert_eq!(data.metadata_vec()?.len(), 1);
            assert_eq!(
                data.metadata_vec()?.first().ok_or("Key not found")?.key,
                "text/plain"
            );
            assert_eq!(
                data.metadata_vec()?.first().ok_or("Key not found")?.value,
                "External payment"
            );
        } else {
            panic!("Expected LnUrlPay, got {:?}", input_type);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_external_parsing_bitcoin_address_and_bolt11(
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Bitcoin parsing endpoint
        let bitcoin_input = "123bitcoin.address.provider32/1";
        let path = format!(
            "/{}",
            percent_encoding::utf8_percent_encode(bitcoin_input, NON_ALPHANUMERIC)
        );
        let bitcoin_address = "1andreas3batLhQa2FawWjeyjCqyBzypd";
        let _bm = mock_external_parser(&path, bitcoin_address, 200);

        // Bolt11 parsing endpoint
        let bolt11_input = "123bolt11.provider32/1";
        let path = format!(
            "/{}",
            percent_encoding::utf8_percent_encode(bolt11_input, NON_ALPHANUMERIC)
        );
        let bolt11 = "lnbc110n1p38q3gtpp5ypz09jrd8p993snjwnm68cph4ftwp22le34xd4r8ftspwshxhmnsdqqxqyjw5qcqpxsp5htlg8ydpywvsa7h3u4hdn77ehs4z4e844em0apjyvmqfkzqhhd2q9qgsqqqyssqszpxzxt9uuqzymr7zxcdccj5g69s8q7zzjs7sgxn9ejhnvdh6gqjcy22mss2yexunagm5r2gqczh8k24cwrqml3njskm548aruhpwssq9nvrvz";
        let _b11m = mock_external_parser(&path, bolt11, 200);

        // Set parsers
        let parsers = vec![
            ExternalInputParser {
                provider_id: "bitcoin".to_string(),
                input_regex: "(.*)(bitcoin.address.provider)(.*)".to_string(),
                parser_url: "http://127.0.0.1:8080/<input>".to_string(),
            },
            ExternalInputParser {
                provider_id: "bolt11".to_string(),
                input_regex: "(.*)(bolt11.provider)(.*)".to_string(),
                parser_url: "http://127.0.0.1:8080/<input>".to_string(),
            },
        ];

        // Parse and check results
        let input_type = parse(bitcoin_input, Some(&parsers)).await?;
        if let InputType::BitcoinAddress { address } = input_type {
            assert_eq!(address.address, bitcoin_address);
        } else {
            panic!("Expected BitcoinAddress, got {:?}", input_type);
        }

        let input_type = parse(bolt11_input, Some(&parsers)).await?;
        if let InputType::Bolt11 { invoice } = input_type {
            assert_eq!(invoice.bolt11, bolt11);
        } else {
            panic!("Expected Bolt11, got {:?}", input_type);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_external_parsing_error() -> Result<(), Box<dyn std::error::Error>> {
        let input = "123provider.domain.error32/1";
        let path = format!(
            "/{}",
            percent_encoding::utf8_percent_encode(input, NON_ALPHANUMERIC)
        );
        let response = "Unrecognized input";
        let _m = mock_external_parser(&path, response, 400);

        let parsers = vec![ExternalInputParser {
            provider_id: "id".to_string(),
            input_regex: "(.*)(provider.domain)(.*)".to_string(),
            parser_url: "http://127.0.0.1:8080/<input>".to_string(),
        }];

        let result = parse(input, Some(&parsers)).await;

        assert!(matches!(result, Err(e) if e.to_string() == "Unrecognized input type"));

        Ok(())
    }
}
