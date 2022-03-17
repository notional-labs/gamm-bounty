use cw_storage_plus::{Map};

pub const IBC_DENOM_TO_PORT_AND_CONN_ID: Map<&str, (String, String)> = Map::new("ibc_denom_to_port_and_conn_id");
pub const CHANNEL_ID_TO_CONN_ID: Map<&str, String> = Map::new("channel_id_to_conn_id");