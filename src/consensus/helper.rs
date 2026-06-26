use std::time::{SystemTime, UNIX_EPOCH};
use crate::types::block::Block;

pub fn current_timestamp() -> u64 {
    let start = SystemTime::now();
    let since = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since.as_secs()
}

pub fn is_valid_timestamp(new_block: &Block, prev_block: &Block) -> bool {
    new_block.timestamp >= prev_block.timestamp
}
