use crate::{proto};
use core::convert::TryFrom;
use crate::msg::Msg;
use cosmwasm_std::{StdResult, StdError};
use crate::{prost_ext::MessageExt};
use prost_types::Any;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct QueryCurrentEpochRequest {   
    pub identifier: String,
}

impl Msg for QueryCurrentEpochRequest {
    type Proto = proto::osmosis::epochs::v1beta1::QueryCurrentEpochRequest;
}

impl TryFrom<proto::osmosis::epochs::v1beta1::QueryCurrentEpochRequest> for QueryCurrentEpochRequest {
    type Error = StdError;

    fn try_from(proto: proto::osmosis::epochs::v1beta1::QueryCurrentEpochRequest) -> StdResult<QueryCurrentEpochRequest> {
        QueryCurrentEpochRequest::try_from(&proto)
    }
}

impl TryFrom<&proto::osmosis::epochs::v1beta1::QueryCurrentEpochRequest> for QueryCurrentEpochRequest {
    type Error = StdError;

    fn try_from(proto: &proto::osmosis::epochs::v1beta1::QueryCurrentEpochRequest) -> StdResult<QueryCurrentEpochRequest> {
        Ok(QueryCurrentEpochRequest {
            identifier: proto.identifier.to_owned(),
        })
    }
}

impl From<QueryCurrentEpochRequest> for proto::osmosis::epochs::v1beta1::QueryCurrentEpochRequest {
    fn from(msg: QueryCurrentEpochRequest) -> proto::osmosis::epochs::v1beta1::QueryCurrentEpochRequest {
        proto::osmosis::epochs::v1beta1::QueryCurrentEpochRequest::from(&msg)
    }
}

impl From<&QueryCurrentEpochRequest> for proto::osmosis::epochs::v1beta1::QueryCurrentEpochRequest {
    fn from(msg: &QueryCurrentEpochRequest) -> proto::osmosis::epochs::v1beta1::QueryCurrentEpochRequest {
        proto::osmosis::epochs::v1beta1::QueryCurrentEpochRequest {
            identifier: msg.identifier.to_owned(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct QueryCurrentEpochResponse {   
    pub current_epoch: u64,
}

impl Msg for QueryCurrentEpochResponse {
    type Proto = proto::osmosis::epochs::v1beta1::QueryCurrentEpochResponse;
}

impl TryFrom<proto::osmosis::epochs::v1beta1::QueryCurrentEpochResponse> for QueryCurrentEpochResponse {
    type Error = StdError;

    fn try_from(proto: proto::osmosis::epochs::v1beta1::QueryCurrentEpochResponse) -> StdResult<QueryCurrentEpochResponse> {
        QueryCurrentEpochResponse::try_from(&proto)
    }
}

impl TryFrom<&proto::osmosis::epochs::v1beta1::QueryCurrentEpochResponse> for QueryCurrentEpochResponse {
    type Error = StdError;

    fn try_from(proto: &proto::osmosis::epochs::v1beta1::QueryCurrentEpochResponse) -> StdResult<QueryCurrentEpochResponse> {
        Ok(QueryCurrentEpochResponse {
            current_epoch: proto.current_epoch,
        })
    }
}

impl From<QueryCurrentEpochResponse> for proto::osmosis::epochs::v1beta1::QueryCurrentEpochResponse {
    fn from(msg: QueryCurrentEpochResponse) -> proto::osmosis::epochs::v1beta1::QueryCurrentEpochResponse {
        proto::osmosis::epochs::v1beta1::QueryCurrentEpochResponse::from(&msg)
    }
}

impl From<&QueryCurrentEpochResponse> for proto::osmosis::epochs::v1beta1::QueryCurrentEpochResponse {
    fn from(msg: &QueryCurrentEpochResponse) -> proto::osmosis::epochs::v1beta1::QueryCurrentEpochResponse {
        proto::osmosis::epochs::v1beta1::QueryCurrentEpochResponse {
            current_epoch: msg.current_epoch,
        }
    }
}