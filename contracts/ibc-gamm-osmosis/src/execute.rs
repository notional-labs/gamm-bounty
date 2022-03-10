use cosmwasm_std::{
    DepsMut, Response, StdError, StdResult,
};

use crate::msg::{
  SetIbcDenomForContractMsg,
};
use crate::state::{IBC_DENOM_TO_PORT_ID};

use sha2::{Sha256, Digest};
use hex::ToHex;

pub fn execute_set_ibc_denom_for_contract(
    deps: DepsMut,
    msg: SetIbcDenomForContractMsg,
) -> StdResult<Response> {
    let calculated_ibc_denom = cal_contract_ibc_denom(msg.contract_port_id.to_owned(), &msg.contract_channel_id, &msg.contract_native_denom);
    if calculated_ibc_denom != msg.ibc_denom {
        return Err(StdError::generic_err("wrong ibc denom for contract"));
    }
    IBC_DENOM_TO_PORT_ID.save(deps.storage, &msg.ibc_denom, &msg.contract_port_id)?;
    let res = Response::new()
        .add_attribute("action", "execute_set_ibc_denom_for_contract");
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