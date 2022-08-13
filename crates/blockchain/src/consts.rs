//  Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
//  This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

// store column families

// stores Blocks (block_id -> block)
pub(crate) const BLOCKS_CF: &str = "blocks";

// stores blockchain metadata such as current block
pub(crate) const BLOCKCHAIN_CF: &str = "blockchain";

pub(crate) const CURRENT_BLOCK_KEY: &str = "curr_block";

// stores txs (tx_id -> TransactionInfo)
pub(crate) const TRANSACTIONS_CF: &str = "txs";

// stores (account_address -> account)
pub(crate) const ACCOUNTS_CF: &str = "accounts";

// (provider_id -> validated_blocks)
pub(crate) const VALIDATOR_BLOCKS_CF: &str = "validated_blocks_by_provider";

// (provider_id -> sealed_blocks)
pub(crate) const SEALER_BLOCKS_CF: &str = "blocks_by_provider";

// providers bundles (provider_id -> bundle)
pub(crate) const PROVIDERS_BUNDLES_CF: &str = "providers_bundles";

// providers bundles (user_id -> bundle)
pub(crate) const CLIENTS_BUNDLES_CF: &str = "users_bundles";

// system settings
pub(crate) const SYSTEM_COL_FAMILY: &str = "system";
