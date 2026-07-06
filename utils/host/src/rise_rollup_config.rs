//! Rise rollup config — a reduced variant of [`RollupConfig`] that omits fields
//! not present in Rise's `optimism_rollupConfig` RPC response.
//!
//! Rise omits these fields (hardcoded in [`From<RiseRollupConfig>`] for [`RollupConfig`]):
//! - `max_sequencer_drift`            → same as `seq_window_size`
//! - `channel_timeout`                → 300
//! - `granite_channel_timeout`        → [`GRANITE_CHANNEL_TIMEOUT`]
//! - `protocol_versions_address`      → [`Address::ZERO`]
//! - `superchain_config_address`      → `None`
//! - `blobs_enabled_l1_timestamp`     → `None`
//! - `da_challenge_address`           → `None`
//! - `interop_message_expiry_window`  → [`DEFAULT_INTEROP_MESSAGE_EXPIRY_WINDOW`]
//! - `regolith_time`                  → `Some(0)` (active from genesis)
//! - `canyon_time`                    → `Some(0)`
//! - `delta_time`                     → `Some(0)`
//! - `ecotone_time`                   → `Some(0)`
//! - `fjord_time`                     → `Some(0)`
//! - `pectra_blob_schedule_time`      → `None`
//! - `interop_time`                   → `None`

use alloy_chains::Chain;
use alloy_primitives::Address;
use kona_genesis::{
    AltDAConfig, BaseFeeConfig, ChainGenesis, HardForkConfig, RollupConfig,
    DEFAULT_INTEROP_MESSAGE_EXPIRY_WINDOW, GRANITE_CHANNEL_TIMEOUT,
};
use serde::Deserialize;

/// Rollup config as returned by Rise's `optimism_rollupConfig` RPC method.
///
/// Convert to a full [`RollupConfig`] via `RollupConfig::from(rise_config)`.
#[derive(Debug, Clone, Deserialize)]
pub struct RiseRollupConfig {
    pub genesis: ChainGenesis,
    pub block_time: u64,
    pub seq_window_size: u64,
    pub l1_chain_id: u64,
    pub l2_chain_id: Chain,
    pub granite_time: Option<u64>,
    pub holocene_time: Option<u64>,
    pub isthmus_time: Option<u64>,
    pub jovian_time: Option<u64>,
    pub batch_inbox_address: Address,
    pub deposit_contract_address: Address,
    pub l1_system_config_address: Address,
    pub alt_da: Option<AltDAConfig>,
    pub chain_op_config: BaseFeeConfig,
}

impl From<RiseRollupConfig> for RollupConfig {
    fn from(rise: RiseRollupConfig) -> Self {
        Self {
            genesis: rise.genesis,
            block_time: rise.block_time,
            max_sequencer_drift: rise.seq_window_size,
            seq_window_size: rise.seq_window_size,
            channel_timeout: 300,
            granite_channel_timeout: GRANITE_CHANNEL_TIMEOUT,
            l1_chain_id: rise.l1_chain_id,
            l2_chain_id: rise.l2_chain_id,
            hardforks: HardForkConfig {
                regolith_time: Some(0),
                canyon_time: Some(0),
                delta_time: Some(0),
                ecotone_time: Some(0),
                fjord_time: Some(0),
                granite_time: rise.granite_time,
                holocene_time: rise.holocene_time,
                pectra_blob_schedule_time: None,
                isthmus_time: rise.isthmus_time,
                jovian_time: rise.jovian_time,
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
            alt_da_config: rise.alt_da,
            chain_op_config: rise.chain_op_config,
        }
    }
}
