#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmoothWeightChangeParams {
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "2")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag = "3")]
    pub initial_pool_weights: ::core::option::Option<super::super::super::gamm::v1beta1::PoolAsset>, 
    #[prost(message, optional, tag = "4")]
    pub target_pool_weights: ::core::option::Option<super::super::super::gamm::v1beta1::PoolAsset>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolParams {
    #[prost(string, tag = "1")]
    pub swap_fee: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub exit_fee: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub smooth_weight_change_params: ::core::option::Option<SmoothWeightChangeParams>, 
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pool {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub id: u64,
    #[prost(message, optional, tag = "3")]
    pub pool_params: ::core::option::Option<PoolParams>,
    #[prost(string, tag = "4")]
    pub future_pool_governor: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub total_shares: ::core::option::Option<super::super::super::super::cosmos::base::v1beta1::Coin>,  
    #[prost(message, repeated, tag = "6")]
    pub pool_assets: ::prost::alloc::vec::Vec<super::super::super::gamm::v1beta1::PoolAsset>,
    #[prost(string, tag = "7")]
    pub total_weight: ::prost::alloc::string::String,
}