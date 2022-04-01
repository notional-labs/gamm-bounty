
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
use cosmos_types::gamm::{
    MsgSwapExactAmountIn, QueryPoolRequest, QueryPoolResponse, Pool,
};
use cosmos_types::msg::{Msg,MsgProto};
use cosmos_types::gamm::{QuerySpotPriceRequest, QuerySpotPriceResponse, QuerySwapExactAmountInRequest, QuerySwapExactAmountInResponse};
use cosmos_types::{SwapAmountInRoute, Coin};

pub const IBC_VERSION: &str = "ibc-gamm-1";
pub const RECEIVE_SWAP_ID: u64 = 1234;
pub const INIT_CALLBACK_ID: u64 = 7890;

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    // we store the reflect_id for creating accounts later
    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
    }
}

/// this is a no-op just to test how this integrates with wasmd
#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: Empty) -> StdResult<Response> {
    Ok(Response::default())
}



fn query_pool(deps: DepsMut, pool_id: u64) -> StdResult<Pool> {
    let req = QueryPoolRequest {
        pool_id: pool_id,
    }.to_any().unwrap();

    let stargate_query: QueryRequest<u8> = QueryRequest::Stargate{
        path: req.type_url,
        data: req.value.into(),
    }.into();

    let raw = to_vec(&stargate_query).map_err(|serialize_err| {
        StdError::generic_err(format!("Serializing QueryRequest: {}", serialize_err))
    })?;

    let res_x: Vec<u8> = deps.querier.raw_query(&raw).unwrap().unwrap().into();

    let res_proto : proto::osmosis::gamm::v1beta1::QueryPoolResponse;
    res_proto = prost::Message::decode(&*res_x).unwrap();
    let res: QueryPoolResponse = TryFrom::try_from(res_proto).unwrap();

    let pool: Pool = Msg::from_any(&res.pool).unwrap();
    Ok(pool)
}


