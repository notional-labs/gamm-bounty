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
    SetIbcDenomForContract(SetIbcDenomForContractMsg),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SetIbcDenomForContractMsg {
    pub ibc_denom: String,
    
    pub contract_channel_id: String,
    // denom of that contract token on the native chain
    pub contract_native_denom: String,
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

    IbcAddLiquidity{
        ibc_add_liquidity: IbcAddLiquidPacket
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct IbcSwapPacket {

    pub pool_id: u64,
    
    pub in_amount: String,
    pub in_denom: String, 

    pub min_out_amount: String,
    pub out_denom: String,

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

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct IbcAddLiquidPacket {
    pub pool_id: u64,

    pub share_out_amount: String,
    pub token1_amount: String,
    pub token1_denom: String,

    pub token2_amount: String,
    pub token2_denom: String,
}
