//! Base functionality.

use crate::{proto};
use cosmwasm_std::{StdResult, StdError};

use core::convert::TryFrom;
use std::fmt;

/// Coin defines a token with a denomination and an amount.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Coin {
    /// Denomination
    pub denom: String,

    /// Amount
    pub amount: String,
}

impl fmt::Display for Coin{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.amount, self.denom)
    }
}

impl TryFrom<proto::cosmos::base::v1beta1::Coin> for Coin {
    type Error = StdError;

    fn try_from(proto: proto::cosmos::base::v1beta1::Coin) -> StdResult<Coin> {
        Coin::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::base::v1beta1::Coin> for Coin {
    type Error = StdError;

    fn try_from(proto: &proto::cosmos::base::v1beta1::Coin) -> StdResult<Coin> {
        Ok(Coin {
            denom: proto.denom.parse().unwrap(),
            amount: proto.amount.parse().unwrap(),
        })
    }
}

impl From<Coin> for proto::cosmos::base::v1beta1::Coin {
    fn from(coin: Coin) -> proto::cosmos::base::v1beta1::Coin {
        proto::cosmos::base::v1beta1::Coin::from(&coin)
    }
}

impl From<&Coin> for proto::cosmos::base::v1beta1::Coin {
    fn from(coin: &Coin) -> proto::cosmos::base::v1beta1::Coin {
        proto::cosmos::base::v1beta1::Coin {
            denom: coin.denom.to_string(),
            amount: coin.amount.to_string(),
        }
    }
}

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