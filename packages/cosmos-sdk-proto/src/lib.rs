#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/cosmos/cosmos-rust/main/.images/cosmos.png",
    html_root_url = "https://docs.rs/cosmos-sdk-proto/0.9.0"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

/// The version (commit hash) of the Cosmos SDK used when generating this library.
pub const COSMOS_SDK_VERSION: &str = include_str!("prost/COSMOS_SDK_COMMIT");

/// Cosmos protobuf definitions.
pub mod cosmos {
    /// Authentication of accounts and transactions.


    /// Balances.
    pub mod bank {
        pub mod v1beta1 {
            include!("prost/cosmos.bank.v1beta1.rs");
        }
    }

    /// Base functionality.
    pub mod base {

        pub mod v1beta1 {
            include!("prost/cosmos.base.v1beta1.rs");
        }


    }

}

/// IBC protobuf definitions.
pub mod ibc {
    /// IBC applications.
    pub mod applications {

        /// ICA support.
        pub mod interchain_accounts {
            pub mod v1 {
                include!("prost/ibc.applications.interchain_accounts.v1.rs");
            }
        }
    }

}

pub mod osmosis {
    pub mod gamm {
        pub mod v1beta1 {
            include!("prost/osmosis.gamm.v1beta1.rs");
        }
    }
}