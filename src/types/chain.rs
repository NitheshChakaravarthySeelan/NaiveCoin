use chrono::Utc;
use crate::types::block::{Block, calculate_hash, calculate_merkle_root};

pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> BlockChain {
        let timestamp = Utc::now().timestamp() as u64;
        let genesis_hash = calculate_hash(
            0, "0", timestamp, &[],
            &calculate_merkle_root(&[]), 0, 1, 0, "genesis",
        );
        let genesis_block = Block::new(
            0,
            genesis_hash,
            "0".to_string(),
            timestamp,
            vec![],
            1,
            0,
            "genesis".to_string(),
        );
        BlockChain {
            blocks: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn get_latest_block(&self) -> &Block {
        self.blocks.last().unwrap()
    }

    pub fn is_valid_chain(&self) -> bool {
        is_valid_chain(&self.blocks)
    }
}

pub fn is_valid_new_block(new_block: &Block, previous_block: &Block) -> bool {
    if previous_block.index + 1 != new_block.index {
        return false;
    }
    if previous_block.hash != new_block.previous_hash {
        return false;
    }
    let recalculated = calculate_hash(
        new_block.index,
        &new_block.previous_hash,
        new_block.timestamp,
        &new_block.data,
        &new_block.merkle_root,
        new_block.nonce,
        new_block.difficulty,
        new_block.minter_balance,
        &new_block.minter_address,
    );
    recalculated == new_block.hash
}

pub fn is_valid_chain(chain: &[Block]) -> bool {
    if chain.is_empty() {
        return false;
    }
    for i in 1..chain.len() {
        if !is_valid_new_block(&chain[i], &chain[i - 1]) {
            return false;
        }
    }
    true
}
