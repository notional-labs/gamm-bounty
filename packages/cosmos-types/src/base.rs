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

// impl From<proto::cosmos::base::v1beta1::Coin> for Coin {

//     fn from(proto: proto::cosmos::base::v1beta1::Coin) -> Coin {
//         Coin::from(&proto)
//     }
// }

// impl From<&proto::cosmos::base::v1beta1::Coin> for Coin {

//     fn from(proto: &proto::cosmos::base::v1beta1::Coin) -> Coin {
//         Coin {
//             denom: proto.denom.parse().unwrap(),
//             amount: proto.amount.parse().unwrap(),
//         }
//     }
// }

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