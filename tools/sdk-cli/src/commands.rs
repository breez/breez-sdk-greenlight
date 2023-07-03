use breez_sdk_core::BuyBitcoinProvider;
use breez_sdk_core::EnvironmentType;
use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct SdkCli {
    /// Optional data dir, default to current directory
    #[clap(name = "data_dir", short = 'd', long = "data_dir")]
    pub(crate) data_dir: Option<String>,
}

#[derive(Parser, Debug, Clone, PartialEq)]
#[clap(rename_all = "snake")]
pub(crate) enum Commands {
    /// Set the API key
    SetAPIKey {
        /// The API key to use        
        key: String,
    },
    /// Set the Environment type
    SetEnv {
        /// The environment to use (staging|production)        
        env: EnvironmentType,
    },
    /// Register a new greenlight node
    RegisterNode {
        /// The optional greenlight device certifiate
        #[clap(name = "device_cert", short = 'c', long = "device_cert")]
        device_cert: Option<std::path::PathBuf>,

        /// The optional greenlight device key
        #[clap(name = "device_key", short = 'k', long = "device_key")]
        device_key: Option<std::path::PathBuf>,

        /// The optional greenlight invite code
        #[clap(name = "invite_code", short = 'i', long = "invite_code")]
        invite_code: Option<String>,
    },

    /// Recover a node using the seed only
    RecoverNode {},

    /// Initialize the sdk for an existing node based on the node credentials
    Init {},

    /// Sync local data with remote node
    Sync {},

    /// Triggers a backup of the local data
    Backup {},

    /// Parse a generic string to get its type and relevant metadata
    Parse {
        /// Generic input (URL, LNURL, BIP-21 BTC Address, LN invoice, etc)
        input: String,
    },

    /// Generate a bolt11 invoice
    ReceivePayment { amount: u64, description: String },

    /// Pay using lnurl pay
    LnurlPay { lnurl: String },

    /// Withdraw using lnurl withdraw
    LnurlWithdraw { lnurl: String },

    /// Authenticate using lnurl auth
    LnurlAuth { lnurl: String },

    /// Send on-chain using a reverse swap
    SendOnchain {
        amount_sat: u64,
        onchain_recipient_address: String,
        /// The fee rate for the claim transaction
        sat_per_byte: u64,
    },

    /// Get the current fees for a potential new reverse swap
    FetchOnchainFees {},

    /// Get the current blocking in-progress reverse swaps, if any exist
    InProgressReverseSwaps {},

    /// Send a lightning payment
    SendPayment {
        bolt11: String,

        #[clap(name = "amount", short = 'a', long = "amt")]
        amount: Option<u64>,
    },

    /// Send a spontaneous (keysend) payment
    SendSpontaneousPayment { node_id: String, amount: u64 },

    /// List all payments
    ListPayments {},

    /// Retrieve a payment by its hash
    PaymentByHash { hash: String },

    /// Send on-chain funds to an external address
    Sweep {
        /// The sweep destination address
        to_address: String,

        /// The fee rate for the sweep transaction
        sat_per_byte: u64,
    },

    /// List available LSPs
    ListLsps {},

    /// Connect to an LSP
    ConnectLSP {
        /// The lsp id the sdk should connect to
        lsp_id: String,
    },

    /// The up to date node information
    NodeInfo {},

    /// List fiat currencies
    ListFiat {},

    /// Fetch available fiat rates
    FetchFiatRates {},

    /// Close all LSP channels
    CloseLSPChannels {},

    /// Stop the node
    StopNode {},

    /// List recommended fees based on the mempool
    RecommendedFees {},

    /// Generate address to receive onchain
    ReceiveOnchain {},

    /// Get the current in-progress swap if exists
    InProgressSwap {},

    /// List refundable swap addresses
    ListRefundables {},

    /// Broadcast a refund transaction for an incomplete swap
    Refund {
        swap_address: String,
        to_address: String,
        sat_per_vbyte: u32,
    },

    /// Execute a low level node command (used for debugging)
    ExecuteDevCommand { command: String },

    /// Generates an URL to buy bitcoin from a 3rd party provider
    BuyBitcoin { provider: BuyBitcoinProvider },
}
