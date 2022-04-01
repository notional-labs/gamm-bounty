

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
use crate::state::{LASTEST_UPDATED_EPOCH};
use cosmos_types::{SwapAmountInRoute, Coin};


use cosmos_types::{
    QueryCurrentEpochRequest
};

pub fn query_current_epoch_id(deps: DepsMut) -> StdResult<u64> {
    let req = QueryCurrentEpochRequest{
        identifier: "day".to_owned(),
    }.to_any().unwrap();

    let stargate_query: QueryRequest<u8> = QueryRequest::Stargate{
        path: req.type_url,
        data: req.value.into(),
    }.into();    

    let raw = to_vec(&stargate_query).map_err(|serialize_err| {
        StdError::generic_err(format!("Serializing QueryRequest: {}", serialize_err))
    })?;

    let res_x: Vec<u8> = deps.querier.raw_query(&raw).unwrap().unwrap().into();

    let res_proto : proto::osmosis::epochs::v1beta1::QueryCurrentEpochResponse;
    res_proto = prost::Message::decode(&*res_x).unwrap();
    let res: QueryCurrentEpochResponse = TryFrom::try_from(res_proto).unwrap();

    Ok(res.current_epoch)
}