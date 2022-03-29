
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
    PacketMsg,
    IbcSwapPacket,
    SpotPriceQueryPacket, SpotPriceQueryResponse, 
};
use cosmos_types::gamm::{
    MsgSwapExactAmountIn, QueryPoolRequest, QueryPoolResponse, Pool,
};
use cosmos_types::msg::{Msg,MsgProto};
use cosmos_types::gamm::{QuerySpotPriceRequest, QuerySpotPriceResponse, QuerySwapExactAmountInRequest, QuerySwapExactAmountInResponse};
use crate::execute::execute_fund;
use crate::state::{REMOTE_CONTRACT_BALANCES, CHANNEL_ID_TO_CONN_ID, get_contract_identifier};
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
        ExecuteMsg::Fund(msg) => {
            execute_fund(deps, info, msg)
        },
        ExecuteMsg::UpdateEpoch(msg) => {
            Err(StdError::generic_err("sdaf"))
        },
    }
}

#[entry_point]
/// enforces ordering and versioing constraints
pub fn ibc_channel_open(_deps: DepsMut, _env: Env, msg: IbcChannelOpenMsg) -> StdResult<()> {
    let channel = msg.channel();

    if channel.version.as_str() != IBC_VERSION {
        return Err(StdError::generic_err(format!(
            "Must set version to `{}`",
            IBC_VERSION
        )));
    }

    if let Some(counter_version) = msg.counterparty_version() {
        if counter_version != IBC_VERSION {
            return Err(StdError::generic_err(format!(
                "Counterparty version must be `{}`",
                IBC_VERSION
            )));
        }
    }

    Ok(())
}

#[entry_point]
/// once it's established, we create the reflect contract
pub fn ibc_channel_connect(
    deps: DepsMut,
    _env: Env,
    msg: IbcChannelConnectMsg,
) -> StdResult<IbcBasicResponse> {
    let channel = msg.channel();
    let chan_id = &channel.endpoint.channel_id;
    let conn_id = &msg.channel().connection_id;

    CHANNEL_ID_TO_CONN_ID.save(deps.storage, chan_id, conn_id).unwrap();
    
    Ok(IbcBasicResponse::new()
        .add_attribute("action", "ibc_connect")
        .add_attribute("channel_id", chan_id)
        .add_event(Event::new("ibc").add_attribute("channel", "connect")))
}

#[entry_point]
/// On closed channel, we take all tokens from reflect contract to this contract.
/// We also delete the channel entry from accounts.
pub fn ibc_channel_close(
    _deps: DepsMut,
    _env: Env,
    msg: IbcChannelCloseMsg,
) -> StdResult<IbcBasicResponse> {
    let channel = msg.channel();
    // get contract address and remove lookup
    let channel_id = channel.endpoint.channel_id.as_str();

    Ok(IbcBasicResponse::new()
        .add_attribute("action", "ibc_close")
        .add_attribute("channel_id", channel_id)
    )
}

/// this is a no-op just to test how this integrates with wasmd
#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: Empty) -> StdResult<Response> {
    Ok(Response::default())
}

// this encode an error or error message into a proper acknowledgement to the recevier
fn encode_ibc_error(msg: impl Into<String>) -> Binary {
    // this cannot error, unwrap to keep the interface simple
    to_binary(&AcknowledgementMsg::<()>::Err(msg.into())).unwrap()
}

#[entry_point]
/// we look for a the proper reflect contract to relay to and send the message
/// We cannot return any meaningful response value as we do not know the response value
/// of execution. We just return ok if we dispatched, error if we failed to dispatch
pub fn ibc_packet_receive(
    deps: DepsMut,
    env: Env,
    msg: IbcPacketReceiveMsg,
) -> StdResult<IbcReceiveResponse> {
    // put this in a closure so we can convert all error responses into acknowledgements
    (|| {
        let packet = msg.packet;
        // which local channel did this packet come on
        let counterparty_contract_port_id = packet.src.port_id;
        let chann_id = packet.dest.channel_id;
        let packet_msg: PacketMsg = from_binary(&packet.data)?;
        match packet_msg {
            PacketMsg::IbcSwap{ 
                ibc_swap_packet
            } => {
                let conn_id = CHANNEL_ID_TO_CONN_ID.load(deps.storage, &chann_id)?;
                let contract_key = get_contract_identifier(conn_id, counterparty_contract_port_id);
                receive_swap(deps, env.contract.address.into(), contract_key, ibc_swap_packet)
            },
            PacketMsg::SpotPriceQuery {
                spot_price_query_packet
            } => {
                receive_spot_price_query(deps, spot_price_query_packet)
            }
        }
    })()
    .or_else(|e| {
        // we try to capture all app-level errors and convert them into
        // acknowledgement packets that contain an error code.
        let acknowledgement = encode_ibc_error(format!("invalid packet: {}", e));
        Ok(IbcReceiveResponse::new()
            .set_ack(acknowledgement)
            .add_event(Event::new("ibc").add_attribute("packet", "receive")))
    })
}

fn receive_spot_price_query(    
    deps: DepsMut,
    spot_price_query_packet: SpotPriceQueryPacket
) -> StdResult<IbcReceiveResponse> {

    let req = QuerySpotPriceRequest {
        pool_id: spot_price_query_packet.pool_id,
        token_in_denom: spot_price_query_packet.in_denom,
        token_out_denom: spot_price_query_packet.out_denom,
        with_swap_fee: spot_price_query_packet.with_swap_fee,
    }.to_any().unwrap();


    let stargate_query: QueryRequest<u8> = QueryRequest::Stargate{
        path: req.type_url,
        data: req.value.into(),
    }.into();

    let raw = to_vec(&stargate_query).map_err(|serialize_err| {
        StdError::generic_err(format!("Serializing QueryRequest: {}", serialize_err))
    })?;
  
    let spot_price_x: Vec<u8> = deps.querier.raw_query(&raw).unwrap().unwrap().into();

    let res_proto : proto::osmosis::gamm::v1beta1::QuerySpotPriceResponse;
    res_proto = prost::Message::decode(&*spot_price_x).unwrap();
    let res: QuerySpotPriceResponse = TryFrom::try_from(res_proto).unwrap();

    let spot_price_query_response = SpotPriceQueryResponse{
        spot_price: res.spot_price,
    };

    let acknowledgement = to_binary(&AcknowledgementMsg::Ok(spot_price_query_response))?;

    Ok(IbcReceiveResponse::new()
        .set_ack(acknowledgement)
    )
}


fn query_swap_exact_amount_in(deps: DepsMut, this_contract_address: String, pool_id: u64, token_in: String, routes: Vec<SwapAmountInRoute>) -> StdResult<u128> {
    let req = QuerySwapExactAmountInRequest {
        pool_id: pool_id,
        token_in: token_in.to_string(),
        sender: this_contract_address,
        routes: routes,
    }.to_any().unwrap();

    let stargate_query: QueryRequest<u8> = QueryRequest::Stargate{
        path: req.type_url,
        data: req.value.into(),
    }.into();

    let raw = to_vec(&stargate_query).map_err(|serialize_err| {
        StdError::generic_err(format!("Serializing QueryRequest: {}", serialize_err))
    })?;

    let res_x: Vec<u8> = deps.querier.raw_query(&raw).unwrap().unwrap().into();

    let res_proto : proto::osmosis::gamm::v1beta1::QuerySwapExactAmountInResponse;
    res_proto = prost::Message::decode(&*res_x).unwrap();
    let res: QuerySwapExactAmountInResponse = TryFrom::try_from(res_proto).unwrap();

    let out_amount = res.token_out_amount.parse().unwrap();

    Ok(out_amount)
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




// fn receive_join_extern(
//     deps: DepsMut,
//     this_contract_address: String,
//     contract_key: String,
//     ibc_swap_packet: IbcSwapPacket,
// ) -> {
    


// }

fn receive_swap(
    deps: DepsMut,
    this_contract_address: String,
    contract_key: String,
    ibc_swap_packet: IbcSwapPacket,
) -> StdResult<IbcReceiveResponse> {
    let funded_amount = REMOTE_CONTRACT_BALANCES.load(deps.storage, (contract_key.to_owned(), ibc_swap_packet.in_denom.to_string()))?;

    let swap_amount = ibc_swap_packet.min_out_amount.parse().unwrap();

    if funded_amount < swap_amount {
        let acknowledgement = encode_ibc_error("not enough fund");
        return Ok(IbcReceiveResponse::new()
            .set_ack(acknowledgement)
            .add_attribute("packet", "receive"));
    }

    let left_amount = funded_amount - swap_amount;
    
    REMOTE_CONTRACT_BALANCES.save(deps.storage, (contract_key, ibc_swap_packet.in_denom.to_string()), &left_amount)?;

    let route = SwapAmountInRoute{
        pool_id : ibc_swap_packet.pool_id,
        token_out_denom: ibc_swap_packet.out_denom.to_string(),
    };
    let token_in = Coin{
        denom: ibc_swap_packet.in_denom.to_string(),
        amount: ibc_swap_packet.in_amount,
    };

    let out_amount = query_swap_exact_amount_in(deps, this_contract_address.to_owned(),ibc_swap_packet.pool_id, token_in.to_string(), vec![route.clone()])?;


    if out_amount < ibc_swap_packet.min_out_amount.parse().unwrap() {
        let acknowledgement = encode_ibc_error("out amount greater than min out amount");
        return Ok(IbcReceiveResponse::new()
            .set_ack(acknowledgement)
            .add_attribute("packet", "receive"));
    };


    let swap_msg_any = MsgSwapExactAmountIn{
        sender: this_contract_address,
        routes: vec![route],
        token_in: token_in,
        token_out_min_amount: ibc_swap_packet.min_out_amount,
    }.to_any().unwrap();

    let swap_msg_stargate = CosmosMsg::Stargate{
        type_url: swap_msg_any.type_url,
        value: swap_msg_any.value.into(),
    };

    let out_coin = cosmwasm_std::Coin{
        amount: out_amount.into(),
        denom: ibc_swap_packet.out_denom,
    };

    let bank_msg = BankMsg::Send{ 
        to_address: ibc_swap_packet.to_address,
        amount: vec![out_coin],
    };

    let acknowledgement = to_binary(&AcknowledgementMsg::<()>::Ok(()))?;

    Ok(IbcReceiveResponse::new()
        .set_ack(acknowledgement)
        .add_message(swap_msg_stargate)
        .add_message(bank_msg)
        .add_attribute("action", "receive_swap"))
}

#[entry_point]
/// never should be called as we do not send packets
pub fn ibc_packet_ack(
    _deps: DepsMut,
    _env: Env,
    _msg: IbcPacketAckMsg,
) -> StdResult<IbcBasicResponse> {
    Ok(IbcBasicResponse::new().add_attribute("action", "ibc_packet_ack"))
}

#[entry_point]
/// never should be called as we do not send packets
pub fn ibc_packet_timeout(
    _deps: DepsMut,
    _env: Env,
    _msg: IbcPacketTimeoutMsg,
) -> StdResult<IbcBasicResponse> {
    Ok(IbcBasicResponse::new().add_attribute("action", "ibc_packet_timeout"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{
        mock_dependencies, mock_env, mock_ibc_channel_connect_ack,
        mock_ibc_channel_open_init, mock_ibc_packet_recv, mock_info,
        MockApi, MockQuerier, MockStorage,
    };
    use cosmwasm_std::{from_slice, OwnedDeps, IbcOrder};
    use crate::msg::{IbcSwapResponse};

    const CREATOR: &str = "creator";

        // connect will run through the entire handshake to set up a proper connect and
    // save the account (tested in detail in `proper_handshake_flow`)
    fn connect(mut deps: DepsMut, channel_id: &str) {
        let handshake_open = mock_ibc_channel_open_init(channel_id, IbcOrder::Ordered, IBC_VERSION);
        // first we try to open with a valid handshake
        ibc_channel_open(deps.branch(), mock_env(), handshake_open).unwrap();

        // then we connect (with counter-party version set)
        let handshake_connect =
            mock_ibc_channel_connect_ack(channel_id, IbcOrder::Ordered, IBC_VERSION);
        ibc_channel_connect(deps.branch(), mock_env(), handshake_connect).unwrap();
    }

    fn setup() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
        };
        let info = mock_info(CREATOR, &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        deps
    }
}


