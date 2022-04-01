use cosmwasm_std::{
    DepsMut, Response, StdResult, MessageInfo
};

use crate::state::{
    EPOCHSTATES
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
    AcknowledgementMsg, 
    InstantiateMsg, ExecuteMsg, 
    SpotPriceQueryResponse, 
};
use cosmos_types::epochs::{
    QueryCurrentEpochResponse
};
use cosmos_types::msg::{Msg,MsgProto};
use cosmos_types::gamm::{QuerySpotPriceRequest, QuerySpotPriceResponse, QuerySwapExactAmountInRequest, QuerySwapExactAmountInResponse};
use crate::state::{LASTEST_UPDATED_EPOCH, CONFIG, CONTRACT_BALANCES};
use cosmos_types::{SwapAmountInRoute, Coin};
use crate::chain_query::{query_current_epoch_id}; 


use cosmos_types::{
    QueryCurrentEpochRequest
};

pub fn execute_update_epoch(deps: DepsMut, this_contract_address: String) -> StdResult<Response> {
    let current_epoch = query_current_epoch_id(deps)?;

    let lastest_updated_epoch = LASTEST_UPDATED_EPOCH.load(deps.storage)?;
    if current_epoch != lastest_updated_epoch {
        let config = CONFIG.load(deps.storage)?;
        let incentivized_reward_denoms = config.incentivized_reward_denoms;
        let gamm_denom = config.gamm_denom;

        let gamm_balance = deps.querier.query_balance(this_contract_address, gamm_denom)?.amount; 
        
        for denom in incentivized_reward_denoms {
            let current_balance = deps.querier.query_balance(this_contract_address, denom)?;
            let lastest_updated_balance = CONTRACT_BALANCES.load(deps.storage, denom)?;


            let diff = 
        }

    };



    let res = Response::new()
        .add_attribute("action", "update_epoch_msg");

    Ok(res)
}




