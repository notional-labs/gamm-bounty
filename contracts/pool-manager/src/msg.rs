use cosmwasm_std::{ContractResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Just needs to know the code_id of a reflect contract to spawn sub-accounts
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
// pub enum QueryMsg {
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Fund(FundMsg),
    UpdateEpoch(UpdateEpochMsg),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct FundMsg {
    pub contract_port_id: String,
    pub contract_connection_id: String, 
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct UpdateEpochMsg {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PacketMsg {
    IbcSwap{
        ibc_swap_packet: IbcSwapPacket
    },
    SpotPriceQuery{
        spot_price_query_packet: SpotPriceQueryPacket
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct IbcSwapPacket {

    pub pool_id: u64,
    
    pub in_amount: u64,
    pub in_denom: String, 

    pub out_denom: String,
    pub min_out_amount: String,

    pub to_address: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct IbcJoinExternAmountInPacket {

    pub pool_id: u64,
    
    pub in_amount: u64,
    pub in_denom: String, 
    pub share_out_min_amount: String,
    pub to_address: String,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SpotPriceQueryPacket {
    pub pool_id: u64,
    pub in_denom: String,
    pub out_denom: String,
    pub with_swap_fee: bool,
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

