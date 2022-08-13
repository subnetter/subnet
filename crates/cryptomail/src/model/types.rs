#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicKey {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateKey {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Keypair {
    #[prost(bytes = "vec", tag = "1")]
    pub private_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
}
/// an eth address
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthAddress {
    #[prost(bytes = "vec", tag = "1")]
    pub bytes: ::prost::alloc::vec::Vec<u8>,
}
/// An x25519 public key used for DH
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreKey {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// user generated id so client can keep track of multiple pre-keys when user decides to recycle the key
    #[prost(uint32, tag = "2")]
    pub id: u32,
}
/// A collection of PreKeys
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreKeys {
    #[prost(message, repeated, tag = "1")]
    pub pre_keys: ::prost::alloc::vec::Vec<PreKey>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Amount {
    #[prost(enumeration = "Token", tag = "1")]
    pub token: i32,
    /// we use strings to escape numbers and use bigInts from strings
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Payment {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<Amount>,
    /// deposit for thread transaction id
    #[prost(bytes = "vec", tag = "2")]
    pub transaction_id: ::prost::alloc::vec::Vec<u8>,
    /// the action this payment is for
    #[prost(enumeration = "PaidActionType", tag = "3")]
    pub paid_action_type: i32,
}
/// A uniquely global message id is built from unique global thread id and and a message id local to the thread
#[derive(Hash, Eq, Clone, PartialEq, ::prost::Message)]
pub struct MessageId {
    /// 8 bytes / uint64 client generated, must be unique in the thread's context only
    #[prost(bytes = "vec", tag = "1")]
    pub message_thread_id: ::prost::alloc::vec::Vec<u8>,
    /// 8 bytes / uint64 must be globally unique obtained from server
    #[prost(bytes = "vec", tag = "2")]
    pub thread_id: ::prost::alloc::vec::Vec<u8>,
}
/// an 8 bytes globally unique thread id
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreadId {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaidAction {
    #[prost(enumeration = "PaidActionType", tag = "1")]
    pub paid_action_type: i32,
    #[prost(message, optional, tag = "2")]
    pub price: ::core::option::Option<Amount>,
}
/// Settings are user updatable data
/// User must sign
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentSettings {
    /// for payments to this user
    #[prost(message, optional, tag = "1")]
    pub eth_address: ::core::option::Option<EthAddress>,
    #[prost(message, repeated, tag = "2")]
    pub paid_actions: ::prost::alloc::vec::Vec<PaidAction>,
    /// An eth ECDSA eth1.x signature (r,s, v) on the above data. Allows to recover address of signer from sig. Proves signer owns eth_address
    #[prost(bytes = "vec", tag = "3")]
    pub eth_signature: ::prost::alloc::vec::Vec<u8>,
}
/// information about an on-chain deposit state
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositConfirmation {
    /// unique message id (thread_id || msg_id)
    #[prost(message, optional, tag = "1")]
    pub message_id: ::core::option::Option<MessageId>,
    /// deposit amount
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<Amount>,
    /// tx originator (depositor)
    #[prost(message, optional, tag = "3")]
    pub from: ::core::option::Option<EthAddress>,
    /// deposit credited to
    #[prost(message, optional, tag = "4")]
    pub to: ::core::option::Option<EthAddress>,
    /// block num
    #[prost(uint64, tag = "5")]
    pub block_num: u64,
    /// tx block hash
    #[prost(bytes = "vec", tag = "6")]
    pub block_hash: ::prost::alloc::vec::Vec<u8>,
    /// number of confirmations for block
    #[prost(uint64, tag = "7")]
    pub confirmations: u64,
    /// timestamp of block if mined
    #[prost(uint64, tag = "8")]
    pub block_time: u64,
}
/// todo: add following string fields: co name, position and user's full name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicAccountInfo {
    /// account's name. Can be an eth name such as upsetter.eth. Unique in the system
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// optional full name
    #[prost(string, tag = "2")]
    pub full_name: ::prost::alloc::string::String,
    /// world location
    #[prost(string, tag = "3")]
    pub location: ::prost::alloc::string::String,
    /// account's public key - client generated pair
    #[prost(message, optional, tag = "4")]
    pub public_key: ::core::option::Option<PublicKey>,
    /// an x25519 (DH exchange key) wrapper. Can be used to send messages to account
    #[prost(message, optional, tag = "5")]
    pub pre_key: ::core::option::Option<PreKey>,
    /// eth ens_name for transfers. Must be same as eth_address provided in PaymentSettings
    #[prost(string, tag = "6")]
    pub eth_name: ::prost::alloc::string::String,
    /// short descriptive text + emojis
    #[prost(string, tag = "7")]
    pub profile: ::prost::alloc::string::String,
    /// company or org name
    #[prost(string, tag = "8")]
    pub org_name: ::prost::alloc::string::String,
    /// position in org or company
    #[prost(string, tag = "9")]
    pub position: ::prost::alloc::string::String,
    /// main large avatar image (transparent, square)
    #[prost(string, tag = "10")]
    pub profile_image_url: ::prost::alloc::string::String,
    /// small avatar image (square, transparent)
    #[prost(string, tag = "11")]
    pub small_profile_image_url: ::prost::alloc::string::String,
    /// custom profile screen background image
    #[prost(string, tag = "12")]
    pub custom_profile_background_image_url: ::prost::alloc::string::String,
    /// link-tree: website, linkedIn, twitter, etc...
    #[prost(message, repeated, tag = "13")]
    pub profile_urls: ::prost::alloc::vec::Vec<WebResource>,
    #[prost(message, optional, tag = "14")]
    pub payment_settings: ::core::option::Option<PaymentSettings>,
    /// signature of account holder on above info
    #[prost(bytes = "vec", tag = "15")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebResource {
    #[prost(enumeration = "WebResourcesTypes", tag = "1")]
    pub web_resource_type: i32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub url: ::prost::alloc::string::String,
}
/// Currency enum includes all supported payment tokens
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Token {
    Eth = 0,
    Usdc = 1,
    Usdt = 2,
    Dai = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PaidActionType {
    Unspecified = 0,
    /// payment for opening and reading a message
    Open = 1,
    /// payment for replying to a message
    Reply = 2,
    /// payment for watching a short <5 minute video
    WatchVideo = 3,
    /// payment for participating in a 5 min zoom
    PaidActionType5MinZoom = 4,
    /// payment for participating in a 10 min zoom
    PaidActionType10MinZoom = 5,
    /// payment for participating in a 20 min zoom
    PaidActionType20MinZoom = 6,
}
/// Web resources recognized by type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WebResourcesTypes {
    Unknown = 0,
    Website = 1,
    Twitter = 2,
    Telegram = 3,
    Linkedin = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MimeType {
    TextUtf8 = 0,
    ImagePng = 1,
    ImageJpg = 2,
    ImageGif = 3,
    /// utf8 md text format
    TextMd = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Compression {
    Uncompressed = 0,
    Zip = 1,
}
/// Account is a User Account. Note that all account info so public including settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    /// public address is last 20 bytes of pub_key
    #[prost(message, optional, tag = "1")]
    pub id_pub_key: ::core::option::Option<PublicKey>,
    /// 0..100
    #[prost(message, optional, tag = "2")]
    pub reputation: ::core::option::Option<Reputation>,
    /// time account created
    #[prost(uint64, tag = "3")]
    pub time_created: u64,
    /// time last login to client
    #[prost(uint64, tag = "4")]
    pub time_last_login: u64,
    /// user required updatable settings
    #[prost(message, optional, tag = "5")]
    pub settings: ::core::option::Option<Settings>,
    /// signed public account info including payment settings
    #[prost(message, optional, tag = "8")]
    pub public_account_info: ::core::option::Option<PublicAccountInfo>,
}
/// Settings are user updatable data
/// User must sign
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Settings {
    /// list account in account directory - all accounts are public - that's the point of this system
    #[prost(bool, tag = "1")]
    pub public_list_account: bool,
    /// when true show rotation art pieces in client background
    #[prost(bool, tag = "2")]
    pub display_art_background: bool,
    /// account active state
    #[prost(bool, tag = "3")]
    pub active: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reputation {
    /// Number of message received with a valid deposit to open the message and amount equals or greater than the user's open price
    #[prost(uint64, tag = "1")]
    pub open_paid_messages_received: u64,
    /// Number of valid open messages opened by the user
    #[prost(uint64, tag = "2")]
    pub open_paid_message_opened: u64,
    /// Number of messages received with a valid deposit to reply to the message and amount equals or greater than the user's reply price
    #[prost(uint64, tag = "3")]
    pub messages_reply_paid_received: u64,
    /// Number of valid reply message opened by the user
    #[prost(uint64, tag = "4")]
    pub messages_reply_paid_opened: u64,
    /// Number of deposits redeemed by account w/o opening their messages
    #[prost(uint64, tag = "5")]
    pub payment_redeemed_no_open: u64,
    /// Number of deposits redeemed by account w/o replying to their messages
    #[prost(uint64, tag = "6")]
    pub payment_redeemed_no_reply: u64,
    /// Computed offline based on all data above - stored here for quick display
    #[prost(float, tag = "7")]
    pub reputation_score: f32,
    /// Service join counter
    #[prost(uint64, tag = "8")]
    pub og_rank: u64,
    /// Current period cmail tokens balance
    #[prost(uint64, tag = "9")]
    pub cmail_token_balance_cur_period: u64,
    /// Last period amount credited to user's wallet
    #[prost(uint64, tag = "10")]
    pub last_drop_cmail_tokens: u64,
    /// all accumulated cmail token balance
    #[prost(uint64, tag = "11")]
    pub cmail_token_balance_total_earned: u64,
}
/// A message has 2 parts - author data and server data. Each part is signed by its creator
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(message, optional, tag = "1")]
    pub message_id: ::core::option::Option<MessageId>,
    /// serialized MessageUserdata
    #[prost(bytes = "vec", tag = "2")]
    pub author_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub server_data: ::core::option::Option<MessageServerData>,
    /// author signature on author_data MessageUserData
    #[prost(bytes = "vec", tag = "4")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// Message parts that are authored and signed by user and not generated on the server
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageUserdata {
    #[prost(message, optional, tag = "1")]
    pub message_id: ::core::option::Option<MessageId>,
    /// sender public key, used to verify signature and to use public account info
    #[prost(message, optional, tag = "2")]
    pub sender_public_key: ::core::option::Option<PublicKey>,
    /// timestamp - epoch time nanos
    #[prost(uint64, tag = "3")]
    pub created: u64,
    /// when author created a payment transaction for this message
    #[prost(message, optional, tag = "4")]
    pub payment: ::core::option::Option<Payment>,
    /// thread message id this is a reply to
    #[prost(bytes = "vec", tag = "5")]
    pub reply_to: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "6")]
    pub recipient_public_key: ::core::option::Option<PublicKey>,
    /// pub ephemeral key created by author, used for decryption of content. Note that this is a x25519_dalek::PublicKey wrapper and not ed25519_dalek::PublicKey
    #[prost(message, optional, tag = "7")]
    pub eph_pub_key: ::core::option::Option<PublicKey>,
    /// the id of the recipient's public pre-key used to encrypt this message
    #[prost(uint32, tag = "8")]
    pub recipient_pre_key_id: u32,
    /// encrypted MessageContent with recipient pub key and sender pub key for DH to obtain aes key
    #[prost(bytes = "vec", tag = "9")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
/// Wrapper of a list of MessageIds
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessagesIds {
    #[prost(message, repeated, tag = "1")]
    pub messages_ids: ::prost::alloc::vec::Vec<MessageId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageKey {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<MessageId>,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
/// A collection of MessageKey(s)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageKeys {
    #[prost(message, repeated, tag = "1")]
    pub message_keys: ::prost::alloc::vec::Vec<MessageKey>,
}
/// Message parts that are generated by the service and not by the author
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageServerData {
    /// recipient opened the message
    #[prost(bool, tag = "1")]
    pub opened: bool,
    /// recipient replied to the message
    #[prost(bool, tag = "2")]
    pub replied: bool,
    #[prost(message, optional, tag = "3")]
    pub deposit_data: ::core::option::Option<DepositData>,
    /// server signature
    #[prost(bytes = "vec", tag = "4")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositData {
    /// number of deposit confirm attempts
    #[prost(uint64, tag = "1")]
    pub verify_attempts: u64,
    /// timestamp of last deposit confirmation attempt
    #[prost(uint64, tag = "2")]
    pub last_verify_attempt: u64,
    /// for a paid message, deposit smart contract info or empty if none available yet
    #[prost(message, optional, tag = "3")]
    pub deposit_confirmation: ::core::option::Option<DepositConfirmation>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Thread {
    /// thread ids are unique 8 bytes across whole namespace - created with first message
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    /// A thread is a collection of 1 or more messages, ordered by creation time. Message thread id is 8 bytes
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub msgs_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreadBox {
    #[prost(enumeration = "ThreadBoxType", tag = "1")]
    pub thread_box_type: i32,
    /// ordered list of threads
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub thread_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// MessageContent is an ordered collection of Content items
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageContent {
    /// subject is utf-8 string
    #[prost(message, optional, tag = "1")]
    pub subject: ::core::option::Option<ContentItem>,
    /// body is utf-8 string of md text to be rendered (including links)
    #[prost(message, optional, tag = "2")]
    pub body: ::core::option::Option<ContentItem>,
    /// zero or more additional media items
    #[prost(message, repeated, tag = "3")]
    pub media_items: ::prost::alloc::vec::Vec<ContentItem>,
}
/// Content represents an individual content item such text or image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentItem {
    /// data encoding. eg. text/utf-8, image/gif
    #[prost(enumeration = "MimeType", tag = "1")]
    pub mime_type: i32,
    /// optional compression done on data field. e.g. deflate/zip
    #[prost(enumeration = "Compression", tag = "2")]
    pub compression: i32,
    /// data. e.g. utf-8 string, png, gif or jpeg
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ThreadBoxType {
    Unknown = 0,
    Inbox = 1,
    Sent = 2,
    Archive = 4,
}
