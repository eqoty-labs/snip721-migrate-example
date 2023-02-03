use cosmwasm_std::{Binary, ContractInfo};
use serde::{Deserialize, Serialize};
use snip721_reference_impl::token::Metadata;

pub static MIGRATED_FROM_KEY: &[u8] = b"migrated_from";
pub static MIGRATED_TO_KEY: &[u8] = b"migrated_to";
/// storage key for allowed Coin prices for purchasing a mint: Vec<Coin>
pub const PURCHASE_PRICES_KEY: &[u8] = b"prices";
/// storage key for the PurchasableMetadata used for every purchased mint
pub const PURCHASABLE_METADATA_KEY: &[u8] = b"pur_metadata";
/// storage key for current ContractMode
pub const CONTRACT_MODE_KEY: &[u8] = b"pur_contract_mode";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PurchasableMetadata {
    /// optional public metadata that can be seen by everyone
    pub public_metadata: Option<Metadata>,
    /// optional private metadata that can only be seen by the owner and whitelist
    pub private_metadata: Option<Metadata>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MigratedFrom {
    /// the info of the contract being migrated from
    pub contract: ContractInfo,
    /// the secret generated by the contract being migrated from
    pub migration_secret: Binary,
    /// the mint_cnt of the contract being migrated from
    pub migrate_in_mint_cnt: u32,
    /// the next mint index out of migrate_in_mint_cnt that must be migrated
    pub migrate_in_next_mint_index: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MigratedTo {
    /// the info of the contract being migrated to
    pub contract: ContractInfo,
    /// the secret needed by another contract to migrate data out
    pub migration_secret: Binary,
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub enum ContractMode {
    MigrateDataIn = 1,
    Running = 2,
    MigratedOut = 3,
}
