//! Chain Client Configuration
use std::cmp::min;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use deepsize::DeepSizeOf;
use near_primitives::types::{AccountId, BlockHeightDelta, NumBlocks, NumSeats, ShardId};
use near_primitives::version::Version;

#[derive(Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct ClientConfig {
    /// Version of the binary.
    pub version: Version,
    /// Chain id for status.
    pub chain_id: String,
    /// Listening rpc port for status.
    pub rpc_addr: String,
    /// Duration to check for producing / skipping block.
    pub block_production_tracking_delay: Duration,
    /// Minimum duration before producing block.
    pub min_block_production_delay: Duration,
    /// Maximum wait for approvals before producing block.
    pub max_block_production_delay: Duration,
    /// Maximum duration before skipping given height.
    pub max_block_wait_delay: Duration,
    /// Duration to reduce the wait for each missed block by validator.
    pub reduce_wait_for_missing_block: Duration,
    /// Skip waiting for sync (for testing or single node testnet).
    pub skip_sync_wait: bool,
    /// How often to check that we are not out of sync.
    pub sync_check_period: Duration,
    /// While syncing, how long to check for each step.
    pub sync_step_period: Duration,
    /// Sync height threshold: below this difference in height don't start syncing.
    pub sync_height_threshold: BlockHeightDelta,
    /// How much time to wait after initial header sync
    pub header_sync_initial_timeout: Duration,
    /// How much time to wait after some progress is made in header sync
    pub header_sync_progress_timeout: Duration,
    /// How much time to wait before banning a peer in header sync if sync is too slow
    pub header_sync_stall_ban_timeout: Duration,
    /// Expected increase of header head weight per second during header sync
    pub header_sync_expected_height_per_second: u64,
    /// Minimum number of peers to start syncing.
    pub min_num_peers: usize,
    /// Period between logging summary information.
    pub log_summary_period: Duration,
    /// Produce empty blocks, use `false` for testing.
    pub produce_empty_blocks: bool,
    /// Epoch length.
    pub epoch_length: BlockHeightDelta,
    /// Number of block producer seats
    pub num_block_producer_seats: NumSeats,
    /// Maximum blocks ahead of us before becoming validators to announce account.
    pub announce_account_horizon: BlockHeightDelta,
    /// Time to persist Accounts Id in the router without removing them.
    pub ttl_account_id_router: Duration,
    /// Horizon at which instead of fetching block, fetch full state.
    pub block_fetch_horizon: BlockHeightDelta,
    /// Horizon to step from the latest block when fetching state.
    pub state_fetch_horizon: NumBlocks,
    /// Time between check to perform catchup.
    pub catchup_step_period: Duration,
    /// Time between checking to re-request chunks.
    pub chunk_request_retry_period: Duration,
    /// Time between running doomslug timer.
    pub doosmslug_step_period: Duration,
    /// Behind this horizon header fetch kicks in.
    pub block_header_fetch_horizon: BlockHeightDelta,
    /// Number of blocks to garbage collect at every gc call.
    pub gc_blocks_limit: NumBlocks,
    /// Accounts that this client tracks
    pub tracked_accounts: Vec<AccountId>,
    /// Shards that this client tracks
    pub tracked_shards: Vec<ShardId>,
    /// Not clear old data, set `true` for archive nodes.
    pub archive: bool,
    /// Number of threads for ViewClientActor pool.
    pub view_client_threads: usize,
}

/*
my_macro! {

pub struct ClientConfig {
    version: Version,
    chain_id: String,
    rpc_addr: String,
    block_production_tracking_delay: Duration,
    min_block_production_delay: Duration,
    max_block_production_delay: Duration,
    max_block_wait_delay: Duration,
    reduce_wait_for_missing_block: Duration,
    skip_sync_wait: bool,
    sync_check_period: Duration,
    sync_step_period: Duration,
    sync_height_threshold: BlockHeightDelta,
    header_sync_initial_timeout: Duration,
    header_sync_progress_timeout: Duration,
    header_sync_stall_ban_timeout: Duration,
    header_sync_expected_height_per_second: u64,
    min_num_peers: usize,
    log_summary_period: Duration,
    produce_empty_blocks: bool,
    epoch_length: BlockHeightDelta,
    num_block_producer_seats: NumSeats,
    announce_account_horizon: BlockHeightDelta,
    ttl_account_id_router: Duration,
    block_fetch_horizon: BlockHeightDelta,
    state_fetch_horizon: NumBlocks,
    catchup_step_period: Duration,
    chunk_request_retry_period: Duration,
    doosmslug_step_period: Duration,
    block_header_fetch_horizon: BlockHeightDelta,
    gc_blocks_limit: NumBlocks,
    tracked_accounts: Vec<AccountId>,
    tracked_shards: Vec<ShardId>,
    archive: bool,
    view_client_threads: usize,
}

} */

impl ClientConfig {
    pub fn test(
        skip_sync_wait: bool,
        min_block_prod_time: u64,
        max_block_prod_time: u64,
        num_block_producer_seats: NumSeats,
        archive: bool,
    ) -> Self {
        ClientConfig {
            version: Default::default(),
            chain_id: "unittest".to_string(),
            rpc_addr: "0.0.0.0:3030".to_string(),
            block_production_tracking_delay: Duration::from_millis(std::cmp::max(
                10,
                min_block_prod_time / 5,
            )),
            min_block_production_delay: Duration::from_millis(min_block_prod_time),
            max_block_production_delay: Duration::from_millis(max_block_prod_time),
            max_block_wait_delay: Duration::from_millis(3 * min_block_prod_time),
            reduce_wait_for_missing_block: Duration::from_millis(0),
            skip_sync_wait,
            sync_check_period: Duration::from_millis(100),
            sync_step_period: Duration::from_millis(10),
            sync_height_threshold: 1,
            header_sync_initial_timeout: Duration::from_secs(10),
            header_sync_progress_timeout: Duration::from_secs(2),
            header_sync_stall_ban_timeout: Duration::from_secs(30),
            header_sync_expected_height_per_second: 1,
            min_num_peers: 1,
            log_summary_period: Duration::from_secs(10),
            produce_empty_blocks: true,
            epoch_length: 10,
            num_block_producer_seats,
            announce_account_horizon: 5,
            ttl_account_id_router: Duration::from_secs(60 * 60),
            block_fetch_horizon: 50,
            state_fetch_horizon: 5,
            catchup_step_period: Duration::from_millis(min_block_prod_time / 2),
            chunk_request_retry_period: min(
                Duration::from_millis(100),
                Duration::from_millis(min_block_prod_time / 5),
            ),
            doosmslug_step_period: Duration::from_millis(100),
            block_header_fetch_horizon: 50,
            gc_blocks_limit: 100,
            tracked_accounts: vec![],
            tracked_shards: vec![],
            archive,
            view_client_threads: 1,
        }
    }
}
