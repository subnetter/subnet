/// An address is an account id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    /// 20 bytes unique address derived from public key
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// A non-negative coin amount
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Amount {
    #[prost(uint64, tag = "1")]
    pub value: u64,
    #[prost(enumeration = "CoinType", tag = "2")]
    pub coin_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceTerms {
    /// unique referencable contract id.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// date created. valid from this date
    #[prost(uint64, tag = "2")]
    pub created: u64,
    /// end of time period provider commits to provide service with these prices
    #[prost(uint64, tag = "3")]
    pub valid_until: u64,
    /// pay per usage or monthly fixed
    #[prost(enumeration = "PricingModel", tag = "4")]
    pub pricing_model: i32,
    /// Contract is only for this user. 0 for contract for new users.
    #[prost(bytes = "vec", tag = "5")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
    /// pricing
    ///
    /// free trial period for new users in days
    #[prost(uint32, tag = "6")]
    pub free_trial_period: u32,
    /// required min balance
    #[prost(message, optional, tag = "7")]
    pub min_balance: ::core::option::Option<Amount>,
    /// required max balance (for user tx when balance go below min)
    #[prost(message, optional, tag = "8")]
    pub max_balance: ::core::option::Option<Amount>,
    /// user's balance at date of message creation when terms are for an existing user
    #[prost(message, optional, tag = "9")]
    pub balance: ::core::option::Option<Amount>,
    /// base cost for message routing
    #[prost(message, optional, tag = "10")]
    pub routing_msg_base_cost: ::core::option::Option<Amount>,
    /// cost for message routing per byte
    #[prost(message, optional, tag = "11")]
    pub routing_msg_cost_per_byte: ::core::option::Option<Amount>,
    /// store per byte per monthly cost
    #[prost(message, optional, tag = "12")]
    pub data_store_per_byte: ::core::option::Option<Amount>,
    /// Optional registration fee
    #[prost(message, optional, tag = "13")]
    pub registration_fee: ::core::option::Option<Amount>,
    /// option for fixed monthly fee instead of pay-per-usage
    #[prost(message, optional, tag = "14")]
    pub monthly_fixed_fee: ::core::option::Option<Amount>,
    /// max storage per user limit in bytes
    #[prost(uint64, tag = "15")]
    pub max_user_storage_space: u64,
    /// max supported routed file size in bytes
    #[prost(uint64, tag = "16")]
    pub max_file_size: u64,
    /// provider's blockchain account to receive transactions
    #[prost(message, optional, tag = "17")]
    pub payable_account: ::core::option::Option<Address>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Payment {
    /// time of payment issued
    #[prost(uint64, tag = "1")]
    pub time_stamp: u64,
    /// on or more invoice id, or item id that this payment is for
    #[prost(uint64, repeated, tag = "2")]
    pub item_ids: ::prost::alloc::vec::Vec<u64>,
    /// User public id
    #[prost(bytes = "vec", tag = "3")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
    /// Provider public id
    #[prost(bytes = "vec", tag = "4")]
    pub provider_id: ::prost::alloc::vec::Vec<u8>,
    /// amount to be paid for provider
    #[prost(message, optional, tag = "5")]
    pub amount: ::core::option::Option<Amount>,
    /// user's signature on all above data
    #[prost(bytes = "vec", tag = "6")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionId {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bill {
    /// bill generation time
    #[prost(uint64, tag = "1")]
    pub generated: u64,
    /// current user balance
    #[prost(message, optional, tag = "2")]
    pub balance: ::core::option::Option<Amount>,
    /// all user transaction that credited the account
    #[prost(message, repeated, tag = "3")]
    pub credit_transactions_ids: ::prost::alloc::vec::Vec<TransactionId>,
    /// list of service contracts and user-signed payments under each
    #[prost(message, repeated, tag = "4")]
    pub section: ::prost::alloc::vec::Vec<BillSection>,
    /// provider signature so this message is self-contained
    #[prost(bytes = "vec", tag = "5")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BillSection {
    /// user's service terms
    #[prost(message, optional, tag = "1")]
    pub service_terms: ::core::option::Option<ServiceTerms>,
    /// payments user made under the contract
    #[prost(message, repeated, tag = "2")]
    pub payments: ::prost::alloc::vec::Vec<Payment>,
}
//// Basic Cryptocurrency and Payments Types

/// Supported built-in coin types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CoinType {
    /// $SNP
    Core = 0,
    /// $SNPS
    Stable = 1,
}
// TODO: think about user adding contract id when doing a transaction to start a service with provider. evidence he agrees to the terms.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PricingModel {
    PayPerUsage = 0,
    PayFixedMonthly = 1,
}
// SNP Protocol service for users payments with Service provider.
// Note that new users submit a transaction to start service via the public cryptocurrency api
// provided by Cryptocurrency Nodes
///////////////////////////

/// A request to pay provider by a serviced user
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentRequest {
    #[prost(message, optional, tag = "1")]
    pub payment: ::core::option::Option<Payment>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentResponse {
    #[prost(enumeration = "payment_response::Result", tag = "1")]
    pub result: i32,
    /// user's current balance with provider after the charge
    #[prost(message, optional, tag = "2")]
    pub balance: ::core::option::Option<Amount>,
    /// requests the user to check balance as it becomes low
    #[prost(bool, tag = "3")]
    pub check_balance: bool,
}
/// Nested message and enum types in `PaymentResponse`.
pub mod payment_response {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        Accepted = 0,
        /// client balance too low to process the payment
        RejectedInsufficientFunds = 1,
        RejectedInternalError = 2,
    }
}
////////////////////////////

/// Request to get the current service contract for the user
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceContractRequest {}
/// Current user service prices
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceContractResponse {
    /// current user balance
    #[prost(message, optional, tag = "1")]
    pub balance: ::core::option::Option<Amount>,
    /// current user's service terms
    #[prost(message, optional, tag = "2")]
    pub service_terms: ::core::option::Option<ServiceTerms>,
}
/////////////////////

/// Request client bill. Currently from service start time
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBillRequest {}
/// Response includes a newly generated bill that covers all user payments, current balance and service contract
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBillResponse {
    #[prost(message, optional, tag = "1")]
    pub bill: ::core::option::Option<Bill>,
}
///
/// Wallet related data types used by the WalletService component
// Basic model:
// Safe (file) -> Wallet {seed} -> Account -> {address, pub_key, derive_index}}
/////////////////////////////

/// A wallet account has a derivation index in wallet and a public key. Account address derived from pub key.
/// Account data is stored in wallets. AccountState is stored on the blockchain.
/// Private key computable from account's wallet seed and derivation_index
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    /// time stamp
    #[prost(uint64, tag = "1")]
    pub created: u64,
    /// user provided name
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// wallet derivation used to compute private key
    #[prost(uint32, tag = "3")]
    pub derivation_index: u32,
    /// entry's pub key corresponding to private. Address/AccountId is derived from the pub key.
    #[prost(bytes = "vec", tag = "4")]
    pub pub_key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Seed {
    #[prost(enumeration = "SeedModel", tag = "1")]
    pub model: i32,
    #[prost(enumeration = "SeedType", tag = "2")]
    pub r#type: i32,
    /// for a hot seed
    #[prost(bytes = "vec", tag = "3")]
    pub seed: ::prost::alloc::vec::Vec<u8>,
}
/// A wallet has a hot seed or seed on an associated hardware wallet
/// Wallet is a hierarchical deterministic wallet.
/// See https://en.bitcoin.it/wiki/Deterministic_wallet
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Wallet {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub created: u64,
    /// Accounts used by user of this wallet
    #[prost(message, repeated, tag = "3")]
    pub account: ::prost::alloc::vec::Vec<Account>,
    #[prost(message, optional, tag = "4")]
    pub seed: ::core::option::Option<Seed>,
}
/// Safe contains wallets owned by a single entity - stored in a `Safe file` with seed data secured by AES, KDF and user's password
/// Contains one or more wallets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Safe {
    /// for now we special-case 3 accounts but safe can contain additional wallets and multiple wallets of each type
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub created: u64,
    /// savings. private key in hardware wallet.
    #[prost(message, optional, tag = "3")]
    pub cold_wallet: ::core::option::Option<Wallet>,
    /// spending wallets with private keys accessible.
    #[prost(message, optional, tag = "4")]
    pub hot_wallet: ::core::option::Option<Wallet>,
    /// a wallet for signing messages
    #[prost(message, optional, tag = "5")]
    pub identity_wallet: ::core::option::Option<Wallet>,
    /// additional wallets
    #[prost(message, repeated, tag = "6")]
    pub additional_wallets: ::prost::alloc::vec::Vec<Wallet>,
}
/// Specifies where the seed is stored
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SeedModel {
    /// ledger hardware wallet
    HwLedger = 0,
    /// in wallet file
    Hot = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SeedType {
    /// 128 bit
    Bip32 = 0,
}
