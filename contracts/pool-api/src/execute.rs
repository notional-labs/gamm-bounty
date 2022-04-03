use cosmwasm_std::{
    DepsMut, Response, StdResult, MessageInfo
};

use crate::state::{
    TotalLockUp
};

use crate::msg::{
  UpdateEpochMsg
};

use cosmwasm_std::{
    entry_point, to_binary, Binary, CosmosMsg, DepsMut, from_binary,
    Empty, Env, Event, IbcBasicResponse, IbcChannelCloseMsg, IbcChannelConnectMsg,
    IbcChannelOpenMsg, IbcPacketAckMsg, IbcPacketReceiveMsg, IbcPacketTimeoutMsg,
    IbcReceiveResponse, MessageInfo, Response, StdError, StdResult,
    QueryRequest, BankMsg
};

use cosmwasm_std::{ to_vec};
use std::convert::TryFrom;
use std::str;

use crate::{proto};
use crate::msg::{
    InstantiateMsg, ExecuteMsg, 
};
use cosmos_types::epochs::{
    QueryCurrentEpochResponse
};
use cosmos_types::msg::{Msg,MsgProto};
use cosmos_types::gamm::{QuerySpotPriceRequest, QuerySpotPriceResponse, QuerySwapExactAmountInRequest, QuerySwapExactAmountInResponse};
use crate::state::{LASTEST_UPDATED_EPOCH, POOLS, Pool, GAMM_BONDED_EACH_POOL, TOTAL_REWARD_EACH_EPOCH, TotalGammBonded, POOLS_STATE_AT_EACH_EPOCH, get_pool_at_epoch_key};
use cosmos_types::{SwapAmountInRoute, Coin};
use crate::chain_query::{query_current_epoch_id}; 
use cosmwasm_std::{Order};
use std::collections::{HashMap};

use cosmos_types::{
    QueryCurrentEpochRequest
};


pub fn execute_update_epoch(deps: DepsMut, this_contract_address: String) -> StdResult<u64> {

    let current_epoch_id = query_current_epoch_id(deps)?;

    if current_epoch_id == LASTEST_UPDATED_EPOCH.load(deps.storage)? {
        return Err(StdError::generic_err("already updated"))
    } 

    let this_contract_balances = deps.querier.query_all_balances(this_contract_address)?;

    let balances_map : &HashMap<String, u128>;

    for balance in this_contract_balances{
        balances_map[&balance.denom] = balance.amount.into();
    }

    let pools : Vec<(u64, Pool)> = POOLS
        .range_de(deps.storage, None, None, Order::Ascending)
        .map(|item| item.map(Into::into))
        .collect::<StdResult<_>>()?;

    for pool_item in pools {
        let pool_id = pool_item.0;
        let pool = pool_item.1;
        update_epoch_pool(deps, pool, current_epoch_id, pool_id, balances_map)
    }

    Ok(current_epoch_id)
}


pub fn update_epoch_pool(deps: DepsMut, pool: Pool, epoch_id: u64, pool_id: u64, contract_balances_map: &HashMap<String, u128>) {
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