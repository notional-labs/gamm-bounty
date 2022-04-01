use cosmwasm_std::{ContractResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Just needs to know the code_id of a reflect contract to spawn sub-accounts
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pool_id: u64,
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
// pub enum QueryMsg {
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct UpdateEpochMsg {
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BondExternInMsg {
    pub account_address: String, 
    
    pub in_denom: String, 
    pub share_out_min_amount: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct UnBondExternInMsg {
    pub account_address: String, 
    
    pub in_amount: u64,
    pub in_denom: String, 
    pub share_out_min_amount: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct WithdrawRewardMsg {
    pub account_address: String, 
    pub amount: u64,
    pub denom: String, 
    pub to_address: String,
}

/// All acknowledgements are wrapped in `ContractResult`.
/// The success value depends on the PacketMsg variant.
pub type AcknowledgementMsg<T> = ContractResult<T>;

/// This is the success response we send on ack for PacketMsg::Dispatch.
/// Just acknowledge success or error
pub type IbcSwapResponse = ();

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SpotPriceQueryResponse {
    pub spot_price: String,
}

