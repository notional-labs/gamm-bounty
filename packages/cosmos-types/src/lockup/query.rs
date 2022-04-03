use crate::{proto};
use core::convert::TryFrom;
use crate::msg::Msg;
use cosmwasm_std::{StdResult, StdError};
use crate::{prost_ext::MessageExt};
use prost_types::{Any, Duration};
use crate::Coin;
use std::convert::TryInto;
// use core::time::Duration;

// use prost_types::Any;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct LockedDenomRequest {
    /// Sender's address.
    pub denom: String,
    pub duration: core::time::Duration,
}

impl Msg for LockedDenomRequest {
    type Proto = proto::osmosis::lockup::LockedDenomRequest;
}

impl TryFrom<proto::osmosis::lockup::LockedDenomRequest> for LockedDenomRequest {
    type Error = StdError;

    fn try_from(proto: proto::osmosis::lockup::LockedDenomRequest) -> StdResult<LockedDenomRequest> {
        LockedDenomRequest::try_from(&proto)
    }
}

impl TryFrom<&proto::osmosis::lockup::LockedDenomRequest> for LockedDenomRequest {
    type Error = StdError;

    fn try_from(proto: &proto::osmosis::lockup::LockedDenomRequest) -> StdResult<LockedDenomRequest> {
        Ok(LockedDenomRequest {
            denom: proto.denom.parse().unwrap(),
            duration: proto.duration.clone().unwrap().try_into().unwrap(),
        })
    }
}

impl From<LockedDenomRequest> for proto::osmosis::lockup::LockedDenomRequest {
    fn from(msg: LockedDenomRequest) -> proto::osmosis::lockup::LockedDenomRequest {
        proto::osmosis::lockup::LockedDenomRequest::from(&msg)
    }
}

impl From<&LockedDenomRequest> for proto::osmosis::lockup::LockedDenomRequest {
    fn from(msg: &LockedDenomRequest) -> proto::osmosis::lockup::LockedDenomRequest {
        proto::osmosis::lockup::LockedDenomRequest {
            denom: msg.denom.to_string(),
            duration: (prost_types::Duration::from(msg.duration)).into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct LockedDenomResponse {
    pub amount: String,
}


impl Msg for LockedDenomResponse {
    type Proto = proto::osmosis::lockup::LockedDenomResponse;
}

impl TryFrom<proto::osmosis::lockup::LockedDenomResponse> for LockedDenomResponse {
    type Error = StdError;

    fn try_from(proto: proto::osmosis::lockup::LockedDenomResponse) -> StdResult<LockedDenomResponse> {
        LockedDenomResponse::try_from(&proto)
    }
}

impl TryFrom<&proto::osmosis::lockup::LockedDenomResponse> for LockedDenomResponse {
    type Error = StdError;

    fn try_from(proto: &proto::osmosis::lockup::LockedDenomResponse) -> StdResult<LockedDenomResponse> {
        Ok(LockedDenomResponse {
            amount: proto.amount.parse().unwrap(),
        })
    }
}

impl From<LockedDenomResponse> for proto::osmosis::lockup::LockedDenomResponse {
    fn from(msg: LockedDenomResponse) -> proto::osmosis::lockup::LockedDenomResponse {
        proto::osmosis::lockup::LockedDenomResponse::from(&msg)
    }
}

impl From<&LockedDenomResponse> for proto::osmosis::lockup::LockedDenomResponse {
    fn from(msg: &LockedDenomResponse) -> proto::osmosis::lockup::LockedDenomResponse {
        proto::osmosis::lockup::LockedDenomResponse {
            amount: msg.amount.to_string(),
        }
    }
}

impl LockedDenomResponse {

    pub fn into_proto(self) -> proto::osmosis::lockup::LockedDenomResponse {
        self.into()
    }

    /// Encode this type using Protocol Buffers.
    pub fn into_bytes(self) -> StdResult<Vec<u8>> {
        self.into_proto().to_bytes()
    }
}
