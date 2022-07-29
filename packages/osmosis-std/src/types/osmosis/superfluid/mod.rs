pub mod v1beta1;
use osmosis_std_derive::CosmwasmExt;
/// SuperfluidAsset stores the pair of superfluid asset type and denom pair
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.SuperfluidAsset")]
pub struct SuperfluidAsset {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(enumeration = "SuperfluidAssetType", tag = "2")]
    pub asset_type: u32,
}
/// SuperfluidIntermediaryAccount takes the role of intermediary between LP token
/// and OSMO tokens for superfluid staking
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.SuperfluidIntermediaryAccount")]
pub struct SuperfluidIntermediaryAccount {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub val_addr: ::prost::alloc::string::String,
    /// perpetual gauge for rewards distribution
    #[prost(uint64, tag = "3")]
    pub gauge_id: u64,
}
/// The Osmo-Equivalent-Multiplier Record for epoch N refers to the osmo worth we
/// treat an LP share as having, for all of epoch N. Eventually this is intended
/// to be set as the Time-weighted-average-osmo-backing for the entire duration
/// of epoch N-1. (Thereby locking whats in use for epoch N as based on the prior
/// epochs rewards) However for now, this is not the TWAP but instead the spot
/// price at the boundary.  For different types of assets in the future, it could
/// change.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.OsmoEquivalentMultiplierRecord")]
pub struct OsmoEquivalentMultiplierRecord {
    #[prost(int64, tag = "1")]
    pub epoch_number: u64,
    /// superfluid asset denom, can be LP token or native token
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub multiplier: ::prost::alloc::string::String,
}
/// SuperfluidDelegationRecord takes the role of intermediary between LP token
/// and OSMO tokens for superfluid staking
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.SuperfluidDelegationRecord")]
pub struct SuperfluidDelegationRecord {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub delegation_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "4")]
    pub equivalent_staked_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.LockIdIntermediaryAccountConnection")]
pub struct LockIdIntermediaryAccountConnection {
    #[prost(uint64, tag = "1")]
    pub lock_id: u64,
    #[prost(string, tag = "2")]
    pub intermediary_account: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.UnpoolWhitelistedPools")]
pub struct UnpoolWhitelistedPools {
    #[prost(uint64, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(u32)]
pub enum SuperfluidAssetType {
    Native = 0,
    /// SuperfluidAssetTypeLendingShare = 2; // for now not exist
    LpShare = 1,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.MsgSuperfluidDelegate")]
pub struct MsgSuperfluidDelegate {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub lock_id: u64,
    #[prost(string, tag = "3")]
    pub val_addr: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.MsgSuperfluidDelegateResponse")]
pub struct MsgSuperfluidDelegateResponse {}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.MsgSuperfluidUndelegate")]
pub struct MsgSuperfluidUndelegate {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub lock_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.MsgSuperfluidUndelegateResponse")]
pub struct MsgSuperfluidUndelegateResponse {}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.MsgSuperfluidUnbondLock")]
pub struct MsgSuperfluidUnbondLock {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub lock_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.MsgSuperfluidUnbondLockResponse")]
pub struct MsgSuperfluidUnbondLockResponse {}
/// MsgLockAndSuperfluidDelegate locks coins with the unbonding period duration,
/// and then does a superfluid lock from the newly created lockup, to the
/// specified validator addr.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.MsgLockAndSuperfluidDelegate")]
pub struct MsgLockAndSuperfluidDelegate {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "3")]
    pub val_addr: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.MsgLockAndSuperfluidDelegateResponse")]
pub struct MsgLockAndSuperfluidDelegateResponse {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
/// MsgUnPoolWhitelistedPool Unpools every lock the sender has, that is
/// associated with pool pool_id. If pool_id is not approved for unpooling by
/// governance, this is a no-op. Unpooling takes the locked gamm shares, and runs
/// "ExitPool" on it, to get the constituent tokens. e.g. z gamm/pool/1 tokens
/// ExitPools into constituent tokens x uatom, y uosmo. Then it creates a new
/// lock for every constituent token, with the duration associated with the lock.
/// If the lock was unbonding, the new lockup durations should be the time left
/// until unbond completion.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.MsgUnPoolWhitelistedPool")]
pub struct MsgUnPoolWhitelistedPool {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub pool_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.MsgUnPoolWhitelistedPoolResponse")]
pub struct MsgUnPoolWhitelistedPoolResponse {
    #[prost(uint64, repeated, tag = "1")]
    pub exited_lock_ids: ::prost::alloc::vec::Vec<u64>,
}
/// Params holds parameters for the superfluid module
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.Params")]
pub struct Params {
    /// the risk_factor is to be cut on OSMO equivalent value of lp tokens for
    /// superfluid staking, default: 5%
    #[prost(string, tag = "1")]
    pub minimum_risk_factor: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.QueryParamsRequest")]
pub struct QueryParamsRequest {}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.AssetTypeRequest")]
pub struct AssetTypeRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.AssetTypeResponse")]
pub struct AssetTypeResponse {
    #[prost(enumeration = "SuperfluidAssetType", tag = "1")]
    pub asset_type: u32,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.AllAssetsRequest")]
pub struct AllAssetsRequest {}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.AllAssetsResponse")]
pub struct AllAssetsResponse {
    #[prost(message, repeated, tag = "1")]
    pub assets: ::prost::alloc::vec::Vec<SuperfluidAsset>,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.AssetMultiplierRequest")]
pub struct AssetMultiplierRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.AssetMultiplierResponse")]
pub struct AssetMultiplierResponse {
    #[prost(message, optional, tag = "1")]
    pub osmo_equivalent_multiplier: ::core::option::Option<OsmoEquivalentMultiplierRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.SuperfluidIntermediaryAccountInfo")]
pub struct SuperfluidIntermediaryAccountInfo {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub val_addr: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub gauge_id: u64,
    #[prost(string, tag = "4")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.AllIntermediaryAccountsRequest")]
pub struct AllIntermediaryAccountsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.AllIntermediaryAccountsResponse")]
pub struct AllIntermediaryAccountsResponse {
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<SuperfluidIntermediaryAccountInfo>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.ConnectedIntermediaryAccountRequest")]
pub struct ConnectedIntermediaryAccountRequest {
    #[prost(uint64, tag = "1")]
    pub lock_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.ConnectedIntermediaryAccountResponse")]
pub struct ConnectedIntermediaryAccountResponse {
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<SuperfluidIntermediaryAccountInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.TotalSuperfluidDelegationsRequest")]
pub struct TotalSuperfluidDelegationsRequest {}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.TotalSuperfluidDelegationsResponse")]
pub struct TotalSuperfluidDelegationsResponse {
    #[prost(string, tag = "1")]
    pub total_delegations: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.SuperfluidDelegationAmountRequest")]
pub struct SuperfluidDelegationAmountRequest {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.SuperfluidDelegationAmountResponse")]
pub struct SuperfluidDelegationAmountResponse {
    #[prost(message, repeated, tag = "1")]
    pub amount: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.SuperfluidDelegationsByDelegatorRequest")]
pub struct SuperfluidDelegationsByDelegatorRequest {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.SuperfluidDelegationsByDelegatorResponse")]
pub struct SuperfluidDelegationsByDelegatorResponse {
    #[prost(message, repeated, tag = "1")]
    pub superfluid_delegation_records: ::prost::alloc::vec::Vec<SuperfluidDelegationRecord>,
    #[prost(message, repeated, tag = "2")]
    pub total_delegated_coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "3")]
    pub total_equivalent_staked_amount:
        ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.SuperfluidUndelegationsByDelegatorRequest")]
pub struct SuperfluidUndelegationsByDelegatorRequest {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.SuperfluidUndelegationsByDelegatorResponse")]
pub struct SuperfluidUndelegationsByDelegatorResponse {
    #[prost(message, repeated, tag = "1")]
    pub superfluid_delegation_records: ::prost::alloc::vec::Vec<SuperfluidDelegationRecord>,
    #[prost(message, repeated, tag = "2")]
    pub total_undelegated_coins:
        ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "3")]
    pub synthetic_locks: ::prost::alloc::vec::Vec<super::lockup::SyntheticLock>,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.SuperfluidDelegationsByValidatorDenomRequest")]
pub struct SuperfluidDelegationsByValidatorDenomRequest {
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.SuperfluidDelegationsByValidatorDenomResponse")]
pub struct SuperfluidDelegationsByValidatorDenomResponse {
    #[prost(message, repeated, tag = "1")]
    pub superfluid_delegation_records: ::prost::alloc::vec::Vec<SuperfluidDelegationRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.EstimateSuperfluidDelegatedAmountByValidatorDenomRequest")]
pub struct EstimateSuperfluidDelegatedAmountByValidatorDenomRequest {
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.EstimateSuperfluidDelegatedAmountByValidatorDenomResponse")]
pub struct EstimateSuperfluidDelegatedAmountByValidatorDenomResponse {
    #[prost(message, repeated, tag = "1")]
    pub total_delegated_coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
/// GenesisState defines the module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto(type_url = "/osmosis.superfluid.GenesisState")]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub superfluid_assets: ::prost::alloc::vec::Vec<SuperfluidAsset>,
    #[prost(message, repeated, tag = "3")]
    pub osmo_equivalent_multipliers: ::prost::alloc::vec::Vec<OsmoEquivalentMultiplierRecord>,
    #[prost(message, repeated, tag = "4")]
    pub intermediary_accounts: ::prost::alloc::vec::Vec<SuperfluidIntermediaryAccount>,
    #[prost(message, repeated, tag = "5")]
    pub intemediary_account_connections:
        ::prost::alloc::vec::Vec<LockIdIntermediaryAccountConnection>,
}
