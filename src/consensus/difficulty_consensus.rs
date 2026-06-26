use crate::types::block::Block;

pub const BLOCK_GENERATION_INTERVAL: u64 = 10;
pub const DIFFICULTY_ADJUSTMENT_INTERVAL: u64 = 10;

pub fn get_difficulty(chain: &[Block]) -> u64 {
    let latest_block = chain.last().unwrap();

    if latest_block.index % DIFFICULTY_ADJUSTMENT_INTERVAL == 0 && latest_block.index != 0 {
        return get_adjusted_difficulty(latest_block, chain);
    }
    latest_block.difficulty
}

pub fn get_adjusted_difficulty(latest_block: &Block, chain: &[Block]) -> u64 {
    if chain.len() < DIFFICULTY_ADJUSTMENT_INTERVAL as usize + 1 {
        return 1;
    }
    let prev_adjustment_block = &chain[chain.len() - DIFFICULTY_ADJUSTMENT_INTERVAL as usize];
    let time_expected = BLOCK_GENERATION_INTERVAL * DIFFICULTY_ADJUSTMENT_INTERVAL;
    let time_taken = latest_block.timestamp - prev_adjustment_block.timestamp;
    if time_taken < time_expected / 2 {
        return prev_adjustment_block.difficulty + 1;
    } else if time_taken > time_expected * 2 {
        return prev_adjustment_block.difficulty - 1;
    }
    prev_adjustment_block.difficulty
}
