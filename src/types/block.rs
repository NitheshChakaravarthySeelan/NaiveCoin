use sha2::{Sha256, Digest};
use hex;
use serde_json;
use crate::transactions::transaction::Transaction;

#[derive(Clone)]
pub struct Block {
    pub index: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: u64,
    pub data: Vec<Transaction>,
    pub merkle_root: String,
    pub difficulty: u64,
    pub nonce: u64,
    pub minter_balance: u64,
    pub minter_address: String,
}

impl Block {
    pub fn new(
        index: u64,
        hash: String,
        previous_hash: String,
        timestamp: u64,
        data: Vec<Transaction>,
        difficulty: u64,
        minter_balance: u64,
        minter_address: String,
    ) -> Block {
        let merkle_root = calculate_merkle_root(&data);
        Block {
            index,
            hash,
            previous_hash,
            timestamp,
            merkle_root,
            data,
            difficulty,
            nonce: 0,
            minter_balance,
            minter_address,
        }
    }
}

pub fn calculate_hash(
    index: u64,
    previous_hash: &str,
    timestamp: u64,
    _data: &[Transaction],
    merkle_root: &str,
    nonce: u64,
    difficulty: u64,
    minter_balance: u64,
    minter_address: &str,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(index.to_be_bytes());
    hasher.update(previous_hash.as_bytes());
    hasher.update(timestamp.to_be_bytes());
    hasher.update(merkle_root.as_bytes());
    hasher.update(nonce.to_be_bytes());
    hasher.update(difficulty.to_be_bytes());
    hasher.update(minter_balance.to_be_bytes());
    hasher.update(minter_address.as_bytes());
    let result = hasher.finalize();
    hex::encode(result.as_slice())
}

pub fn calculate_merkle_root(data: &[Transaction]) -> String {
    let mut hasher = Sha256::new();
    for item in data {
        let serialized = serde_json::to_string(item).unwrap();
        hasher.update(serialized.as_bytes());
    }
    let result = hasher.finalize();
    hex::encode(result.as_slice())
}
