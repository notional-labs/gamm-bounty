use crate::{proto, Coin};
use cosmwasm_std::{StdResult, StdError};
use prost_types::Timestamp;

use std::convert::TryInto;
use core::convert::TryFrom;
use crate::msg::Msg;

/// Coin defines a token with a denomination and an amount.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct SwapAmountInRoute {
    /// Denomination
    pub pool_id: u64,

    /// Amount
    pub token_out_denom: String,
}

impl TryFrom<proto::osmosis::gamm::v1beta1::SwapAmountInRoute> for SwapAmountInRoute {
    type Error = StdError;

    fn try_from(proto: proto::osmosis::gamm::v1beta1::SwapAmountInRoute) -> StdResult<SwapAmountInRoute> {
        SwapAmountInRoute::try_from(&proto)
    }
}

impl TryFrom<&proto::osmosis::gamm::v1beta1::SwapAmountInRoute> for SwapAmountInRoute {
    type Error = StdError;

    fn try_from(proto: &proto::osmosis::gamm::v1beta1::SwapAmountInRoute) -> StdResult<SwapAmountInRoute> {
        Ok(SwapAmountInRoute {
            pool_id: proto.pool_id,
            token_out_denom: proto.token_out_denom.parse().unwrap(),
        })
    }
}

impl From<SwapAmountInRoute> for proto::osmosis::gamm::v1beta1::SwapAmountInRoute {

    fn from(swap_amount_in_route: SwapAmountInRoute) -> proto::osmosis::gamm::v1beta1::SwapAmountInRoute {
        proto::osmosis::gamm::v1beta1::SwapAmountInRoute::from(&swap_amount_in_route)
    }
}

impl From<&SwapAmountInRoute> for proto::osmosis::gamm::v1beta1::SwapAmountInRoute {
    fn from(swap_amount_in_route: &SwapAmountInRoute) -> proto::osmosis::gamm::v1beta1::SwapAmountInRoute {
        proto::osmosis::gamm::v1beta1::SwapAmountInRoute {
            pool_id: swap_amount_in_route.pool_id,
            token_out_denom: swap_amount_in_route.token_out_denom.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PoolAsset {
    pub token: Coin,
    pub weight: f64,
}

impl TryFrom<proto::osmosis::gamm::v1beta1::PoolAsset> for PoolAsset {
    type Error = StdError;

    fn try_from(proto: proto::osmosis::gamm::v1beta1::PoolAsset) -> StdResult<PoolAsset> {
        PoolAsset::try_from(&proto)
    }
}

impl TryFrom<&proto::osmosis::gamm::v1beta1::PoolAsset> for PoolAsset {
    type Error = StdError;

    fn try_from(proto: &proto::osmosis::gamm::v1beta1::PoolAsset) -> StdResult<PoolAsset> {
        Ok(PoolAsset {
            token: proto.token.as_ref().unwrap().try_into().unwrap(),
            weight: proto.weight.parse().unwrap(),
        })
    }
}

impl From<PoolAsset> for proto::osmosis::gamm::v1beta1::PoolAsset {

    fn from(swap_amount_in_route: PoolAsset) -> proto::osmosis::gamm::v1beta1::PoolAsset {
        proto::osmosis::gamm::v1beta1::PoolAsset::from(&swap_amount_in_route)
    }
}

impl From<&PoolAsset> for proto::osmosis::gamm::v1beta1::PoolAsset {
    fn from(swap_amount_in_route: &PoolAsset) -> proto::osmosis::gamm::v1beta1::PoolAsset {
        proto::osmosis::gamm::v1beta1::PoolAsset {
            token: (proto::cosmos::base::v1beta1::Coin::from(&swap_amount_in_route.token)).into(),
            weight: swap_amount_in_route.weight.to_string(),
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct Pool {
    pub swap_fee: f64,
    pub exit_fee: f64,
    pub total_shares: Coin,
    pub pool_assets: Vec<PoolAsset>,
    pub total_weight: f64,
}

impl Msg for Pool {
    type Proto = proto::osmosis::gamm::pool_model::balancer::Pool;
}

impl TryFrom<proto::osmosis::gamm::pool_model::balancer::Pool> for Pool {
    type Error = StdError;

    fn try_from(proto: proto::osmosis::gamm::pool_model::balancer::Pool) -> StdResult<Pool> {
        Pool::try_from(&proto)
    }
}

impl TryFrom<&proto::osmosis::gamm::pool_model::balancer::Pool> for Pool {
    type Error = StdError;

    fn try_from(proto: &proto::osmosis::gamm::pool_model::balancer::Pool) -> StdResult<Pool> {
        Ok(Pool {
            swap_fee: proto.pool_params.as_ref().unwrap().swap_fee.parse().unwrap(),
            exit_fee: proto.pool_params.as_ref().unwrap().exit_fee.parse().unwrap(),
            total_shares: proto.total_shares.as_ref().unwrap().try_into().unwrap(),
            pool_assets: proto.pool_assets.iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
            total_weight: proto.total_weight.parse().unwrap(),
        })
    }
}

impl From<Pool> for proto::osmosis::gamm::pool_model::balancer::Pool {

    fn from(pool: Pool) -> proto::osmosis::gamm::pool_model::balancer::Pool {
        proto::osmosis::gamm::pool_model::balancer::Pool::from(&pool)
    }
}

impl From<&Pool> for proto::osmosis::gamm::pool_model::balancer::Pool {
    fn from(_pool: &Pool) -> proto::osmosis::gamm::pool_model::balancer::Pool {
        proto::osmosis::gamm::pool_model::balancer::Pool {
            address: "".to_owned(),
            id: 0,
            pool_params: None,
            future_pool_governor: "".to_owned(),
            total_shares: None,
            pool_assets: vec![],
            total_weight: "".to_owned(),
        }
    }
}

pub fn SolveConstantFunctionInvariant(
    token_balance_fixed_before: f64,
    token_balance_fixed_after: f64,
    token_weight_fixed: f64,
    token_balance_unknown_before: f64,
    token_weight_unknown: f64,
) -> f64 {
    let weight_ratio = token_weight_fixed / token_weight_unknown;

    let y = token_balance_fixed_before / token_balance_fixed_after;

    let foo = y.powf(weight_ratio);

    let mutiplier = 1f64 - foo;

    token_balance_unknown_before * mutiplier
}

pub fn FeeRatio(
    normalized_weight: f64,
    swap_fee: f64
) -> f64 {
    let zar = (1f64 - normalized_weight) * swap_fee;
    1f64 - zar
}

pub fn CalcPoolOutGivenSingleIn(
    token_balance_in: f64,
    normalized_token_weight_in: f64,
    pool_supply: f64,
    token_amount_in: f64,
    swap_fee: f64,
) -> f64 {
    let token_amount_in_after_fee = token_amount_in * FeeRatio(normalized_token_weight_in, swap_fee);
    - SolveConstantFunctionInvariant(token_balance_in + token_amount_in_after_fee, token_balance_in, normalized_token_weight_in, pool_supply,1f64)     
}


impl Pool {
    fn GetPoolAsset(&self, denom: String) -> StdResult<&PoolAsset> {
        for pool_asset in &self.pool_assets {
            if pool_asset.token.denom == denom {
                return Ok(pool_asset);
            }
        }
        return Err(StdError::generic_err("pool asset not found"))
    }

    fn CalOutShareAmount(&self, token_in: Coin) -> StdResult<u64> {
        let pool_asset = self.GetPoolAsset(token_in.denom)?;

        let normalized_weight = pool_asset.weight / self.total_weight;

        let share_out_amount = CalcPoolOutGivenSingleIn(
            pool_asset.token.amount as f64, 
            normalized_weight,
            self.total_shares.amount as f64,
            token_in.amount as f64,
            self.swap_fee
        ).floor();
        Ok(share_out_amount as u64)
    }
}


// pub trait Pool:
//     Clone + Sized + TryFrom<Self::Proto, Error = StdError> + Into<Self::Proto>
// {
//     /// Protocol Buffers type
//     type Proto: MsgProto;

//     /// Parse this message proto from [`Any`].
//     fn from_any(any: &Any) -> StdResult<Self> {
//         Self::Proto::from_any(any)?.try_into()
//     }    

//     /// Serialize this message proto as [`Any`].
//     fn to_any(&self) -> StdResult<Any> {
//         self.clone().into_any()
//     }

//     /// Convert this message proto into [`Any`].
//     fn into_any(self) -> StdResult<Any> {
//         self.into().to_any()
//     }
// }


// /// Coin defines a token with a denomination and an amount.
// #[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
// pub struct SmoothWeightChangeParams {
//     pub start_time : Timestamp,
//     pub weight: String,
// }

// impl TryFrom<proto::osmosis::gamm::pool_model::balancer::SmoothWeightChangeParams> for SmoothWeightChangeParams {
//     type Error = StdError;

//     fn try_from(proto: proto::osmosis::gamm::pool_model::balancer::SmoothWeightChangeParams) -> StdResult<SmoothWeightChangeParams> {
//         SmoothWeightChangeParams::try_from(&proto)
//     }
// }

// impl TryFrom<&proto::osmosis::gamm::pool_model::balancer::SmoothWeightChangeParams> for SmoothWeightChangeParams {
//     type Error = StdError;

//     fn try_from(proto: &proto::osmosis::gamm::pool_model::balancer::SmoothWeightChangeParams) -> StdResult<SmoothWeightChangeParams> {
//         Ok(SmoothWeightChangeParams {
//             token: TryFrom::try_from(proto.token.clone().unwrap())?,
//             weight: proto.weight.parse().unwrap(),
//         })
//     }
// }

// impl From<SmoothWeightChangeParams> for proto::osmosis::gamm::pool_model::balancer::SmoothWeightChangeParams {

//     fn from(swap_amount_in_route: SmoothWeightChangeParams) -> proto::osmosis::gamm::pool_model::balancer::SmoothWeightChangeParams {
//         proto::osmosis::gamm::pool_model::balancer::SmoothWeightChangeParams::from(&swap_amount_in_route)
//     }
// }

// impl From<&SmoothWeightChangeParams> for proto::osmosis::gamm::pool_model::balancer::SmoothWeightChangeParams {
//     fn from(swap_amount_in_route: &SmoothWeightChangeParams) -> proto::osmosis::gamm::pool_model::balancer::SmoothWeightChangeParams {
//         proto::osmosis::gamm::pool_model::balancer::SmoothWeightChangeParams {
//             token: (proto::cosmos::base::v1beta1::Coin::from(&swap_amount_in_route.token)).into(),
//             weight: swap_amount_in_route.weight.to_string(),
//         }
//     }
// }
