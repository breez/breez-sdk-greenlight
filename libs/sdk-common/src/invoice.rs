use std::num::ParseIntError;
use std::str::FromStr;
use std::time::{SystemTimeError, UNIX_EPOCH};

use anyhow::anyhow;
use bitcoin::secp256k1::{self, PublicKey};
use hex::ToHex;
use lightning::routing::gossip::RoutingFees;
use lightning::routing::*;
use lightning_invoice::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
#[cfg(feature = "liquid")]
use {
    bitcoin::hashes::hex::ToHex as BitcoinHashToHex, lightning_125::ln::msgs::DecodeError,
    lightning_125::offers::offer::Offer, lightning_125::offers::parse::Bolt12ParseError,
};

use crate::prelude::*;

pub type InvoiceResult<T, E = InvoiceError> = Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum InvoiceError {
    #[error("{0}")]
    Generic(String),

    #[error("{0}")]
    InvalidNetwork(String),

    #[error("{0}")]
    Validation(String),
}

impl InvoiceError {
    pub fn generic(err: &str) -> Self {
        Self::Generic(err.to_string())
    }

    pub fn invalid_network(err: &str) -> Self {
        Self::InvalidNetwork(err.to_string())
    }

    pub fn validation(err: &str) -> Self {
        Self::Validation(err.to_string())
    }
}

impl From<CreationError> for InvoiceError {
    fn from(err: CreationError) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<Bolt11ParseError> for InvoiceError {
    fn from(err: Bolt11ParseError) -> Self {
        Self::Validation(err.to_string())
    }
}

impl From<Bolt11SemanticError> for InvoiceError {
    fn from(err: Bolt11SemanticError) -> Self {
        Self::Validation(err.to_string())
    }
}

impl From<ParseIntError> for InvoiceError {
    fn from(err: ParseIntError) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<regex::Error> for InvoiceError {
    fn from(err: regex::Error) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<secp256k1::Error> for InvoiceError {
    fn from(err: secp256k1::Error) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<SystemTimeError> for InvoiceError {
    fn from(err: SystemTimeError) -> Self {
        Self::Generic(err.to_string())
    }
}

fn parse_short_channel_id(id_str: &str) -> InvoiceResult<u64> {
    let parts: Vec<&str> = id_str.split('x').collect();
    if parts.len() != 3 {
        return Err(InvoiceError::generic("Invalid short channel id"));
    }
    let block_num = parts[0].parse::<u64>()?;
    let tx_num = parts[1].parse::<u64>()?;
    let tx_out = parts[2].parse::<u64>()?;

    Ok((block_num & 0xFFFFFF) << 40 | (tx_num & 0xFFFFFF) << 16 | (tx_out & 0xFFFF))
}

fn format_short_channel_id(id: u64) -> String {
    let block_num = (id >> 40) as u32;
    let tx_num = ((id >> 16) & 0xFFFFFF) as u32;
    let tx_out = (id & 0xFFFF) as u16;
    format!("{block_num}x{tx_num}x{tx_out}")
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Amount {
    Bitcoin {
        amount_msat: u64,
    },
    /// An amount of currency specified using ISO 4712.
    Currency {
        /// The currency that the amount is denominated in.
        iso4217_code: String,
        /// The amount in the currency unit adjusted by the ISO 4712 exponent (e.g., USD cents).
        fractional_amount: u64,
    },
}

impl TryFrom<lightning::offers::offer::Amount> for Amount {
    type Error = anyhow::Error;

    fn try_from(amount: lightning::offers::offer::Amount) -> Result<Self, Self::Error> {
        match amount {
            lightning::offers::offer::Amount::Bitcoin { amount_msats } => Ok(Amount::Bitcoin {
                amount_msat: amount_msats,
            }),
            lightning::offers::offer::Amount::Currency {
                iso4217_code,
                amount,
            } => Ok(Amount::Currency {
                iso4217_code: String::from_utf8(iso4217_code.to_vec())
                    .map_err(|_| anyhow!("Expecting a valid ISO 4217 character sequence"))?,
                fractional_amount: amount,
            }),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LNOffer {
    /// String representation of the Bolt12 offer
    pub offer: String,
    pub chains: Vec<String>,
    /// If set, it represents the minimum amount that an invoice must have to be valid for this offer
    pub min_amount: Option<Amount>,
    pub description: Option<String>,
    /// Epoch time from which an invoice should no longer be requested. If None, the offer does not expire.
    pub absolute_expiry: Option<u64>,
    pub issuer: Option<String>,
    /// The public key used by the recipient to sign invoices.
    pub signing_pubkey: Option<String>,
    pub paths: Vec<LnOfferBlindedPath>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LnOfferBlindedPath {
    /// For each blinded hop, we store the node ID (pubkey as hex).
    pub blinded_hops: Vec<String>,
}

/// Wrapper for a BOLT11 LN invoice
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LNInvoice {
    pub bolt11: String,
    pub network: Network,
    pub payee_pubkey: String,
    pub payment_hash: String,
    pub description: Option<String>,
    pub description_hash: Option<String>,
    pub amount_msat: Option<u64>,
    pub timestamp: u64,
    pub expiry: u64,
    pub routing_hints: Vec<RouteHint>,
    pub payment_secret: Vec<u8>,
    pub min_final_cltv_expiry_delta: u64,
}

impl LNInvoice {
    pub fn contains_hint_for_node(&self, pubkey: &str) -> bool {
        self.routing_hints
            .iter()
            .any(|hint| hint.hops.iter().any(|hop| hop.src_node_id == pubkey))
    }
}

/// Details of a specific hop in a larger route hint
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteHintHop {
    /// The node_id of the non-target end of the route
    pub src_node_id: String,
    /// The short_channel_id of this channel
    pub short_channel_id: String,
    /// The fees which must be paid to use this channel
    pub fees_base_msat: u32,
    pub fees_proportional_millionths: u32,

    /// The difference in CLTV values between this node and the next node.
    pub cltv_expiry_delta: u64,
    /// The minimum value, in msat, which must be relayed to the next hop.
    pub htlc_minimum_msat: Option<u64>,
    /// The maximum value in msat available for routing with a single HTLC.
    pub htlc_maximum_msat: Option<u64>,
}

/// A route hint for a LN payment
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteHint {
    pub hops: Vec<RouteHintHop>,
}

impl RouteHint {
    pub fn to_ldk_hint(&self) -> InvoiceResult<router::RouteHint> {
        let mut hops = Vec::new();
        for hop in self.hops.iter() {
            let pubkey_res = PublicKey::from_str(&hop.src_node_id)?;

            let router_hop = router::RouteHintHop {
                src_node_id: pubkey_res,
                short_channel_id: parse_short_channel_id(&hop.short_channel_id)?,
                fees: RoutingFees {
                    base_msat: hop.fees_base_msat,
                    proportional_millionths: hop.fees_proportional_millionths,
                },
                cltv_expiry_delta: hop.cltv_expiry_delta as u16,
                htlc_minimum_msat: hop.htlc_minimum_msat,
                htlc_maximum_msat: hop.htlc_maximum_msat,
            };
            hops.push(router_hop);
        }
        Ok(router::RouteHint(hops))
    }

    pub fn from_ldk_hint(hint: &router::RouteHint) -> RouteHint {
        let mut hops = Vec::new();
        for hop in hint.0.iter() {
            let pubkey_res = hop.src_node_id.serialize().encode_hex::<String>();

            let router_hop = RouteHintHop {
                src_node_id: pubkey_res,
                short_channel_id: format_short_channel_id(hop.short_channel_id),
                fees_base_msat: hop.fees.base_msat,
                fees_proportional_millionths: hop.fees.proportional_millionths,
                cltv_expiry_delta: u64::from(hop.cltv_expiry_delta),
                htlc_minimum_msat: hop.htlc_minimum_msat,
                htlc_maximum_msat: hop.htlc_maximum_msat,
            };
            hops.push(router_hop);
        }
        RouteHint { hops }
    }
}

pub fn add_routing_hints(
    invoice: &str,
    merge_with_existing: bool,
    route_hints: &Vec<RouteHint>,
    new_amount_msats: Option<u64>,
) -> InvoiceResult<RawBolt11Invoice> {
    let signed = invoice.parse::<SignedRawBolt11Invoice>()?;
    let invoice = Bolt11Invoice::from_signed(signed)?;

    let mut ldk_hints: Vec<router::RouteHint> = vec![];
    for h in route_hints {
        ldk_hints.push(h.to_ldk_hint()?);
    }

    let mut invoice_builder = InvoiceBuilder::new(invoice.currency())
        .invoice_description(invoice.description())
        .payment_hash(*invoice.payment_hash())
        .timestamp(invoice.timestamp())
        .expiry_time(invoice.expiry_time())
        .payment_secret(*invoice.payment_secret())
        .min_final_cltv_expiry_delta(invoice.min_final_cltv_expiry_delta())
        .basic_mpp();
    if let Some(new_amount_msat) = new_amount_msats {
        invoice_builder = invoice_builder.amount_milli_satoshis(new_amount_msat)
    }

    // When merging route hints, only route hints are added that go through different nodes than ones in the invoice route hints.
    // Otherwise when not merging route hints, the invoice route hints are replaced by the provided route hints.
    let unique_hop_hints: Vec<router::RouteHint> = match route_hints.len() {
        0 => invoice.route_hints(),
        _ => match merge_with_existing {
            true => {
                let invoice_hints_hop_src_node_ids: Vec<String> = invoice
                    .route_hints()
                    .into_iter()
                    .flat_map(|hint| hint.0)
                    .map(|hop| hop.src_node_id.serialize().encode_hex::<String>())
                    .collect();

                let unique_to_add: Vec<&RouteHint> = route_hints
                    .iter()
                    .filter(|hint| {
                        hint.hops
                            .iter()
                            .all(|hop| !invoice_hints_hop_src_node_ids.contains(&hop.src_node_id))
                    })
                    .collect();

                // Adding the lsp hint
                let mut all_hints = invoice.route_hints();
                for hint in unique_to_add {
                    all_hints.push(hint.to_ldk_hint()?);
                }
                all_hints
            }
            false => ldk_hints,
        },
    };

    // Adding the unique existing hints
    for hint in unique_hop_hints {
        invoice_builder = invoice_builder.private_route(hint);
    }

    Ok(invoice_builder.build_raw()?)
}

// Validate that the LNInvoice network matches the provided network
pub fn validate_network(invoice: LNInvoice, network: Network) -> InvoiceResult<()> {
    match invoice.network == network {
        true => Ok(()),
        false => Err(InvoiceError::invalid_network(
            "Invoice network does not match config",
        )),
    }
}

/// Parse a BOLT11 payment request and return a structure contains the parsed fields.
pub fn parse_invoice(bolt11: &str) -> InvoiceResult<LNInvoice> {
    if bolt11.trim().is_empty() {
        return Err(InvoiceError::validation("Bolt11 is an empty string"));
    }
    let re = Regex::new(r"(?i)^lightning:")?;
    let bolt11 = re.replace_all(bolt11, "");
    let signed = bolt11.parse::<SignedRawBolt11Invoice>()?;
    let invoice = Bolt11Invoice::from_signed(signed)?;
    let since_the_epoch = invoice.timestamp().duration_since(UNIX_EPOCH)?;

    // make sure signature is valid
    invoice.check_signature()?;

    // Try to take payee pubkey from the tagged fields, if doesn't exist recover it from the signature
    let payee_pubkey: String = match invoice.payee_pub_key() {
        Some(key) => key.serialize().encode_hex::<String>(),
        None => invoice
            .recover_payee_pub_key()
            .serialize()
            .encode_hex::<String>(),
    };

    // convert hints to bridge interface
    let invoice_hints = invoice.route_hints();
    let converted_hints = invoice_hints.iter().map(RouteHint::from_ldk_hint).collect();
    // return the parsed invoice
    let ln_invoice = LNInvoice {
        bolt11: bolt11.to_string(),
        network: invoice.network().into(),
        payee_pubkey,
        expiry: invoice.expiry_time().as_secs(),
        amount_msat: invoice.amount_milli_satoshis(),
        timestamp: since_the_epoch.as_secs(),
        routing_hints: converted_hints,
        payment_hash: invoice.payment_hash().encode_hex::<String>(),
        payment_secret: invoice.payment_secret().0.to_vec(),
        description: match invoice.description() {
            Bolt11InvoiceDescription::Direct(msg) => Some(msg.to_string()),
            Bolt11InvoiceDescription::Hash(_) => None,
        },
        description_hash: match invoice.description() {
            Bolt11InvoiceDescription::Direct(_) => None,
            Bolt11InvoiceDescription::Hash(h) => Some(h.0.to_string()),
        },
        min_final_cltv_expiry_delta: invoice.min_final_cltv_expiry_delta(),
    };
    Ok(ln_invoice)
}

#[cfg(feature = "liquid")]
// Covers BIP 21 URIs and simple onchain Liquid addresses (which are valid BIP 21 with the 'liquidnetwork:' prefix)
pub fn parse_liquid_address(input: &str) -> Result<LiquidAddressData, DeserializeError> {
    LiquidAddressData::from_addr(input).or_else(|_| input.parse::<LiquidAddressData>())
}

#[cfg(feature = "liquid")]
pub fn parse_bolt12_offer(input: &str) -> Result<LNOffer, Bolt12ParseError> {
    let offer = input.parse::<Offer>()?;
    // TODO This conversion (between lightning-v0.0.125 to -v0.0.118 Amount types)
    //      won't be needed when gl-client upgrades to >=0.0.125
    let min_amount = offer
        .amount()
        .map(|amount| match amount {
            lightning_125::offers::offer::Amount::Bitcoin { amount_msats } => Ok(Amount::Bitcoin {
                amount_msat: amount_msats,
            }),
            lightning_125::offers::offer::Amount::Currency {
                iso4217_code,
                amount,
            } => Ok(Amount::Currency {
                iso4217_code: String::from_utf8(iso4217_code.to_vec())
                    .map_err(|_| anyhow!("Expecting a valid ISO 4217 character sequence"))?,
                fractional_amount: amount,
            }),
        })
        .transpose()
        .map_err(|_e: anyhow::Error| Bolt12ParseError::Decode(DecodeError::InvalidValue))?;

    Ok(LNOffer {
        offer: input.to_string(),
        chains: offer
            .chains()
            .iter()
            .map(|chain| chain.to_string())
            .collect(),
        min_amount,
        description: offer.description().map(|d| d.to_string()),
        absolute_expiry: offer.absolute_expiry().map(|expiry| expiry.as_secs()),
        issuer: offer.issuer().map(|s| s.to_string()),
        signing_pubkey: offer.signing_pubkey().map(|pk| pk.to_string()),
        paths: offer
            .paths()
            .iter()
            .map(|path| LnOfferBlindedPath {
                blinded_hops: path
                    .blinded_hops()
                    .iter()
                    .map(|hop| hop.blinded_node_id.to_hex())
                    .collect(),
            })
            .collect::<Vec<LnOfferBlindedPath>>(),
    })
}

#[cfg(test)]
mod tests {
    use crate::invoice::*;

    #[test]
    fn test_parse_invoice() {
        let payreq = String::from("lnbc110n1p38q3gtpp5ypz09jrd8p993snjwnm68cph4ftwp22le34xd4r8ftspwshxhmnsdqqxqyjw5qcqpxsp5htlg8ydpywvsa7h3u4hdn77ehs4z4e844em0apjyvmqfkzqhhd2q9qgsqqqyssqszpxzxt9uuqzymr7zxcdccj5g69s8q7zzjs7sgxn9ejhnvdh6gqjcy22mss2yexunagm5r2gqczh8k24cwrqml3njskm548aruhpwssq9nvrvz");
        let res = parse_invoice(&payreq).unwrap();

        let private_key_vec =
            hex::decode("3e171115f50b2c355836dc026a6d54d525cf0d796eb50b3460a205d25c9d38fd")
                .unwrap();
        let mut private_key: [u8; 32] = Default::default();
        private_key.copy_from_slice(&private_key_vec[0..32]);
        let hint_hop = self::RouteHintHop {
            src_node_id: res.payee_pubkey,
            short_channel_id: format_short_channel_id(1234),
            cltv_expiry_delta: 2000,
            htlc_minimum_msat: Some(3000),
            htlc_maximum_msat: Some(4000),
            fees_base_msat: 1000,
            fees_proportional_millionths: 100,
        };
        let route_hint = self::RouteHint {
            hops: vec![hint_hop],
        };
        let encoded = add_routing_hints(&payreq, true, &vec![route_hint], Some(100)).unwrap();
        print!("{encoded:?}");
    }

    #[test]
    fn test_parse_invoice_network() {
        let payreq = String::from("lnbc110n1p38q3gtpp5ypz09jrd8p993snjwnm68cph4ftwp22le34xd4r8ftspwshxhmnsdqqxqyjw5qcqpxsp5htlg8ydpywvsa7h3u4hdn77ehs4z4e844em0apjyvmqfkzqhhd2q9qgsqqqyssqszpxzxt9uuqzymr7zxcdccj5g69s8q7zzjs7sgxn9ejhnvdh6gqjcy22mss2yexunagm5r2gqczh8k24cwrqml3njskm548aruhpwssq9nvrvz");
        let res: LNInvoice = parse_invoice(&payreq).unwrap();
        assert!(validate_network(res.clone(), Network::Bitcoin).is_ok());

        let private_key_vec =
            hex::decode("3e171115f50b2c355836dc026a6d54d525cf0d796eb50b3460a205d25c9d38fd")
                .unwrap();
        let mut private_key: [u8; 32] = Default::default();
        private_key.copy_from_slice(&private_key_vec[0..32]);
        let hint_hop = self::RouteHintHop {
            src_node_id: res.payee_pubkey,
            short_channel_id: format_short_channel_id(1234),
            fees_base_msat: 1000,
            fees_proportional_millionths: 100,
            cltv_expiry_delta: 2000,
            htlc_minimum_msat: Some(3000),
            htlc_maximum_msat: Some(4000),
        };
        let route_hint = self::RouteHint {
            hops: vec![hint_hop],
        };
        let encoded = add_routing_hints(&payreq, false, &vec![route_hint], Some(100)).unwrap();
        print!("{encoded:?}");
    }

    #[test]
    fn test_parse_invoice_invalid_bitcoin_network() {
        let payreq = String::from("lnbc110n1p38q3gtpp5ypz09jrd8p993snjwnm68cph4ftwp22le34xd4r8ftspwshxhmnsdqqxqyjw5qcqpxsp5htlg8ydpywvsa7h3u4hdn77ehs4z4e844em0apjyvmqfkzqhhd2q9qgsqqqyssqszpxzxt9uuqzymr7zxcdccj5g69s8q7zzjs7sgxn9ejhnvdh6gqjcy22mss2yexunagm5r2gqczh8k24cwrqml3njskm548aruhpwssq9nvrvz");
        let res = parse_invoice(&payreq);

        assert!(res.is_ok());
        assert!(validate_network(res.unwrap(), Network::Testnet).is_err());
    }

    #[test]
    fn test_parse_invoice_invalid_testnet_network() {
        let payreq = String::from("lntb15u1pj53l9tpp5p7kjsjcv3eqa39upytmj6k7ac8rqvdffyqr4um98pq5n4ppwxvnsdpzxysy2umswfjhxum0yppk76twypgxzmnwvyxqrrsscqp79qy9qsqsp53xw4x5ezpzvnheff9mrt0ju72u5a5dnxyh4rq6gtweufv9650d4qwqj3ds5xfg4pxc9h7a2g43fmntr4tt322jzujsycvuvury50u994kzr8539qf658hrp07hyz634qpvkeh378wnvf7lddp2x7yfgyk9cp7f7937");
        let res = parse_invoice(&payreq);

        assert!(res.is_ok());
        assert!(validate_network(res.unwrap(), Network::Bitcoin).is_err());
    }

    #[test]
    fn test_format_short_channel_id() {
        let valid_short_channel_ids = vec![
            (0, "0x0x0"),
            (936_502_917_475_117, "851x12489658x11053"),
            (455_944_619_913_684, "414x11395355x29140"),
            (u64::MAX, "16777215x16777215x65535"),
        ];
        for (scid, scid_str) in valid_short_channel_ids {
            let res = format_short_channel_id(scid);
            assert_eq!(res, scid_str);
        }
    }

    #[test]
    fn test_parse_short_channel_id() {
        let valid_short_channel_ids = vec![
            ("0x0x0", 0),
            ("16000000x0x3965", 17_592_186_044_416_003_965),
            ("94838x10x3", 104_275_483_755_675_651),
            ("16777215x16777215x65535", u64::MAX),
        ];
        for (scid_str, scid) in valid_short_channel_ids {
            let res = parse_short_channel_id(scid_str);
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), scid);
        }

        let invalid_short_channel_ids = vec![
            "0",
            "16000000x0x-3965",
            "18446744073709551615",
            "16777215x65535",
        ];
        for scid_str in invalid_short_channel_ids {
            let res = parse_short_channel_id(scid_str);
            assert!(res.is_err());
        }
    }
}
