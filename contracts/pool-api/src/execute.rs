use cosmwasm_std::{
    DepsMut, Response, StdResult, MessageInfo
};

use crate::state::{
    
};

use crate::msg::{
  UpdateEpochMsg
};

use cosmwasm_std::{
    entry_point, to_binary, Binary, CosmosMsg, from_binary,
    Empty, Env, Event, IbcBasicResponse, IbcChannelCloseMsg, IbcChannelConnectMsg,
    IbcChannelOpenMsg, IbcPacketAckMsg, IbcPacketReceiveMsg, IbcPacketTimeoutMsg,
    IbcReceiveResponse,  StdError, 
    QueryRequest, BankMsg
};

use cosmwasm_std::{ to_vec};
use std::convert::TryFrom;
use std::str::FromStr;

use crate::{proto};
use crate::msg::{
    InstantiateMsg, ExecuteMsg, 
};
use cosmos_types::epochs::{
    QueryCurrentEpochResponse
};
use cosmos_types::msg::{Msg,MsgProto};
use cosmos_types::gamm::{QuerySpotPriceRequest, QuerySpotPriceResponse, QuerySwapExactAmountInRequest, QuerySwapExactAmountInResponse};
use crate::state::{LASTEST_UPDATED_EPOCH, LATEST_CONTRACT_BALANCES,
    POOLS, PoolInfo, OUR_GAMM_BONDED_EACH_POOL, TOTAL_REWARD_EACH_EPOCH, TotalGammBonded, LOCK_IDS, get_pool_at_epoch_key, get_lock_key, DistrInfos, DistrInfo, DISTR_INFOS, get_distr_info_key};
use cosmos_types::{SwapAmountInRoute, Coin};
use cosmwasm_std::{Order};
use std::collections::{HashMap};
use chain_query::{query_current_epoch_id, query_estimate_reward, query_total_lock_up};
use bigdecimal::BigDecimal;
use num_bigint::{BigInt, ParseBigIntError, Sign, ToBigInt};
use num_traits::{FromPrimitive, ToPrimitive};



use cosmos_types::{
    QueryCurrentEpochRequest
};

pub const DURATION_IN_DAY: [u8;3] = [1,7,14];

pub fn execute_update_epoch(deps: DepsMut, this_contract_address: String) -> StdResult<u16> {

    let current_epoch_id = query_current_epoch_id(deps)?;

    if current_epoch_id == LASTEST_UPDATED_EPOCH.load(deps.storage)? {
        return Err(StdError::generic_err("already updated"))
    } 

    let this_contract_balances = deps.querier.query_all_balances(this_contract_address)?;

    let balances_map : &HashMap<String, u64>;

    for balance in this_contract_balances{
        balances_map[&balance.denom] = balance.amount.into() as u64;
    }

    let pools : Vec<(u16, PoolInfo)> = POOLS
        .range_de(deps.storage, None, None, Order::Ascending)
        .map(|item| item.map(Into::into))
        .collect::<StdResult<_>>()?;

    for duration in DURATION_IN_DAY {
        for pool_item in pools {
            let pool_id = pool_item.0;
            let pool = pool_item.1;
            cal_total_reward_this_epoch(deps, &pool, this_contract_address, current_epoch_id, pool_id, duration)?;
            update_epoch_pool(deps, pool, current_epoch_id, pool_id, balances_map)
        }
    }
    
    Ok(current_epoch_id)
}

pub fn get_marker_earned_last_epoch(deps: DepsMut, pool_info: &PoolInfo, duration: u8, current_balances: &HashMap<String, u128>) -> StdResult<u64>{
    let marker_denom : &str;
    if duration == 1 {
        marker_denom = &pool_info.unique_token_markers.0;
    }
    else if duration == 7 {
        marker_denom = &pool_info.unique_token_markers.1;
    }
    else {
        marker_denom = &pool_info.unique_token_markers.2;
    }

    let current_balance = current_balances[marker_denom];

    current_balance - LATEST_CONTRACT_BALANCES.load(deps.storage, marker_denom)?


}

pub fn cal_reward_per_gamm_lockuped_last_epoch(deps: DepsMut, pool_, this_contract_address: String, ) -> StdResult<()> {
    let total_lock_up = 

}


pub fn cal_total_reward_this_epoch(deps: DepsMut, pool_info: &PoolInfo, this_contract_address: String, current_epoch_id: u16, pool_id: u16, duration_in_day: u8) -> StdResult<()> {
    let lock_id = LOCK_IDS.load(deps.storage, get_lock_key(pool_id, duration_in_day))?;
    let our_est_reward = query_estimate_reward(deps, this_contract_address, vec![lock_id], current_epoch_id as i64)?;
    let duration: core::time::Duration;
    if duration_in_day == 1 {
        duration = core::time::Duration::new(86400,0);
    }
    else if duration_in_day == 7 {
        duration = core::time::Duration::new(604800, 0);
    } else {
        duration = core::time::Duration::new(1209600, 0);
    }
    let total_lock_up: BigDecimal = BigDecimal::from_str(&query_total_lock_up(deps, pool_info.gamm_denom, duration)?).unwrap();
    let distr_infos: Vec<DistrInfo> = vec![];
    for coin in our_est_reward{
        if coin.denom == pool_info.pool_asset_denoms.0 && coin.denom == pool_info.pool_asset_denoms.1{

            // total_reward = total_lock_up / OUR_GAMM_BONDED_EACH_POOL * our_est_reward amount
            let our_est_reward_amount: BigDecimal = BigDecimal::new(BigInt::from_u128(coin.amount).unwrap(), 0);
            let total_reward = (total_lock_up / OUR_GAMM_BONDED_EACH_POOL) * our_est_reward_amount;   
            let distr_info = DistrInfo {
                denom: coin.denom.to_string(),
                total_reward: our_est_reward_amount.to_u64().unwrap(),
                reward_per_gamm_lockuped: 0f64,
            };
            distr_infos.append(vec![distr_info]);
        }
    }
    DISTR_INFOS.save(deps.storage, get_distr_info_key(pool_id, current_epoch_id, duration_in_day), &distr_infos)?;
    Ok(())
}







pub fn update_epoch_pool(deps: DepsMut, pool: PoolInfo, epoch_id: u64, pool_id: u64, contract_balances_map: &HashMap<String, u128>) {
    let total_lock_up = TotalGammBonded{
        lock_1_day: 0,
        lock_7_day: 0,
        lock_14_day: 0,
    };
    
    let unique_token_makers = pool.unique_token_makers;
    let marker_1day_denom = unique_token_makers.0;
    let gamm_1day_bonded_per_reward = GAMM_BONDED_EACH_POOL / contract_balances_map[&marker_1day_denom] as f64;

    let total_gamm_1day_bonded =  gamm_1day_bonded_per_reward * TOTAL_REWARD_EACH_EPOCH;
    let marker_7day_denom = unique_token_makers.1;
    let gamm_7day_bonded_per_reward = GAMM_BONDED_EACH_POOL / contract_balances_map[&marker_7day_denom] as f64;

    let total_gamm_7day_bonded =  gamm_7day_bonded_per_reward * TOTAL_REWARD_EACH_EPOCH;
    let marker_14day_denom = unique_token_makers.2;
    let gamm_14day_bonded_per_reward = GAMM_BONDED_EACH_POOL / contract_balances_map[&marker_14day_denom] as f64;

    let total_gamm_14day_bonded =  gamm_14day_bonded_per_reward * TOTAL_REWARD_EACH_EPOCH;
    total_lock_up.lock_1_day = total_gamm_1day_bonded as u128;
    total_lock_up.lock_7_day = total_gamm_7day_bonded as u128;
    total_lock_up.lock_14_day = total_gamm_14day_bonded as u128;

    POOLS_STATE_AT_EACH_EPOCH.save(deps.storage, get_pool_at_epoch_key(pool_id, epoch_id), &total_lock_up).unwrap();
}



pub fn execute_pool_api_service(deps: DepsMut, pool_id: u64) {
    





}