
use cosmwasm_std::{
    entry_point, to_binary, Binary, CosmosMsg, DepsMut, from_binary,
    Empty, Env, Event, IbcBasicResponse, IbcChannelCloseMsg, IbcChannelConnectMsg,
    IbcChannelOpenMsg, IbcPacketAckMsg, IbcPacketReceiveMsg, IbcPacketTimeoutMsg,
    IbcReceiveResponse, MessageInfo,  Reply, Response, StdError, StdResult,
    QueryRequest, BankMsg
};
// use cosmwasm_std::stargaze::StargateResponse;

use cosmwasm_std::{ to_vec};
use std::convert::TryFrom;
use std::str;

use crate::{proto};
use crate::msg::{
    AcknowledgementMsg, 
    InstantiateMsg, ExecuteMsg, 
    // SetIbcDenomForContractMsg, 
    // IbcSwapPacket, 
    // SpotPriceQueryPacket,
    PacketMsg,
    IbcSwapPacket,
    SpotPriceQueryPacket, SpotPriceQueryResponse,
    IbcAddLiquidPacket,
};
use cosmos_types::tx::{
    MsgSwapExactAmountIn,
};
use cosmos_types::msg::Msg;
use cosmos_types::query::{QuerySpotPriceRequest, QuerySpotPriceResponse, QuerySwapExactAmountInRequest, QuerySwapExactAmountInResponse};
use crate::execute::execute_set_ibc_denom_for_contract;
use crate::state::{IBC_DENOM_TO_PORT_AND_CONN_ID, CHANNEL_ID_TO_CONN_ID};
use cosmos_types::{SwapAmountInRoute, Coin, MsgJoinPool};

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
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::SetIbcDenomForContract(msg) => {
            execute_set_ibc_denom_for_contract(deps, msg)
        },
    }
}

#[entry_point]
pub fn reply(_deps: DepsMut, _env: Env, _reply: Reply) -> StdResult<Response> {
    Ok(Response::new())
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
                receive_swap(deps, env.contract.address.into(), counterparty_contract_port_id, conn_id, ibc_swap_packet)
            },
            PacketMsg::SpotPriceQuery {
                spot_price_query_packet
            } => {
                receive_spot_price_query(deps, spot_price_query_packet)
            }
            PacketMsg::IbcAddLiquidity{
                ibc_add_liquidity
            } => {
                let conn_id = CHANNEL_ID_TO_CONN_ID.load(deps.storage, &chann_id)?;
                receive_add_liquidity(deps, env.contract.address.into(), counterparty_contract_port_id, conn_id, ibc_add_liquidity)
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

// fn increment_swap_queue_counter(storage: &mut dyn Storage) {
//     let current_counter = swap_queue_counter_read(storage).load().unwrap();
//     if current_counter != 19 {
//         swap_queue_counter(storage).save(&(current_counter + 1)).unwrap();
//     }
//     else {
//         swap_queue_counter(storage).save(&(0)).unwrap();
//     }
// }

fn receive_add_liquidity(
    deps: DepsMut,
    this_contract_address: String,
    counterparty_port_id: String,
    conn_id: String,
    ibc_add_liquidity: IbcAddLiquidPacket,
) -> StdResult<IbcReceiveResponse>{
    let (expected_port_id, expected_conn_id) = IBC_DENOM_TO_PORT_AND_CONN_ID.load(deps.storage, &ibc_add_liquidity.token1_denom.to_owned())?;
    
    if !((counterparty_port_id == expected_port_id ) && ( conn_id == expected_conn_id )){
        let acknowledgement = encode_ibc_error("contract don't have permission to move fund");
        return Ok(IbcReceiveResponse::new()
            .set_ack(acknowledgement)
            .add_attribute("packet", "receive"));
    }

    let token_max1 = Coin{
        amount: ibc_add_liquidity.token1_amount,
        denom: ibc_add_liquidity.token1_denom
    }; 

    let token_max2 = Coin{
        amount: ibc_add_liquidity.token2_amount,
        denom: ibc_add_liquidity.token2_denom
    }; 
    let token_maxs = vec![token_max1, token_max2];

    let joinpool_msg_any = MsgJoinPool{
        sender: this_contract_address,
        pool_id: ibc_add_liquidity.pool_id,
        share_out_amount: ibc_add_liquidity.share_out_amount,
        token_in_maxs: token_maxs,
    }.to_any().unwrap();

    let swap_msg_stargate = CosmosMsg::Stargate{
        type_url: joinpool_msg_any.type_url,
        value: joinpool_msg_any.value.into(),
    };

    let acknowledgement = to_binary(&AcknowledgementMsg::<()>::Ok(()))?;

    Ok(IbcReceiveResponse::new()
        .set_ack(acknowledgement)
        .add_message(swap_msg_stargate)
        .add_attribute("action", "receive_add_liquidity"))

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

    // let spot_price = String::from_utf8(spot_price_x)?;

    // let cosmos_tx_proto : proto::ibc::applications::interchain_accounts::v1::CosmosTx;
    // cosmos_tx_proto =  prost::Message::decode(res_bin.response.into()).unwrap();

    // ==========================
    // let stargate_query = QueryRequest::Stargate{
    //     path: req.type_url,
    //     data: req.value.into(),
    // }.into();

    // let s: QuerySpotPriceResponse = deps.querier.query(&stargate_query)?;

    // ===========================

    // let test_res = QuerySpotPriceResponse {
    //     spot_price: "1.022258688245249243".to_string(),
    // };

    // let test_vec = to_vec(&test_res)?;
    // let test_string = String::from_utf8(test_vec)?;

    // return Err(StdError::generic_err(test_string));

    // ==========================
    
    let spot_price_query_response = SpotPriceQueryResponse{
        spot_price: res.spot_price,
    };

    let acknowledgement = to_binary(&AcknowledgementMsg::Ok(spot_price_query_response))?;

    // add to_address to swap queue

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
// fn get_swap_queue_counter(storage: &dyn Storage) -> u8 {
//     swap_queue_counter_read(storage).load().unwrap()
// }

// processes PacketMsg::Dispatch variant
fn receive_swap(
    deps: DepsMut,
    this_contract_address: String,
    counterparty_port_id: String,
    conn_id: String,
    ibc_swap_packet: IbcSwapPacket,
) -> StdResult<IbcReceiveResponse> {
    let (expected_port_id, expected_conn_id) = IBC_DENOM_TO_PORT_AND_CONN_ID.load(deps.storage, &ibc_swap_packet.in_denom.to_owned())?;

    if !((counterparty_port_id == expected_port_id ) && ( conn_id == expected_conn_id )){
        let acknowledgement = encode_ibc_error("contract don't have permission to move fund");
        return Ok(IbcReceiveResponse::new()
            .set_ack(acknowledgement)
            .add_attribute("packet", "receive"));
    }

    let route = SwapAmountInRoute{
        pool_id : ibc_swap_packet.pool_id,
        token_out_denom: ibc_swap_packet.out_denom.to_string(),
    };
    let token_in = Coin{
        denom: ibc_swap_packet.in_denom.to_string(),
        amount: ibc_swap_packet.in_amount.to_string(),
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
    // let msg: CosmosMsg = bank_msg.clone().into();

    // let cosmos_msgs = [CosmosMsg::Bank(bank_msg.clone())];

    // get current position of swap request in the swap queue
    // let current_counter = get_and_increment_swap_queue_counter(deps.storage);
    // let swap_submsg = SubMsg::reply_always(swap_msg_stargate, current_counter.into());
    let acknowledgement = to_binary(&AcknowledgementMsg::<()>::Ok(()))?;

    // add to_address to swap queue
    // swap_queue(deps.storage).save(&[current_counter], &ibc_swap_packet.to_address)?;

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


    #[test]

    fn marshal_work() {


        let ibc_swap_packet = IbcSwapPacket{
            pool_id: 9,
            in_amount: "".to_string(),
            in_denom :  "".to_string(),
            out_denom : "".to_owned(),
            to_address: "".to_string(),
            min_out_amount: "".to_owned(),
        };

        let packet = PacketMsg::IbcSwap {
            ibc_swap_packet: ibc_swap_packet,
        };

        let  data = to_binary(&packet).unwrap();

        let _: PacketMsg = from_binary(&data).unwrap();

        let acknowledgement = to_binary(&AcknowledgementMsg::<IbcSwapResponse>::Ok(())).unwrap();

        let ack: AcknowledgementMsg<IbcSwapResponse> =
            from_slice(&acknowledgement).unwrap();
        ack.unwrap();
    }

    #[test]
    fn handle_dispatch_packet() {
        let mut deps = setup();

        let channel_id = "channel-123";

        let ibc_swap_packet = IbcSwapPacket {
            pool_id: 1,
            in_amount: "10".to_owned(),
            in_denom: "test".to_owned(),
            min_out_amount: "1".to_owned(), 
            out_denom: "test".to_owned(), 
            to_address: "addr".to_owned(),
        };

        let packet = PacketMsg::IbcSwap {
            ibc_swap_packet: ibc_swap_packet,
        };


        // register the channel
        connect(deps.as_mut(), channel_id);

        CHANNEL_ID_TO_CONN_ID.save(deps.as_mut().storage, channel_id, &"connection-2".to_string()).unwrap();
        IBC_DENOM_TO_PORT_AND_CONN_ID.save(deps.as_mut().storage, "test", &("their-port".to_owned(), "connection-2".to_owned())).unwrap();


        // receive a packet for an unregistered channel returns app-level error (not Result::Err)
        let msg = mock_ibc_packet_recv(channel_id, &packet).unwrap();
        let res: IbcReceiveResponse = ibc_packet_receive(deps.as_mut(), mock_env(), msg).unwrap();

        // let acknowledgement = to_binary(&AcknowledgementMsg::<IbcSwapResponse>::Ok(()))?;
        // assert app-level success
        let ack: AcknowledgementMsg<IbcSwapResponse> =
            from_slice(&res.acknowledgement).unwrap();
        ack.unwrap();

    }


}


