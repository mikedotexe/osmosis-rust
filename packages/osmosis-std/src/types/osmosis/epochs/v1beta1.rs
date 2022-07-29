use osmosis_std_derive::CosmwasmExt;
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.epochs.v1beta1.EpochInfo")]
pub struct EpochInfo {
    #[prost(string, tag = "1")]
    pub identifier: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(message, optional, tag = "3")]
    pub duration: ::core::option::Option<crate::shim::Duration>,
    #[prost(int64, tag = "4")]
    pub current_epoch: u64,
    #[prost(message, optional, tag = "5")]
    pub current_epoch_start_time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(bool, tag = "6")]
    pub epoch_counting_started: bool,
    #[prost(int64, tag = "8")]
    pub current_epoch_start_height: u64,
}
/// GenesisState defines the epochs module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.epochs.v1beta1.GenesisState")]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub epochs: ::prost::alloc::vec::Vec<EpochInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.epochs.v1beta1.QueryEpochsInfoRequest")]
pub struct QueryEpochsInfoRequest {}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.epochs.v1beta1.QueryEpochsInfoResponse")]
pub struct QueryEpochsInfoResponse {
    #[prost(message, repeated, tag = "1")]
    pub epochs: ::prost::alloc::vec::Vec<EpochInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.epochs.v1beta1.QueryCurrentEpochRequest")]
pub struct QueryCurrentEpochRequest {
    #[prost(string, tag = "1")]
    pub identifier: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.epochs.v1beta1.QueryCurrentEpochResponse")]
pub struct QueryCurrentEpochResponse {
    #[prost(int64, tag = "1")]
    pub current_epoch: u64,
}
