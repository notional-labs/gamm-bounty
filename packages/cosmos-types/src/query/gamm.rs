use crate::{proto};
use core::convert::TryFrom;
use crate::msg::Msg;
use cosmwasm_std::{StdResult, StdError};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct QuerySpotPriceRequest {
    /// Sender's address.
    pub pool_id: u64,

    pub token_in_denom: String,

    pub token_out_denom: String,

    pub with_swap_fee: bool,
}

impl Msg for QuerySpotPriceRequest {
    type Proto = proto::osmosis::gamm::v1beta1::QuerySpotPriceRequest;
}

impl TryFrom<proto::osmosis::gamm::v1beta1::QuerySpotPriceRequest> for QuerySpotPriceRequest {
    type Error = StdError;

    fn try_from(proto: proto::osmosis::gamm::v1beta1::QuerySpotPriceRequest) -> StdResult<QuerySpotPriceRequest> {
        QuerySpotPriceRequest::try_from(&proto)
    }
}

impl TryFrom<&proto::osmosis::gamm::v1beta1::QuerySpotPriceRequest> for QuerySpotPriceRequest {
    type Error = StdError;

    fn try_from(proto: &proto::osmosis::gamm::v1beta1::QuerySpotPriceRequest) -> StdResult<QuerySpotPriceRequest> {
        Ok(QuerySpotPriceRequest {
            pool_id: proto.pool_id,
            token_in_denom: proto.token_in_denom.parse().unwrap(),
            token_out_denom: proto.token_out_denom.parse().unwrap(),
            with_swap_fee: proto.with_swap_fee,
        })
    }
}

impl From<QuerySpotPriceRequest> for proto::osmosis::gamm::v1beta1::QuerySpotPriceRequest {
    fn from(msg: QuerySpotPriceRequest) -> proto::osmosis::gamm::v1beta1::QuerySpotPriceRequest {
        proto::osmosis::gamm::v1beta1::QuerySpotPriceRequest::from(&msg)
    }
}

impl From<&QuerySpotPriceRequest> for proto::osmosis::gamm::v1beta1::QuerySpotPriceRequest {
    fn from(msg: &QuerySpotPriceRequest) -> proto::osmosis::gamm::v1beta1::QuerySpotPriceRequest {
        proto::osmosis::gamm::v1beta1::QuerySpotPriceRequest {
            pool_id: msg.pool_id,
            token_in_denom: msg.token_in_denom.to_string(),
            token_out_denom: msg.token_out_denom.to_string(),
            with_swap_fee: msg.with_swap_fee,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
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