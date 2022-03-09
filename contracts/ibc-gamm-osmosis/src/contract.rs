use cosmwasm_std::{
    entry_point, from_slice, to_binary, wasm_execute, BankMsg, Binary, CosmosMsg, Deps, DepsMut,
    Empty, Env, Event, IbcBasicResponse, IbcChannelCloseMsg, IbcChannelConnectMsg,
    IbcChannelOpenMsg, IbcOrder, IbcPacketAckMsg, IbcPacketReceiveMsg, IbcPacketTimeoutMsg,
    IbcReceiveResponse, MessageInfo, Order, QueryResponse, Reply, Response, StdError, StdResult,
    SubMsg, SubMsgExecutionResponse, SubMsgResult, WasmMsg,
};

use cosmwasm_std::{ Storage};

use crate::msg::{
    AcknowledgementMsg, 
    InstantiateMsg, PacketMsg, ExecuteMsg, SetIBCDenomForContractMsg, IbcSwap, IbcSwapResponse,
};
use crate::tx::{
    MsgSwapExactAmountIn, Msg,
};
use crate::execute::execute_set_ibc_denom_for_contract;
use crate::state::{IBC_DENOM_TO_PORT_ID,swap_queue, swap_queue_counter_read, swap_queue_counter};
use crate::{SwapAmountInRoute, Coin};

pub const IBC_VERSION: &str = "ibc-gamm-v1";
pub const RECEIVE_SWAP_ID: u64 = 1234;
pub const INIT_CALLBACK_ID: u64 = 7890;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // we store the reflect_id for creating accounts later
    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::SetIBCDenomForContractMsg(msg) => {
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
                handle_swap_callback(deps, env.contract.address.into(), response)
            }
            // else if 20 <= reply.id && reply.id< 40 {

            // }
            else {
                Err(StdError::generic_err("invalid reply id"))
            }


        }
        _ => Err(StdError::generic_err("invalid reply id or result")),

    }
}


pub fn handle_swap_callback(
    deps: DepsMut,
    this_contract_address: String,
    response: SubMsgExecutionResponse,
) -> StdResult<Response> {



    Ok(Response::new().add_attribute("action", "execute_init_callback"))
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

    Ok(IbcBasicResponse::new()
        .add_attribute("action", "ibc_connect")
        .add_attribute("channel_id", chan_id)
        .add_event(Event::new("ibc").add_attribute("channel", "connect")))
}

#[entry_point]
/// On closed channel, we take all tokens from reflect contract to this contract.
/// We also delete the channel entry from accounts.
pub fn ibc_channel_close(
    deps: DepsMut,
    env: Env,
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
        let msg: PacketMsg = from_slice(&packet.data)?;
        match msg {
            PacketMsg::IbcSwap(msg) => receive_swap(deps, env.contract.address.into(), counterparty_contract_port_id, msg),
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

fn increment_swap_queue_counter(storage: &mut dyn Storage) {
    let current_counter = swap_queue_counter_read(storage).load().unwrap();
    if current_counter != 19 {
        swap_queue_counter(storage).save(&(current_counter + 1));
    }
    else {
        swap_queue_counter(storage).save(&(0));
    }
}

fn get_swap_queue_counter(storage: &dyn Storage) -> u8 {
    swap_queue_counter_read(storage).load().unwrap()
}


// processes PacketMsg::Dispatch variant
fn receive_swap(
    deps: DepsMut,
    this_contract_address: String,
    counterparty_contract_port_id: String,
    msg: IbcSwap,
) -> StdResult<IbcReceiveResponse> {

    let port_id_of_in_denom = IBC_DENOM_TO_PORT_ID.load(deps.storage, &msg.in_denom)?;

    if counterparty_contract_port_id != port_id_of_in_denom {
        let acknowledgement = encode_ibc_error("contract don't have permission to move fund");
        return Ok(IbcReceiveResponse::new()
            .set_ack(acknowledgement)
            .add_attribute("packet", "receive"));
    }


    let route = SwapAmountInRoute{
        pool_id : msg.pool_id,
        token_out_denom: msg.out_denom.to_string(),
    };
    let token_in = Coin{
        denom: msg.in_denom.to_string(),
        amount: msg.in_amount.to_string(),
    };

    let swap_msg = MsgSwapExactAmountIn{
        sender: this_contract_address,
        routes: vec![route],
        token_in: token_in,
        token_out_min_amount: msg.min_out_amount,
    }.to_any().unwrap();

    let swap_msg = CosmosMsg::Stargate{
        type_url: swap_msg.type_url,
        value: swap_msg.value.into(),
    };
    
    let msg = SubMsg::reply_always(swap_msg, get_swap_queue_counter(deps.storage).into());
    let acknowledgement = to_binary(&AcknowledgementMsg::<IbcSwapResponse>::Ok(()))?;

    swap_queue()



    Ok(IbcReceiveResponse::new()
        .set_ack(acknowledgement)
        .add_submessage(msg)
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
}use cosmwasm_std::{ Storage};
