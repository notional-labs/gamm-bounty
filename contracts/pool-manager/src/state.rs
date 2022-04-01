use cw_storage_plus::{Map, Item, };
use cosmwasm_std::{Uint128, Storage};
use cosmos_types::{Coin, DecCoin};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct EpochState {
    // reward per gamm bonded up until this epoch
    pub total_reward_per_gamm: Vec<DecCoin>,
}

pub const EPOCHSTATES: Map<u64, EpochState> = Map::new("epochstates");

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Config {
    pub first_epoch: u64,

    pub pool_id: u64,

    pub incentivized_reward_denoms: Vec<String>,

    pub gamm_denom: String,

    pub asset_denoms: (String, String)
}

pub const CONFIG: Item<Config> = Item::new("config");

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct LastEpochUpdated {
    pub epoch_id: u64,
    
}

pub const CONTRACT_BALANCES: Map<String, u64> = Map::new("contract_balances");

pub const 

pub const LASTEST_UPDATED_EPOCH: Item<u64> = Item::new("last_epoch_updated");

// a remote account is identified by contract identifier and remote account address (contract identifier + "/" + remote account address)
// this map (remote account identifier, denom) to amount
pub const REMOTE_ACCOUNT_BALANCES: Map<(String, String), Uint128> = Map::new("remote_address_balances");

// this map a remote account identifier to all the gamm coins it owns
pub const GAMM_VAULT: Map<&str, Uint128> = Map::new("gamm_vault");