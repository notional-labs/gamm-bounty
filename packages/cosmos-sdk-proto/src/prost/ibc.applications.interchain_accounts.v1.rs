#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosTx {
    #[prost(message, repeated, tag = "1")]
    pub messages: ::prost::alloc::vec::Vec<::prost_types::Any>,
}