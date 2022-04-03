use cw_storage_plus::{Map, Item, };
use cosmwasm_std::{Uint128, Storage};
use cosmos_types::{Coin, DecCoin};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct PoolInfo {
    pub gamm_denom: String,
    pub unique_token_makers: (String, String, String),
}

pub const GAMM_BONDED_EACH_POOL: f64 = 1000000000000000f64;
pub const TOTAL_REWARD_EACH_EPOCH: f64 = 1000000000000f64;

pub const POOLS: Map<u64, PoolInfo> = Map::new("pools");

pub const LASTEST_UPDATED_EPOCH: Item<u64> = Item::new("last_epoch_updated");

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct TotalGammBonded {
    pub lock_1_day: u128,
    pub lock_7_day: u128,
    pub lock_14_day: u128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct DistrInfo {
    pub total_reward: u128,
    pub reward_per_gamm_lockuped: f64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct PoolState {

    pub total_gamm_bonded: TotalGammBonded,

    pub total_re: u8,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct Lock {
    pub start_epoch: u16,

    pub lock_id: u64,
}

pub const LOCK_IDS: Map<(u16, u16, u8), Lock> = Map::new("lock_ids");

pub const POOLS_STATES: Map<(u16, u16), TotalGammBonded> = Map::new("pools_state_at_epoch");

// reward accumilate up untill a certain epoch from a gamm bonded
pub const REWARD_ACCUMILATIONS_PER_GAMM: Map<(u16, u16, u8), f64> = Map::new("reward_accumilations_per_gamm"); 

pub const DISTR_INFOS: Map<(u16, u16, u8), DistrInfo> = Map::new("distr_infos");

pub fn get_reward_accumulations_per_gamm_key(pool_id: u16, epoch_id: u16, duration_in_day: u8) -> (u16, u16, u8) {
    (pool_id, epoch_id, duration_in_day)
}

pub fn get_lock_key(pool_id: u16, epoch_id: u16, duration_in_day: u8) -> (u16, u16, u8){
    (pool_id, epoch_id, duration_in_day)
}

pub fn get_distr_info_key(pool_id: u16, epoch_id: u16, duration_in_day: u8) -> (u16, u16, u8) {
    (pool_id, epoch_id, duration_in_day)
}

pub fn get_pool_at_epoch_key(pool_id: u16, epoch_id: u16) -> (u16, u16) {
    (pool_id, epoch_id)
}