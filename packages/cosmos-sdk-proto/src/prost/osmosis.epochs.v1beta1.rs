#[derive(Clone, PartialEq, ::prost::Message)]
pub struct 	QueryCurrentEpochRequest{
    #[prost(string, tag = "1")]
    pub identifier: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct 	QueryCurrentEpochResponse{
    #[prost(uint64, tag = "1")]
    pub current_epoch: u64,
}