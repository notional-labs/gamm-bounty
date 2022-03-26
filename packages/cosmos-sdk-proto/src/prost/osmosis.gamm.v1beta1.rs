#[derive(Clone, PartialEq, ::prost::Message)]   
pub struct SwapAmountInRoute {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub token_out_denom: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwapExactAmountIn {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountInRoute>,
    #[prost(message, optional, tag = "3")]
    pub token_in: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,  
    #[prost(string, tag = "4")]
    pub token_out_min_amount: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgJoinPool {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(string, tag = "3")]
    pub share_out_amount: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub token_in_maxs: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,  
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgJoinSwapExternAmountIn {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
    #[prost(message, optional, tag = "3")]
    pub token_in: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,  
    #[prost(string, tag = "4")]
    pub share_out_min_amount: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotPriceRequest {
    /// address is the address to query balances for.
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,

    #[prost(string, tag = "2")]
    pub token_in_denom: ::prost::alloc::string::String,

    #[prost(string, tag = "3")]
    pub token_out_denom: ::prost::alloc::string::String,

    #[prost(bool, tag = "4")]
    pub with_swap_fee: bool,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotPriceResponse {
    #[prost(string, tag = "1")]
    pub spot_price: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct 	QuerySwapExactAmountInRequest{
    /// address is the address to query balances for.
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,

    #[prost(uint64, tag = "2")]
    pub pool_id: u64,

    #[prost(string, tag = "3")]
    pub token_in: ::prost::alloc::string::String,

    #[prost(message, repeated, tag = "4")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountInRoute>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySwapExactAmountInResponse {
    #[prost(string, tag = "1")]
    pub token_out_amount: ::prost::alloc::string::String,  
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolAsset {
    #[prost(message, optional, tag = "1")]
    pub token: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "2")]
    pub weight: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct 	QueryPoolRequest{
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct 	QueryPoolResponse{
    #[prost(message, optional, tag = "1")]
    pub pool: ::core::option::Option<::prost_types::Any>,
}