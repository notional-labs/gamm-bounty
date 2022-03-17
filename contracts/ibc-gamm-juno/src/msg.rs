use cosmwasm_std::{Coin, CosmosMsg, Empty, Timestamp, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{ContractResult};

/// This needs no info. Owner of the contract is whoever signed the InstantiateMsg.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetDenom(SetIbcDenomMsg),
    IbcSwap(IbcSwapMsg),
    IbcFundGammContract(IbcFundGammContractMsg),
    SpotPriceQuery(SpotPriceQueryMsg),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SetIbcDenomMsg {
    pub ibc_denom: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct IbcSwapMsg {
    pub channel_id: String,
    
    pub pool_id:u64,

    pub in_denom: String,
    pub in_amount: String,
    
    pub out_denom: String,
    pub min_out_amount: String,

    pub to_address: String,
}

/// This is the message we accept via Receive
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IbcFundGammContractMsg {
    /// The local channel to send the packets on
    pub channel_id: String,
    /// The remote address to send to.
    /// Don't use HumanAddress as this will likely have a different Bech32 prefix than we use
    /// and cannot be validated locally
    pub remote_gamm_contract_address: String,
}

/// This is the message we accept via Receive
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SpotPriceQueryMsg {
    /// The local channel to send the packets on
    pub channel_id: String,
    /// The remote address to send to.
    /// Don't use HumanAddress as this will likely have a different Bech32 prefix than we use
    /// and cannot be validated locally
    pub pool_id: u64,
    pub in_denom: String,
    pub out_denom: String,
    pub with_swap_fee: bool,
}

//custom ibc packet
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
// pub enum PacketMsg {
//     IbcSwapPacket(IbcSwapPacket),
//     Ics20Packet(Ics20Packet),
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PacketMsg {
    IbcSwap{
        ibc_swap_packet: IbcSwapPacket
    },
    SpotPriceQuery{
        spot_price_query_packet: SpotPriceQueryPacket
    },
    Ics20{
        ics20_packet: Ics20Packet
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Ics20Packet {
    /// amount of tokens to transfer is encoded as a string, but limited to u64 max
    pub amount: Uint128,
    /// the token denomination to be transferred
    pub denom: String,
    /// the recipient address on the destination chain
    pub receiver: String,
    /// the sender address
    pub sender: String,
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