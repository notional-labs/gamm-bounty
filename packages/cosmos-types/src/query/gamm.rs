use crate::{proto};
use core::convert::TryFrom;
use crate::msg::Msg;
use cosmwasm_std::{StdResult, StdError};
use crate::{prost_ext::MessageExt,};
// use prost_types::Any;
use serde::{Serialize, Deserialize};
use crate::SwapAmountInRoute;

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


// ===================================
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct QuerySwapExactAmountInRequest {
    /// Sender's address.
    pub sender: String,

    pub pool_id: u64,

    pub token_in: String,

    pub routes: Vec<SwapAmountInRoute>,
}

impl Msg for QuerySwapExactAmountInRequest {
    type Proto = proto::osmosis::gamm::v1beta1::QuerySwapExactAmountInRequest;
}

impl TryFrom<proto::osmosis::gamm::v1beta1::QuerySwapExactAmountInRequest> for QuerySwapExactAmountInRequest {
    type Error = StdError;

    fn try_from(proto: proto::osmosis::gamm::v1beta1::QuerySwapExactAmountInRequest) -> StdResult<QuerySwapExactAmountInRequest> {
        QuerySwapExactAmountInRequest::try_from(&proto)
    }
}

impl TryFrom<&proto::osmosis::gamm::v1beta1::QuerySwapExactAmountInRequest> for QuerySwapExactAmountInRequest {
    type Error = StdError;
// khanh yeu ngan
    fn try_from(proto: &proto::osmosis::gamm::v1beta1::QuerySwapExactAmountInRequest) -> StdResult<QuerySwapExactAmountInRequest> {
        Ok(QuerySwapExactAmountInRequest {
            sender: proto.sender.parse().unwrap(),
            pool_id: proto.pool_id,
            token_in: proto.token_in.parse().unwrap(),
            routes: proto
                .routes
                .iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?
        })
    }
}

impl From<QuerySwapExactAmountInRequest> for proto::osmosis::gamm::v1beta1::QuerySwapExactAmountInRequest {
    fn from(msg: QuerySwapExactAmountInRequest) -> proto::osmosis::gamm::v1beta1::QuerySwapExactAmountInRequest {
        proto::osmosis::gamm::v1beta1::QuerySwapExactAmountInRequest::from(&msg)
    }
}

impl From<&QuerySwapExactAmountInRequest> for proto::osmosis::gamm::v1beta1::QuerySwapExactAmountInRequest {
    fn from(msg: &QuerySwapExactAmountInRequest) -> proto::osmosis::gamm::v1beta1::QuerySwapExactAmountInRequest {
        proto::osmosis::gamm::v1beta1::QuerySwapExactAmountInRequest {
            sender: msg.sender.to_string(),
            pool_id: msg.pool_id,
            token_in: msg.token_in.to_string(),
            routes: msg.routes.iter().map(Into::into).collect(),
        }
    }
}


// ==================================
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct QuerySwapExactAmountInResponse {
    pub token_out_amount: String,
}


impl Msg for QuerySwapExactAmountInResponse {
    type Proto = proto::osmosis::gamm::v1beta1::QuerySwapExactAmountInResponse;
}

impl TryFrom<proto::osmosis::gamm::v1beta1::QuerySwapExactAmountInResponse> for QuerySwapExactAmountInResponse {
    type Error = StdError;

    fn try_from(proto: proto::osmosis::gamm::v1beta1::QuerySwapExactAmountInResponse) -> StdResult<QuerySwapExactAmountInResponse> {
        QuerySwapExactAmountInResponse::try_from(&proto)
    }
}

impl TryFrom<&proto::osmosis::gamm::v1beta1::QuerySwapExactAmountInResponse> for QuerySwapExactAmountInResponse {
    type Error = StdError;

    fn try_from(proto: &proto::osmosis::gamm::v1beta1::QuerySwapExactAmountInResponse) -> StdResult<QuerySwapExactAmountInResponse> {
        Ok(QuerySwapExactAmountInResponse {
            token_out_amount: proto.token_out_amount.parse().unwrap(),
        })
    }
}

impl From<QuerySwapExactAmountInResponse> for proto::osmosis::gamm::v1beta1::QuerySwapExactAmountInResponse {
    fn from(msg: QuerySwapExactAmountInResponse) -> proto::osmosis::gamm::v1beta1::QuerySwapExactAmountInResponse {
        proto::osmosis::gamm::v1beta1::QuerySwapExactAmountInResponse::from(&msg)
    }
}

impl From<&QuerySwapExactAmountInResponse> for proto::osmosis::gamm::v1beta1::QuerySwapExactAmountInResponse {
    fn from(msg: &QuerySwapExactAmountInResponse) -> proto::osmosis::gamm::v1beta1::QuerySwapExactAmountInResponse {
        proto::osmosis::gamm::v1beta1::QuerySwapExactAmountInResponse {
            token_out_amount: msg.token_out_amount.to_string(),
        }
    }
}