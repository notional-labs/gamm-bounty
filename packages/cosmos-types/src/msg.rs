//! Transaction messages

use crate::{prost_ext::MessageExt, proto, Any};
use cosmwasm_std::{StdResult,StdError};
use core::convert::TryFrom;
use core::convert::TryInto;

/// Types which impl this trait map one-to-one with a corresponding Protocol
/// Buffers type, but can assert additional invariants and/or additional
/// functionality beyond the raw proto, as well as providing a more idiomatic
/// Rust type to work with.
pub trait Msg:
    Clone + Sized + TryFrom<Self::Proto, Error = StdError> + Into<Self::Proto>
{
    /// Protocol Buffers type
    type Proto: MsgProto;

    /// Parse this message proto from [`Any`].
    fn from_any(any: &Any) -> StdResult<Self> {
        Self::Proto::from_any(any)?.try_into()
    }    

    /// Serialize this message proto as [`Any`].
    fn to_any(&self) -> StdResult<Any> {
        self.clone().into_any()
    }

    /// Convert this message proto into [`Any`].
    fn into_any(self) -> StdResult<Any> {
        self.into().to_any()
    }
}

/// Proto types which can be used as a [`Msg`].
pub trait MsgProto: Default + MessageExt + Sized {
    /// Type URL value
    const TYPE_URL: &'static str;

    /// Parse this message proto from [`Any`].
    fn from_any(any: &Any) -> StdResult<Self> {
        if any.type_url == Self::TYPE_URL {
            Ok(Self::decode(&*any.value).unwrap())
        } else {
            return Err(StdError::generic_err("can't unmarshal from any"));
        }
    }    

    /// Serialize this message proto as [`Any`].
    fn to_any(&self) -> StdResult<Any> {
        self.to_bytes().map(|bytes| Any {
            type_url: Self::TYPE_URL.to_owned(),
            value: bytes,
        })
    }
}

impl MsgProto for proto::osmosis::gamm::v1beta1::MsgSwapExactAmountIn {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.MsgSwapExactAmountIn";
}

impl MsgProto for proto::osmosis::gamm::v1beta1::MsgJoinPool {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.MsgJoinPool";
}

impl MsgProto for proto::cosmos::bank::v1beta1::MsgSend {
    const TYPE_URL: &'static str = "/cosmos.bank.v1beta1.MsgSend";
}

impl MsgProto for proto::osmosis::gamm::v1beta1::MsgJoinSwapExternAmountIn {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.MsgJoinSwapExternAmountIn";
}

impl MsgProto for proto::osmosis::gamm::v1beta1::QuerySpotPriceRequest {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.Query/SpotPrice";
}

impl MsgProto for proto::osmosis::gamm::v1beta1::QuerySpotPriceResponse {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.QuerySpotPriceReponse";
}

impl MsgProto for proto::osmosis::gamm::v1beta1::QuerySwapExactAmountInRequest {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.Query/EstimateSwapExactAmountIn";
}

impl MsgProto for proto::osmosis::gamm::v1beta1::QuerySwapExactAmountInResponse {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.QuerySwapExactAmountInResponse";
}

impl MsgProto for proto::osmosis::gamm::v1beta1::QueryPoolRequest {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.Query/Pool";
}

impl MsgProto for proto::osmosis::gamm::v1beta1::QueryPoolResponse {
    const TYPE_URL: &'static str = "/osmosis.gamm.v1beta1.QueryPoolResponse";
}

impl MsgProto for proto::osmosis::gamm::pool_model::balancer::Pool {
    const TYPE_URL: &'static str = "";
}