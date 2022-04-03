use crate::{proto};
use core::convert::TryFrom;
use crate::msg::Msg;
use cosmwasm_std::{StdResult, StdError};
use crate::{prost_ext::MessageExt};
use prost_types::Any;

// use prost_types::Any;
use serde::{Serialize, Deserialize};
use crate::SwapAmountInRoute;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct RewardsEstRequest {
    /// Sender's address.
    pub owner: String,
    pub lock_ids: Vec<u64>,
    pub end_epoch: i64,
}

impl Msg for RewardsEstRequest {
    type Proto = proto::osmosis::gamm::v1beta1::RewardsEstRequest;
}

impl TryFrom<proto::osmosis::gamm::v1beta1::RewardsEstRequest> for RewardsEstRequest {
    type Error = StdError;

    fn try_from(proto: proto::osmosis::gamm::v1beta1::RewardsEstRequest) -> StdResult<RewardsEstRequest> {
        RewardsEstRequest::try_from(&proto)
    }
}

impl TryFrom<&proto::osmosis::gamm::v1beta1::RewardsEstRequest> for RewardsEstRequest {
    type Error = StdError;

    fn try_from(proto: &proto::osmosis::gamm::v1beta1::RewardsEstRequest) -> StdResult<RewardsEstRequest> {
        Ok(RewardsEstRequest {
            owner: proto.owner.parse().unwrap(),
            
        })
    }
}

impl From<RewardsEstRequest> for proto::osmosis::gamm::v1beta1::RewardsEstRequest {
    fn from(msg: RewardsEstRequest) -> proto::osmosis::gamm::v1beta1::RewardsEstRequest {
        proto::osmosis::gamm::v1beta1::RewardsEstRequest::from(&msg)
    }
}

impl From<&RewardsEstRequest> for proto::osmosis::gamm::v1beta1::RewardsEstRequest {
    fn from(msg: &RewardsEstRequest) -> proto::osmosis::gamm::v1beta1::RewardsEstRequest {
        proto::osmosis::gamm::v1beta1::RewardsEstRequest {
            pool_id: msg.pool_id,
            token_in_denom: msg.token_in_denom.to_string(),
            token_out_denom: msg.token_out_denom.to_string(),
            with_swap_fee: msg.with_swap_fee,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct QuerySpotPriceResponse {
    pub spot_price: String,
}


impl Msg for QuerySpotPriceResponse {
    type Proto = proto::osmosis::gamm::v1beta1::QuerySpotPriceResponse;
}

impl TryFrom<proto::osmosis::gamm::v1beta1::QuerySpotPriceResponse> for QuerySpotPriceResponse {
    type Error = StdError;

    fn try_from(proto: proto::osmosis::gamm::v1beta1::QuerySpotPriceResponse) -> StdResult<QuerySpotPriceResponse> {
        QuerySpotPriceResponse::try_from(&proto)
    }
}

impl TryFrom<&proto::osmosis::gamm::v1beta1::QuerySpotPriceResponse> for QuerySpotPriceResponse {
    type Error = StdError;

    fn try_from(proto: &proto::osmosis::gamm::v1beta1::QuerySpotPriceResponse) -> StdResult<QuerySpotPriceResponse> {
        Ok(QuerySpotPriceResponse {
            spot_price: proto.spot_price.parse().unwrap(),
        })
    }
}

impl From<QuerySpotPriceResponse> for proto::osmosis::gamm::v1beta1::QuerySpotPriceResponse {
    fn from(msg: QuerySpotPriceResponse) -> proto::osmosis::gamm::v1beta1::QuerySpotPriceResponse {
        proto::osmosis::gamm::v1beta1::QuerySpotPriceResponse::from(&msg)
    }
}

impl From<&QuerySpotPriceResponse> for proto::osmosis::gamm::v1beta1::QuerySpotPriceResponse {
    fn from(msg: &QuerySpotPriceResponse) -> proto::osmosis::gamm::v1beta1::QuerySpotPriceResponse {
        proto::osmosis::gamm::v1beta1::QuerySpotPriceResponse {
            spot_price: msg.spot_price.to_string(),
        }
    }
}

impl QuerySpotPriceResponse {
    pub fn new<I>(
        spot_price: I,
    ) -> Self
    where
        I: ToString,
    {
        QuerySpotPriceResponse { 
            spot_price: spot_price.to_string(),
        }
    }

    pub fn into_proto(self) -> proto::osmosis::gamm::v1beta1::QuerySpotPriceResponse {
        self.into()
    }

    /// Encode this type using Protocol Buffers.
    pub fn into_bytes(self) -> StdResult<Vec<u8>> {
        self.into_proto().to_bytes()
    }
}
