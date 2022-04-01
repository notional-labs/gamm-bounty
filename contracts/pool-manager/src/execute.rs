use cosmwasm_std::{
    DepsMut, Response, StdResult, MessageInfo
};

use crate::msg::{
  FundMsg, UpdateEpochMsg
};
use crate::state::{get_contract_identifier};

pub fn execute_update_epoch(
    deps: DepsMut,
    info: MessageInfo,
    msg: FundMsg,

) -> StdResult<Response> {
    let contract_key = get_contract_identifier(msg.contract_connection_id, msg.contract_port_id);
    
    for fund in info.funds {
        REMOTE_CONTRACT_BALANCES.save(deps.storage, (contract_key.to_owned(), fund.denom), &fund.amount)?;
    }
    
    let res = Response::new()
        .add_attribute("action", "execute_fund_msg");

    Ok(res)

}

pub fn execute_fund(
    deps: DepsMut,
    info: MessageInfo,
    msg: FundMsg,

) -> StdResult<Response> {
    let contract_key = get_contract_identifier(msg.contract_connection_id, msg.contract_port_id);

    for fund in info.funds {
        REMOTE_CONTRACT_BALANCES.save(deps.storage, (contract_key.to_owned(), fund.denom), &fund.amount)?;
    }

    let res = Response::new()
        .add_attribute("action", "execute_fund_msg");

    Ok(res)
}