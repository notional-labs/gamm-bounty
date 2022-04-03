use crate::{proto};
use core::convert::TryFrom;
use crate::msg::Msg;
use cosmwasm_std::{StdResult, StdError};
use crate::{prost_ext::MessageExt};
use prost_types::Any;
use crate::Coin;

// use prost_types::Any;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct RewardsEstRequest {
    /// Sender's address.
    pub owner: String,
    pub lock_ids: Vec<u64>,
    pub end_epoch: i64,
}

impl Msg for RewardsEstRequest {
    type Proto = proto::osmosis::incentives::RewardsEstRequest;
}

impl TryFrom<proto::osmosis::incentives::RewardsEstRequest> for RewardsEstRequest {
    type Error = StdError;

    fn try_from(proto: proto::osmosis::incentives::RewardsEstRequest) -> StdResult<RewardsEstRequest> {
        RewardsEstRequest::try_from(&proto)
    }
}

impl TryFrom<&proto::osmosis::incentives::RewardsEstRequest> for RewardsEstRequest {
    type Error = StdError;

    fn try_from(proto: &proto::osmosis::incentives::RewardsEstRequest) -> StdResult<RewardsEstRequest> {
        Ok(RewardsEstRequest {
            owner: proto.owner.parse().unwrap(),
            lock_ids: proto.lock_ids.clone(),
            end_epoch: proto.end_epoch,
        })
    }
}

impl From<RewardsEstRequest> for proto::osmosis::incentives::RewardsEstRequest {
    fn from(msg: RewardsEstRequest) -> proto::osmosis::incentives::RewardsEstRequest {
        proto::osmosis::incentives::RewardsEstRequest::from(&msg)
    }
}

impl From<&RewardsEstRequest> for proto::osmosis::incentives::RewardsEstRequest {
    fn from(msg: &RewardsEstRequest) -> proto::osmosis::incentives::RewardsEstRequest {
        proto::osmosis::incentives::RewardsEstRequest {
            owner: msg.owner.to_string(),
            lock_ids: msg.lock_ids.clone(),
            end_epoch: msg.end_epoch,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct RewardsEstResponse {
    pub coins: Vec<Coin>,
}


impl Msg for RewardsEstResponse {
    type Proto = proto::osmosis::incentives::RewardsEstResponse;
}

impl TryFrom<proto::osmosis::incentives::RewardsEstResponse> for RewardsEstResponse {
    type Error = StdError;

    fn try_from(proto: proto::osmosis::incentives::RewardsEstResponse) -> StdResult<RewardsEstResponse> {
        RewardsEstResponse::try_from(&proto)
    }
}

impl TryFrom<&proto::osmosis::incentives::RewardsEstResponse> for RewardsEstResponse {
    type Error = StdError;

    fn try_from(proto: &proto::osmosis::incentives::RewardsEstResponse) -> StdResult<RewardsEstResponse> {
        Ok(RewardsEstResponse {
            coins: proto
                .coins
                .iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<RewardsEstResponse> for proto::osmosis::incentives::RewardsEstResponse {
    fn from(msg: RewardsEstResponse) -> proto::osmosis::incentives::RewardsEstResponse {
        proto::osmosis::incentives::RewardsEstResponse::from(&msg)
    }
}

impl From<&RewardsEstResponse> for proto::osmosis::incentives::RewardsEstResponse {
    fn from(msg: &RewardsEstResponse) -> proto::osmosis::incentives::RewardsEstResponse {
        proto::osmosis::incentives::RewardsEstResponse {
            coins: msg.coins.iter().map(Into::into).collect(),
        }
    }
}

impl RewardsEstResponse {

    pub fn into_proto(self) -> proto::osmosis::incentives::RewardsEstResponse {
        self.into()
    }

    /// Encode this type using Protocol Buffers.
    pub fn into_bytes(self) -> StdResult<Vec<u8>> {
        self.into_proto().to_bytes()
    }
}
