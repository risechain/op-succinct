//! Rise rollup config — a reduced variant of [`RollupConfig`] that mirrors Rise's
//! `optimism_rollupConfig` RPC response.
//!
//! Rise's node serves a trimmed schema, so the response cannot fill a [`RollupConfig`] on its
//! own. The omitted fields are hardcoded in the [`From`] impls below.
//!
//! Every field declared here is required: no `Option` stands in for a field the node may stop
//! sending, so the next schema change fails loudly at startup instead of silently deserializing
//! to `None` — which is how the hardfork times went unnoticed once Rise dropped them.
//!
//! Hardforks: Rise runs a single fork, active from genesis, and serves no activation times.
//! - `regolith_time` … `jovian_time`  → `Some(0)`
//! - `pectra_blob_schedule_time`      → `None`
//! - `interop_time`                   → `None`
//!
//! Fee parameters Rise pins at zero and drops from `system_config`:
//! - `overhead`                       → 0 (vestigial post-Ecotone)
//! - `scalar`                         → 0 (the L1 fee is free)
//! - `base_fee_scalar`                → `None` (post-Ecotone both are read out of `scalar`)
//! - `blob_base_fee_scalar`           → `None`
//! - `operator_fee_scalar`            → `None` (no operator fee is charged)
//! - `operator_fee_constant`          → `None`
//! - `da_footprint_gas_scalar`        → `None` (no DA footprint is charged)
//!
//! Derivation parameters and addresses Rise does not serve:
//! - `max_sequencer_drift`            → 600
//! - `channel_timeout`                → 300
//! - `granite_channel_timeout`        → [`GRANITE_CHANNEL_TIMEOUT`]
//! - `protocol_versions_address`      → [`Address::ZERO`]
//! - `superchain_config_address`      → `None`
//! - `blobs_enabled_l1_timestamp`     → `None`
//! - `da_challenge_address`           → `None`
//! - `alt_da_config`                  → `None`
//! - `interop_message_expiry_window`  → [`DEFAULT_INTEROP_MESSAGE_EXPIRY_WINDOW`]

use alloy_chains::Chain;
use alloy_eips::BlockNumHash;
use alloy_primitives::{Address, B64, U256};
use kona_genesis::{
    BaseFeeConfig, ChainGenesis, HardForkConfig, RollupConfig, SystemConfig,
    DEFAULT_INTEROP_MESSAGE_EXPIRY_WINDOW, GRANITE_CHANNEL_TIMEOUT,
};
use op_alloy_consensus::decode_eip_1559_params;
use serde::Deserialize;

/// Rollup config as returned by Rise's `optimism_rollupConfig` RPC method.
///
/// Convert to a full [`RollupConfig`] via `RollupConfig::from(rise_config)`.
#[derive(Debug, Clone, Deserialize)]
pub struct RiseRollupConfig {
    pub genesis: RiseChainGenesis,
    pub block_time: u64,
    pub seq_window_size: u64,
    pub l1_chain_id: u64,
    pub l2_chain_id: Chain,
    pub batch_inbox_address: Address,
    pub deposit_contract_address: Address,
    pub l1_system_config_address: Address,
    pub chain_op_config: BaseFeeConfig,
}

/// Genesis anchor of [`RiseRollupConfig`], a reduced variant of [`ChainGenesis`].
#[derive(Debug, Clone, Deserialize)]
pub struct RiseChainGenesis {
    pub l1: BlockNumHash,
    pub l2: BlockNumHash,
    pub l2_time: u64,
    pub system_config: RiseSystemConfig,
}

/// Genesis system config of [`RiseChainGenesis`], a reduced variant of [`SystemConfig`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiseSystemConfig {
    pub batcher_addr: Address,
    pub gas_limit: u64,
    /// EIP-1559 denominator and elasticity, packed as in the post-Holocene header nonce.
    pub eip1559_params: B64,
    pub min_base_fee: u64,
}

impl From<RiseRollupConfig> for RollupConfig {
    fn from(rise: RiseRollupConfig) -> Self {
        Self {
            genesis: rise.genesis.into(),
            block_time: rise.block_time,
            max_sequencer_drift: 600,
            seq_window_size: rise.seq_window_size,
            channel_timeout: 300,
            granite_channel_timeout: GRANITE_CHANNEL_TIMEOUT,
            l1_chain_id: rise.l1_chain_id,
            l2_chain_id: rise.l2_chain_id,
            // Rise activates every fork up to Jovian at genesis, so no block is ever a fork's
            // first block and every block follows the same rules.
            hardforks: HardForkConfig {
                regolith_time: Some(0),
                canyon_time: Some(0),
                delta_time: Some(0),
                ecotone_time: Some(0),
                fjord_time: Some(0),
                granite_time: Some(0),
                holocene_time: Some(0),
                pectra_blob_schedule_time: None,
                isthmus_time: Some(0),
                jovian_time: Some(0),
                interop_time: None,
            },
            batch_inbox_address: rise.batch_inbox_address,
            deposit_contract_address: rise.deposit_contract_address,
            l1_system_config_address: rise.l1_system_config_address,
            protocol_versions_address: Address::ZERO,
            superchain_config_address: None,
            blobs_enabled_l1_timestamp: None,
            da_challenge_address: None,
            interop_message_expiry_window: DEFAULT_INTEROP_MESSAGE_EXPIRY_WINDOW,
            alt_da_config: None,
            chain_op_config: rise.chain_op_config,
        }
    }
}

impl From<RiseChainGenesis> for ChainGenesis {
    fn from(rise: RiseChainGenesis) -> Self {
        Self {
            l1: rise.l1,
            l2: rise.l2,
            l2_time: rise.l2_time,
            system_config: Some(rise.system_config.into()),
        }
    }
}

impl From<RiseSystemConfig> for SystemConfig {
    fn from(rise: RiseSystemConfig) -> Self {
        let (elasticity, denominator) = decode_eip_1559_params(rise.eip1559_params);

        Self {
            batcher_address: rise.batcher_addr,
            overhead: U256::ZERO,
            scalar: U256::ZERO,
            gas_limit: rise.gas_limit,
            base_fee_scalar: None,
            blob_base_fee_scalar: None,
            eip1559_denominator: Some(denominator),
            eip1559_elasticity: Some(elasticity),
            operator_fee_scalar: None,
            operator_fee_constant: None,
            min_base_fee: Some(rise.min_base_fee),
            da_footprint_gas_scalar: None,
        }
    }
}
