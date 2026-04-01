pub const BLOCK_GENERATION_INTERVAL: u64 = 10;
pub const DIFFICULTY_ADJUSTMENT_INTERVAL: u64 = 10;

pub fn get_difficulty(chain: &Vec<Block>) -> u64 {
    let latestBlock = chain.last().unwrap();

    if (latestBlock.index % DIFFICULTY_ADJUSTMENT_INTERVAL) == 0 && latestBlock.index != 0 {
        return get_adjusted_difficulty(&latestBlock, chain);
    }
    latestBlock.difficulty()
}

pub fn get_adjusted_difficulty(latestBlock: &Block, chain: &Vec<Block>) -> u64 {
    if chain.len() < (DIFFICULTY_ADJUSTMENT_INTERVAL as usize + 1) {
        return 1;
    }
    let prev_adjustment_block = chain[chain.len() - DIFFICULTY_ADJUSTMENT_INTERVAL as usize];
    let timeExpected = BLOCK_GENERATION_INTERVAL * DIFFICULTY_ADJUSTMENT_INTERVAL;
    let timeTaken = latestBlock.get_timestamp() - prev_adjustment_block.get_timestamp();
    if timeTaken < timeExpected / 2 {
        return prev_adjustment_block.difficulty() + 1;
    } else if timeTaken > timeExpected * 2 {
        return prev_adjustment_block.difficulty() - 1;
    }
    prev_adjustment_block.difficulty()
}