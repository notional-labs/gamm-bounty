use cosmwasm_std::{
    DepsMut, Response, StdError, StdResult, QueryRequest, IbcQuery, QueryResponse, ChannelResponse
    , from_binary, from_slice, BankQuery, BalanceResponse
};
use std::option::Option;

use crate::msg::{
  SetIbcDenomForContractMsg,
};
use crate::state::{IBC_DENOM_TO_PORT_AND_CONN_ID};

use sha2::{Sha256, Digest};
use hex::ToHex;

pub fn execute_set_ibc_denom_for_contract(
    deps: DepsMut,
    msg: SetIbcDenomForContractMsg,
) -> StdResult<Response> {
    let transfer_port_id = "transfer";
    let calculated_ibc_denom = cal_contract_ibc_denom(transfer_port_id.to_owned(), &msg.contract_channel_id, &msg.contract_native_denom);
    if calculated_ibc_denom != msg.ibc_denom {
        return Err(StdError::generic_err("wrong ibc denom for contract"));
    }

    let ibc_query = IbcQuery::Channel{
        channel_id: msg.contract_channel_id,
        port_id: Option::from(transfer_port_id.to_owned()),
    }
    .into();

    let res: ChannelResponse = deps.querier.query(&ibc_query)?;

    let transfer_channel = res.channel.unwrap();

    let conn_id = transfer_channel.connection_id;

    let contract_port_id = transfer_channel.counterparty_endpoint.port_id;

    IBC_DENOM_TO_PORT_AND_CONN_ID.save(deps.storage, &msg.ibc_denom, &(contract_port_id.to_owned(), conn_id.to_owned()))?;
    let res = Response::new()
        .add_attribute("action", "execute_set_ibc_denom_for_contract")
        .add_attribute("ibc_denom_conn_id", conn_id)
        .add_attribute("ibc_denom_counterparty_port_id", contract_port_id)
        .add_attribute("ibc_denom", msg.ibc_denom);
    Ok(res)
}

pub fn cal_contract_ibc_denom(contract_port_id: String, contract_channel_id: &str, contract_native_denom: &str) -> String {
    let denom_path = contract_port_id + "/" + contract_channel_id + "/" + contract_native_denom;
    let mut hasher = Sha256::new();
    hasher.update(denom_path.as_bytes());

    let denom_path_hash = hasher.finalize();
    // let mut hash_bz = vec![0;32];

    // hash_bz[..32].clone_from_slice(&denom_path_hash.as_slice());

    let mut s = String::with_capacity(2 * denom_path_hash.len());
    denom_path_hash.write_hex(&mut s).expect("Failed to write");
    return "ibc/".to_string() + &s.to_uppercase();
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use super::*;
    use cosmwasm_std::testing::{
        mock_ibc_channel_close_init, mock_ibc_channel_connect_ack,
        mock_ibc_channel_open_init, mock_ibc_channel_open_try, mock_ibc_packet_recv,
        mock_wasmd_attr, MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR,
    };
    use cosmwasm_std::{attr, coin, coins, from_slice, BankMsg, OwnedDeps, WasmMsg};

    #[test]
    fn cal_ibc_denom_works() {
        let calculated_ibc_denom = cal_contract_ibc_denom("transfer".to_owned(), "channel-1", "test");
        
        let expected_ibc_denom = "ibc/1A757F169E3BB799B531736E060340FF68F37CBCEA881A147D83F84F7D87E828";
        assert_eq!(calculated_ibc_denom, expected_ibc_denom)
    }

    #[test]

    fn Set_ibc_denom_for_contract_msg() {
        let ibc_denom = "ibc/1A757F169E3BB799B531736E060340FF68F37CBCEA881A147D83F84F7D87E828";

        let msg = SetIbcDenomForContractMsg{
            ibc_denom: ibc_denom.to_owned(),
            contract_channel_id: "channel-1".to_string(),
            contract_native_denom: "denom".to_string(),
        };


    }




}
