use sha2::{Sha256, Digest};
use num_bigint::BigUint;
use std::ops::{Mul, Div};
use crate::transactions::transaction::Transaction;
use crate::consensus::helper::current_timestamp;
use crate::types::block::{Block, calculate_hash, calculate_merkle_root};

pub const MINTING_WITHOUT_COIN_INDEX: u64 = 10;

pub fn is_block_staking_valid(
    previous_hash: &str,
    address: &str,
    timestamp: u64,
    mut balance: u64,
    mut difficulty: u64,
    index: u64,
) -> bool {
    difficulty += 1;

    if index <= MINTING_WITHOUT_COIN_INDEX {
        balance += 1;
    }

    let two = BigUint::from(2u32);
    let max_hash = two.pow(256);

    let balance_big = BigUint::from(balance);
    let difficulty_big = BigUint::from(difficulty);

    let target = max_hash.mul(balance_big).div(difficulty_big);

    let mut hasher = Sha256::new();
    hasher.update(previous_hash.as_bytes());
    hasher.update(address.as_bytes());
    hasher.update(timestamp.to_be_bytes());
    let result = hasher.finalize();

    let hash_num = BigUint::from_bytes_be(&result);

    hash_num <= target
}

pub fn find_block(
    index: u64,
    previous_hash: &str,
    data: Vec<Transaction>,
    difficulty: u64,
) -> Block {
    let mut past_timestamp = 0u64;
    loop {
        let timestamp = current_timestamp();
        if past_timestamp != timestamp {
            let merkle_root = calculate_merkle_root(&data);
            let hash = calculate_hash(
                index, previous_hash, timestamp, &data,
                &merkle_root, 0, difficulty, 0, "",
            );
            if is_block_staking_valid(previous_hash, &hash, timestamp, 0, difficulty, index) {
                return Block::new(
                    index, hash, previous_hash.to_string(), timestamp,
                    data, difficulty, 0, "".to_string(),
                );
            }
            past_timestamp = timestamp;
        }
    }
}
