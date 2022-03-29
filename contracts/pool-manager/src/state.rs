use cw_storage_plus::{Map, Item};
use cosmwasm_std::{Uint128};
use cosmos_types::{Coin};

pub struct Config {
    epoch: u64,
    updated: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");

// a remote contract is identified by (connection id + remote port id) connection id + "/" + remote port id
// this map (remote contract identifier, denom) to amount 
pub const REMOTE_CONTRACT_BALANCES: Map<(String,String), Uint128> = Map::new("remote_contract_balances");

// this map channel id to its connection id
pub const CHANNEL_ID_TO_CONN_ID: Map<&str, String> = Map::new("channel_id_to_conn_id");

// a remote account is identified by contract identifier and remote account address (contract identifier + "/" + remote account address)
// this map (remote account identifier, denom) to amount
pub const REMOTE_ACCOUNT_BALANCES: Map<(String, String), Uint128> = Map::new("remote_address_balances");

// this map a remote account identifier to all the gamm coins it owns
pub const GAMM_VAULT: Map<&str, Vec<Coin>> = Map::new("gamm_vault");

pub fn get_remote_address_identifier(contract_identifier: String, remote_address: String) -> String {
    contract_identifier + "/" + &remote_address
}

pub fn get_contract_identifier(contract_connection_id: String, contract_port_id: String) -> String {
    contract_connection_id + "/" + &contract_port_id
}