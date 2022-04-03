use cw_storage_plus::{Map, Item, };
use cosmwasm_std::{Uint128, Storage, StdResult, StdError};
use cosmos_types::{Coin, DecCoin};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct PoolInfo {
    pub gamm_denom: String,
    pub pool_asset_denoms: (String,String),
    pub unique_token_makers: (String, String, String),
}

pub const OUR_GAMM_BONDED_EACH_POOL: u128 = 1000000000000000u128;
pub const TOTAL_REWARD_EACH_EPOCH: u128 =   1000000000000u128;

pub const POOLS: Map<u16, PoolInfo> = Map::new("pools");

pub const LASTEST_UPDATED_EPOCH: Item<u16> = Item::new("last_epoch_updated");

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct TotalGammBonded {
    pub lock_1_day: u128,
    pub lock_7_day: u128,
    pub lock_14_day: u128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct DistrInfo {
    pub denom: String,
    pub total_reward: u128,
    pub reward_per_gamm_lockuped: f64,
}

// #[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub type DistrInfos = Vec<DistrInfo>;

trait GetDistr {
    fn get_distr(&self, denom: String) -> StdResult<&DistrInfo>;
}

impl GetDistr for DistrInfos {
    fn get_distr(&self, denom: String) -> StdResult<&DistrInfo> {
        for distr_info in self{
            if distr_info.denom == denom {
                return Ok(distr_info);
            }
        }
        Err(StdError::generic_err("no distr info found"))
    }
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

pub const LOCK_IDS: Map<(u16, u8), u64> = Map::new("lock_ids");

pub const POOLS_STATES: Map<(u16, u16), TotalGammBonded> = Map::new("pools_state_at_epoch");

// reward accumilate up untill a certain epoch from a gamm bonded
pub const REWARD_ACCUMILATIONS_PER_GAMM: Map<(u16, u16, u8), f64> = Map::new("reward_accumilations_per_gamm"); 

pub const DISTR_INFOS: Map<(u16, u16, u8), DistrInfos> = Map::new("distr_infos");

pub fn get_reward_accumulations_per_gamm_key(pool_id: u16, epoch_id: u16, duration_in_day: u8) -> (u16, u16, u8) {
    (pool_id, epoch_id, duration_in_day)
}

pub fn get_lock_key(pool_id: u16, duration_in_day: u8) -> (u16, u8){
    (pool_id, duration_in_day)
}

pub fn get_distr_info_key(pool_id: u16, epoch_id: u16, duration_in_day: u8) -> (u16, u16, u8) {
    (pool_id, epoch_id, duration_in_day)
}

pub fn get_pool_at_epoch_key(pool_id: u16, epoch_id: u16) -> (u16, u16) {
    (pool_id, epoch_id)
}