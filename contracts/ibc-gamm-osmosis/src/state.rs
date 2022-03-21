use cw_storage_plus::{Map};
use cosmwasm_std::{Uint128};

pub const CONTRACTS_FUND: Map<(String,String), Uint128> = Map::new("contracts_fund");
pub const CHANNEL_ID_TO_CONN_ID: Map<&str, String> = Map::new("channel_id_to_conn_id");

pub fn get_contract_key(contract_connection_id: String, contract_port_id: String) -> String {
    contract_connection_id + "/" + &contract_port_id
}