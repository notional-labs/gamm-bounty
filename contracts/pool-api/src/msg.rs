use cosmwasm_std::{ContractResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Just needs to know the code_id of a reflect contract to spawn sub-accounts
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub unique_token_provider: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    InitPoolApiService(InitPoolApiServiceMsg),
    UpdateEpoch(UpdateEpochMsg),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct InitPoolApiServiceMsg {
    pub pool_id: u64,
    pub unique_token_denoms: (String, String, String),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct UpdateEpochMsg {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    LastestEpochTotalLockUp(QueryLastestEpochTotalLockUp),
    EpochTotalLockUp(QueryEpochTotalLockUp),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct QueryLastestEpochTotalLockUp {
    pub pool_id: u64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct QueryEpochTotalLockUp {
    pub pool_id: u64,
    pub epoch_id: u64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct TotalLockUpResponse {
    pub lock_1_day: u128,
    pub lock_7_day: u128,
    pub lock_14_day: u128,
}

