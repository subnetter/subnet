//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

// config keys
pub(crate) const ETH_NET_ID_CONFIG_KEY: &str = "eth_net_id";
pub(crate) const DEPOSIT_CONFIRMATIONS_CONFIG_KEY: &str = "min_deposit_confirms";
pub(crate) const ALLOW_HTTP_USER_MEDIA_KEY: &str = "allow_http_user_media";

// store column families
pub(crate) const THREADS_COL_FAMILY: &str = "threads"; // stores Threads (thread_id -> thread: ordered lists of messages)
pub(crate) const ACCOUNTS_COL_FAMILY: &str = "accounts"; // user accounts (account_address -> account)
pub(crate) const BOXES_COL_FAMILY: &str = "boxes"; // holds users boxes -inbox, sent, archive. (account_address || [In, Sent, Archive] -> box)
pub(crate) const ACCOUNTS_NAMES_COL_FAMILY: &str = "account_names"; // Unique account names (account_name -> account_address)
pub(crate) const PUB_ACCOUNTS_COL_FAMILY: &str = "pub_accounts"; // index of publicly listed accounts. (account_name -> account_address)
pub(crate) const MESSAGES_COL_FAMILY: &str = "messages"; // holds messages (thread_id ||msg_id -> msg )

// COL family for system stored sets such as unconfirmed paid messages. Structure:
// (UNCONFIRMED_MSGS_IDS_KEY -> set of messages ids)
pub(crate) const SYSTEM_COL_FAMILY: &str = "system";
pub(crate) const UNCONFIRMED_MSGS_IDS_KEY: &str = "unconfirmed_msgs"; // Messages pending deposit confirmations. value: set of messages ids.

pub(crate) const ACCOUNTS_COUNTER_ID_KEY: &str = "accounts_counter";

// misc consts
pub(crate) const MSG_ID_LEN: usize = 16;
pub(crate) const THREAD_ID_LEN: usize = 8;
pub(crate) const MSG_THREAD_ID_LEN: usize = 8;

pub(crate) const ETH_ADDRESS_LEN: usize = 20;
pub(crate) const PUB_KEY_LEN: usize = 32; // ed25519 pub key length
pub(crate) const PRE_KEY_LEN: usize = 32; // ex25519 pub key length
pub(crate) const MAX_ACCOUNT_NAME: usize = 32;
pub const MAX_TIME_DRIFT_NANO_SECS: i64 = 60 * 60 * 48 * 1000 * 1000 * 1000; // 48 hours in nano secs. Used in checking for valid deposits
pub const ETH_PRICE_CACHE_DUR_NANO_SECS: i64 = 30 * 60 * 1000 * 1000 * 1000; // 30 minutes in nano secs.

// pub(crate) const DEPOSITS_VALIDATION_TASK_FREQ_SEQ: u64 = 60; // todo: move to config

pub(crate) const MAX_BINARY_CONTENT_SIZE_BYTES: usize = 30_1024; // 30K - encrypted message content max size

// cmail tokens for signing up
pub(crate) const SIGN_UP_TOKENS_AMOUNT: u64 = 100;

// cmail tokens for sending a paid message (pay to open or pay to reply)
pub(crate) const PAID_MESSAGE_SENT_TOKENS_AMOUNT: u64 = 100;

// cmail tokens for receiving a pay to open message
pub(crate) const PAID_MESSAGE_RECEIVED_TOKENS_AMOUNT: u64 = 100;

// cmail tokens for sending a message with a paid reply deposit
pub(crate) const PAID_REPLY_SENT_TOKENS_AMOUNT: u64 = 100;

// cmail tokens for receiving a reply to a paid reply message
pub(crate) const PAID_REPLY_RECEIVED_TOKENS_AMOUNT: u64 = 100;

// cmail tokens for opening a paid message
pub(crate) const READ_PAID_MESSAGE_TOKENS_AMOUNT: u64 = 100;

// Kovan testnet net id
pub(crate) const KOVAN_NET_ID: u64 = 42;

// pub(crate) const LOCAL_DEVNET_ID: u64 = 8545;

// for kovan - for mainnet - change to 6
pub(crate) const DEPOSIT_CONFIRMATION_PERIOD_BLOCKS: u64 = 3;
