use cosmwasm_std::{
    DepsMut, Response, StdResult, MessageInfo
};

use crate::msg::{
  FundMsg,
};
use crate::state::{CONTRACTS_FUND, get_contract_key};

pub fn execute_fund(
    deps: DepsMut,
    info: MessageInfo,
    msg: FundMsg,

) -> StdResult<Response> {
    let contract_key = get_contract_key(msg.contract_connection_id, msg.contract_port_id);

    for fund in info.funds {
        CONTRACTS_FUND.save(deps.storage, (contract_key.to_owned(), fund.denom), &fund.amount)?;
    }

    let res = Response::new()
        .add_attribute("action", "execute_fund_msg");

    Ok(res)
}