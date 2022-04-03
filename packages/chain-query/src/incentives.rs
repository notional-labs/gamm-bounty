
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

use cosmos_types::epochs::{
    QueryCurrentEpochResponse
};
use cosmos_types::msg::{Msg,MsgProto};
use cosmos_types::gamm::{QuerySpotPriceRequest, QuerySpotPriceResponse, QuerySwapExactAmountInRequest, QuerySwapExactAmountInResponse};
use cosmos_types::{SwapAmountInRoute, Coin};


use cosmos_types::incentives::{
    RewardsEstRequest, RewardsEstResponse,
};

pub fn query_estimate_reward(deps: DepsMut, owner: String, lock_ids: Vec<u64>, end_epoch: i64) -> StdResult<Vec<Coin>> {
    let req = RewardsEstRequest{
        owner: owner,
        lock_ids: lock_ids,
        end_epoch: end_epoch,
        
    }.to_any().unwrap();

    let stargate_query: QueryRequest<u8> = QueryRequest::Stargate{
        path: req.type_url,
        data: req.value.into(),
    }.into();    

    let raw = to_vec(&stargate_query).map_err(|serialize_err| {
        StdError::generic_err(format!("Serializing QueryRequest: {}", serialize_err))
    })?;

    let res_x: Vec<u8> = deps.querier.raw_query(&raw).unwrap().unwrap().into();

    let res_proto : proto::osmosis::incentives::RewardsEstResponse;
    res_proto = prost::Message::decode(&*res_x).unwrap();
    let res: RewardsEstResponse = TryFrom::try_from(res_proto).unwrap();

    Ok(res.coins)
}