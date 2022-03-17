use cosmwasm_std::{
    entry_point, to_binary, CosmosMsg, Deps, DepsMut, Env, IbcMsg, MessageInfo, Order,
    QueryResponse, Response, StdError, StdResult, Reply, from_binary,
};
use std::convert::TryFrom;

use crate::ibc::PACKET_LIFETIME;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, IbcSwapMsg, IbcSwapPacket, IbcFundGammContractMsg,Ics20Packet, PacketMsg,
    SpotPriceQueryPacket, SpotPriceQueryMsg

};
use crate::state::{config, config_read, Config};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    // we store the reflect_id for creating accounts later
    let cfg = Config { admin: info.sender, denom: String::from("nil") };
    config(deps.storage).save(&cfg)?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::IbcSwap(msg)  => handle_ibc_swap(deps, env, info, msg),
        ExecuteMsg::SetDenom(msg) => handle_set_denom(deps, info, msg.ibc_denom),
        ExecuteMsg::IbcFundGammContract(msg) => handle_send_funds(deps, env, info, msg),
        ExecuteMsg::SpotPriceQuery(msg) => handle_spot_price_query(deps, env, info, msg),
    }
}

#[entry_point]
pub fn reply(_deps: DepsMut, _env: Env, _reply: Reply) -> StdResult<Response> {
    Ok(Response::new())
}

pub fn handle_spot_price_query(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: SpotPriceQueryMsg,
) -> StdResult<Response> {
    let query_spot_price_packet = SpotPriceQueryPacket{
        pool_id: msg.pool_id,
        in_denom: msg.in_denom,
        out_denom: msg.out_denom,
        with_swap_fee: msg.with_swap_fee,
    };

    let packet_msg = PacketMsg::SpotPriceQuery{
        spot_price_query_packet: query_spot_price_packet
    };

    // prepare message
    let send_packet_msg = IbcMsg::SendPacket {
        channel_id: msg.channel_id,
        data: to_binary(&packet_msg)?,
        timeout: env.block.time.plus_seconds(PACKET_LIFETIME).into(),
    };

    let res = Response::new()
        .add_message(send_packet_msg)
        .add_attribute("action", "handle_send_funds");
    Ok(res)
}

pub fn handle_set_denom(
    deps: DepsMut,
    info: MessageInfo,
    denom: String,
) -> StdResult<Response> {
    // auth check
    let mut cfg = config(deps.storage).load()?;
    if info.sender != cfg.admin {
        return Err(StdError::generic_err("Only admin may set new denom"));
    }
    cfg.denom = denom  ;
    config(deps.storage).save(&cfg)?;

    Ok(Response::new()
        .add_attribute("action", "handle_update_denom")
        .add_attribute("new_admin", cfg.denom))
}


pub fn handle_send_funds(
    _deps: DepsMut,
    env: Env,
    mut info: MessageInfo,
    msg: IbcFundGammContractMsg
) -> StdResult<Response> {
    // intentionally no auth check

    // require some funds
    let amount = match info.funds.pop() {
        Some(coin) => coin,
        None => {
            return Err(StdError::generic_err(
                "you must send the coins you wish to ibc transfer",
            ))
        }
    };
    // if there are any more coins, reject the message
    if !info.funds.is_empty() {
        return Err(StdError::generic_err("you can only ibc transfer one coin"));
    }

    // construct a packet to send
    let transfer_packet = Ics20Packet {
        amount: (10000000 as u32).into(),
        denom: "test".to_owned(),
        sender: info.sender.as_ref().to_owned(),
        receiver: msg.remote_gamm_contract_address,
    };

    // prepare message
    let send_packet_msg = IbcMsg::SendPacket {
        channel_id: msg.channel_id,
        data: to_binary(&transfer_packet)?,
        timeout: env.block.time.plus_seconds(PACKET_LIFETIME).into(),
    };

    let res = Response::new()
        .add_message(send_packet_msg)
        .add_attribute("action", "handle_send_funds");
    Ok(res)
}


pub fn handle_ibc_swap(
    deps: DepsMut,
    env: Env,
    mut info: MessageInfo,
    msg: IbcSwapMsg
) -> StdResult<Response> {
    let cfg = config(deps.storage).load()?;
    // require some funds
    let amount = match info.funds.pop() {
        Some(coin) => coin,
        None => {
            return Err(StdError::generic_err(
                "you must send the coins you wish to ibc transfer",
            ))
        }
    };
    
    // if there are any more coins, reject the message
    if !info.funds.is_empty() {
        return Err(StdError::generic_err("you can only ibc transfer one coin"));
    }

    let packet = IbcSwapPacket{
        pool_id: msg.pool_id.to_owned(),
        in_amount: msg.in_amount.to_owned(),
        in_denom :  msg.in_denom.to_owned(),
        out_denom : msg.out_denom.to_owned(),
        to_address: msg.to_address.to_owned(),
        min_out_amount: msg.min_out_amount.to_owned(),
    };
    let packet_msg = PacketMsg::IbcSwap{
        ibc_swap_packet: packet
    };

    let msg_swap = IbcMsg::SendPacket {
        channel_id: msg.channel_id,
        data: to_binary(&packet_msg).unwrap(),
        timeout: env.block.time.plus_seconds(PACKET_LIFETIME).into(),
    };

    let res = Response::new()
        .add_message(msg_swap)
        .add_attribute("action", "swap_amount")
        .add_attribute("packet", format!("{}", to_binary(&packet_msg).unwrap()))
        .add_attribute("in_denom", msg.in_denom);
    Ok(res)
}


#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

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
        println!("{}", data);

        let msg: IbcSwapPacket = from_binary(&data).unwrap();
    }

}
