#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardsEstRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, repeated, tag = "2")]
    pub lock_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(int64, tag = "3")]
    pub end_epoch: i64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardsEstResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<crate::cosmos::base::v1beta1::Coin>,
}