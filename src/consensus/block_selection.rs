use sha2::{Sha256, Digest};
use num_bigint::BigUint;
use num_traits::{One};
use std::ops::{Mul, Div};
use crate::types::block::{Block, Transaction};
use crate::consensus::helper::current_timestamp;

pub const MINTINGWITHOUTCOININDEX: u64 = 10;

pub fn is_block_staking_valid(
    previous_hash: &str,
    address: &str,
    timestamp: u64,
    mut balance: u64,
    mut difficulty: u64,
    index: u64
) -> bool {

    difficulty += 1;

    // Allow minting without coin for first few blocks
    if index <= MINTINGWITHOUTCOININDEX {
        balance += 1;
    }

    // 2^256
    let two = BigUint::from(2u32);
    let max_hash = two.pow(256);

    // target = 2^256 * balance / difficulty
    let balance_big = BigUint::from(balance as u32);
    let difficulty_big = BigUint::from(difficulty as u32);

    let target = max_hash.mul(balance_big).div(difficulty_big);

    // Create SHA256 hash
    let mut hasher = Sha256::new();
    hasher.update(previous_hash.as_bytes());
    hasher.update(address.as_bytes());
    hasher.update(timestamp.to_be_bytes());
    let result = hasher.finalize();

    // Convert hash to number
    let hash_num = BigUint::from_bytes_be(&result);

    // Check staking condition
    hash_num <= target
}

pub fn find_block(
    index: u64,
    previous_hash: &str,
    data: Vec<Transaction>,
    difficulty: u64,
) -> Block {
    let pastTimestamp = 0;
    while true {
        let timestamp = current_timestamp();
        if (pastTimestamp != timestamp) {
            let hash = calculateHash(index, previous_hash, timestamp, &data, &calculateMerkleRoot(&data), 0, difficulty, 0, "");
            if is_block_staking_valid(previous_hash, &hash, timestamp, 0, difficulty, index) {
                return Block::new(index, hash, previous_hash.to_string(), timestamp, data, difficulty, 0, "".to_string());
            }
            pastTimestamp = timestamp;
        }
    }
}