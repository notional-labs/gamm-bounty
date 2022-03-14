use cosmwasm_std::{
    entry_point, to_binary, Binary, CosmosMsg, DepsMut, from_binary,
    Empty, Env, Event, IbcBasicResponse, IbcChannelCloseMsg, IbcChannelConnectMsg,
    IbcChannelOpenMsg, IbcPacketAckMsg, IbcPacketReceiveMsg, IbcPacketTimeoutMsg,
    IbcReceiveResponse, MessageInfo,  Reply, Response, StdError, StdResult,
    SubMsg, SubMsgExecutionResponse, SubMsgResult, IbcMsg, QueryRequest,
};

use cosmwasm_std::{ Storage};


use crate::msg::{
    AcknowledgementMsg, 
    InstantiateMsg, ExecuteMsg, 
    // SetIbcDenomForContractMsg, 
    IbcSwapPacket, 
    IbcSwapResponse,
    SetIbcDenomForContractMsg,
};
use crate::tx::{
    MsgSwapExactAmountIn, Msg, MsgSend,
};
use crate::execute::execute_set_ibc_denom_for_contract;
use crate::state::{IBC_DENOM_TO_PORT_AND_CONN_ID,swap_queue, swap_queue_counter_read, swap_queue_counter, swap_queue_read, CHANNEL_ID_TO_CONN_ID};
use crate::{SwapAmountInRoute, Coin};

pub const IBC_VERSION: &str = "ibc-gamm-1";
pub const RECEIVE_SWAP_ID: u64 = 1234;
pub const INIT_CALLBACK_ID: u64 = 7890;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    // we store the reflect_id for creating accounts later
    swap_queue_counter(deps.storage).save(&(0)).unwrap();
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
pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> StdResult<Response> {
    match reply.result {
        SubMsgResult::Err(err) => {
            
            Ok(Response::new().set_data(encode_ibc_error(err)))
        }
        SubMsgResult::Ok(response) => {
            if reply.id < 20 {
                handle_swap_callback(deps, env.contract.address.into(), reply.id as u8,response)
            }
            // else if 20 <= reply.id && reply.id< 40 {

            // }
            else {
                Err(StdError::generic_err("invalid reply id"))
            }
        }
    }
}

pub fn handle_swap_callback(
    deps: DepsMut,
    this_contract_address: String,
    reply_id: u8,
    response: SubMsgExecutionResponse,
) -> StdResult<Response> {
    let to_address = swap_queue_read(deps.storage).load(&[reply_id])?;

    let coin_out = parse_out_coin_from_event(response.events);

    let send_msg_any = MsgSend{
        from_address: this_contract_address,
        to_address: to_address,
        amount: vec![coin_out],
    }.to_any().unwrap();

    let send_msg_stargate = CosmosMsg::Stargate{
        type_url: send_msg_any.type_url,
        value: send_msg_any.value.into(),
    };

    let send_msg = SubMsg::new(send_msg_stargate);
    Ok(Response::new()
        .add_submessage(send_msg)
        .add_attribute("action", "execute_init_callback"))
}

fn parse_out_coin_from_event(events: Vec<Event>) -> Coin {
    let out_coin = events
            .into_iter()
            .find(|e| e.ty == "token_swapped")
            .and_then(|ev| {
                ev.attributes
                    .into_iter()
                    .find(|a| a.key == "tokens_out")
            })
            .map(|a| a.value).unwrap();

    let mut seperator_index = 0;

    for (i, c) in out_coin.chars().enumerate() {
        if c.is_alphabetic() {
            seperator_index = i;
            break;
        }
    }
    let amount = out_coin[..seperator_index].to_string();
    let denom = out_coin[seperator_index..].to_string();
    Coin{
        denom: denom,
        amount: amount,
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

    CHANNEL_ID_TO_CONN_ID.save(deps.storage, chan_id, conn_id)?;
    
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
        let packet_msg: IbcSwapPacket = from_binary(&packet.data)?;
        println!("1");
        let conn_id = CHANNEL_ID_TO_CONN_ID.load(deps.storage, &chann_id)?;
        println!("1");

        receive_swap(deps, env.contract.address.into(), counterparty_contract_port_id, conn_id, packet_msg)
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

fn get_and_increment_swap_queue_counter(storage: &mut dyn Storage) -> u8 {
    let current_counter = swap_queue_counter_read(storage).load().unwrap();
    if current_counter != 19 {
        swap_queue_counter(storage).save(&(current_counter + 1)).unwrap();
    }
    else {
        swap_queue_counter(storage).save(&(0)).unwrap();
    }
    current_counter
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
    msg: IbcSwapPacket,
) -> StdResult<IbcReceiveResponse> {
    let (expected_port_id, expected_conn_id) = IBC_DENOM_TO_PORT_AND_CONN_ID.load(deps.storage, &msg.in_denom.to_owned())?;

    if !((counterparty_port_id == expected_port_id ) && ( conn_id == expected_conn_id )){
        let acknowledgement = encode_ibc_error("contract don't have permission to move fund");
        return Ok(IbcReceiveResponse::new()
            .set_ack(acknowledgement)
            .add_attribute("packet", "receive"));
    }
    println!("2");

    let route = SwapAmountInRoute{
        pool_id : msg.pool_id,
        token_out_denom: msg.out_denom.to_string(),
    };
    let token_in = Coin{
        denom: msg.in_denom.to_string(),
        amount: msg.in_amount.to_string(),
    };

    let swap_msg_any = MsgSwapExactAmountIn{
        sender: this_contract_address,
        routes: vec![route],
        token_in: token_in,
        token_out_min_amount: msg.min_out_amount,
    }.to_any().unwrap();

    let swap_msg_stargate = CosmosMsg::Stargate{
        type_url: swap_msg_any.type_url,
        value: swap_msg_any.value.into(),
    };
    
    // get current position of swap request in the swap queue
    let current_counter = get_and_increment_swap_queue_counter(deps.storage);
    let swap_submsg = SubMsg::reply_always(swap_msg_stargate, current_counter.into());
    let acknowledgement = to_binary(&AcknowledgementMsg::<IbcSwapResponse>::Ok(()))?;

    // add to_address to swap queue
    swap_queue(deps.storage).save(&[current_counter], &msg.to_address)?;

    Ok(IbcReceiveResponse::new()
        .set_ack(acknowledgement)
        .add_submessage(swap_submsg)
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
        mock_dependencies, mock_env, mock_ibc_channel_close_init, mock_ibc_channel_connect_ack,
        mock_ibc_channel_open_init, mock_ibc_channel_open_try, mock_ibc_packet_recv, mock_info,
        mock_wasmd_attr, MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR,
    };
    use cosmwasm_std::{attr, coin, coins, from_slice, BankMsg, OwnedDeps, WasmMsg, IbcOrder};

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
        let res = ibc_channel_connect(deps.branch(), mock_env(), handshake_connect).unwrap();
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
        let packet = IbcSwapPacket{
            pool_id: 9,
            in_amount: "".to_string(),
            in_denom :  "".to_string(),
            out_denom : "".to_owned(),
            to_address: "".to_string(),
            min_out_amount: "".to_owned(),
        };

        let  data = to_binary(&packet).unwrap();

        let _: IbcSwapPacket = from_binary(&data).unwrap();

        let acknowledgement = to_binary(&AcknowledgementMsg::<IbcSwapResponse>::Ok(())).unwrap();

        let ack: AcknowledgementMsg<IbcSwapResponse> =
            from_slice(&acknowledgement).unwrap();
        ack.unwrap();
    }

    #[test]
    fn handle_dispatch_packet() {
        let mut deps = setup();

        let channel_id = "channel-123";

        let ibc_denom = "ibc/1A757F169E3BB799B531736E060340FF68F37CBCEA881A147D83F84F7D87E828";

        let msg = SetIbcDenomForContractMsg{
            ibc_denom: ibc_denom.to_owned(),
            contract_channel_id: "channel-1".to_string(),
            contract_native_denom: "denom".to_string(),
        };


        let ibc_msg = IbcSwapPacket {
            pool_id: 1,
            in_amount: "10".to_owned(),
            in_denom: "test".to_owned(),
            min_out_amount: "1".to_owned(), 
            out_denom: "test".to_owned(), 
            to_address: "addr".to_owned(),
        };

        // register the channel
        connect(deps.as_mut(), channel_id);

        CHANNEL_ID_TO_CONN_ID.save(deps.as_mut().storage, channel_id, &"connection-2".to_string()).unwrap();
        IBC_DENOM_TO_PORT_AND_CONN_ID.save(deps.as_mut().storage, "test", &("their-port".to_owned(), "connection-2".to_owned())).unwrap();


        // receive a packet for an unregistered channel returns app-level error (not Result::Err)
        let msg = mock_ibc_packet_recv(channel_id, &ibc_msg).unwrap();
        let res: IbcReceiveResponse = ibc_packet_receive(deps.as_mut(), mock_env(), msg).unwrap();

        // let acknowledgement = to_binary(&AcknowledgementMsg::<IbcSwapResponse>::Ok(()))?;
        // assert app-level success
        let ack: AcknowledgementMsg<IbcSwapResponse> =
            from_slice(&res.acknowledgement).unwrap();
        ack.unwrap();

    }


}


