use cw_storage_plus::{Map, Item, };
use cosmwasm_std::{Uint128, Storage};
use cosmos_types::{Coin, DecCoin};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct Pool {
    pub gamm_denom: String,
    pub unique_token_makers: (String, String, String),
}

pub const GAMM_BONDED_EACH_POOL: f64 = 1000000000000000f64;
pub const TOTAL_REWARD_EACH_EPOCH: f64    = 1000000000000f64;

pub const POOLS: Map<u64, Pool> = Map::new("pools");

pub const LASTEST_UPDATED_EPOCH: Item<u64> = Item::new("last_epoch_updated");

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct TotalLockUp {
    pub lock_1_day: u128,
    pub lock_7_day: u128,
    pub lock_14_day: u128,
}

pub const POOLS_STATE_AT_EACH_EPOCH: Map<(u64, u64), TotalLockUp> = Map::new("pools_state_at_epoch");

pub fn get_pool_at_epoch_key(pool_id: u64, epoch_id: u64) -> (u64, u64) {
    (pool_id, epoch_id)
}
