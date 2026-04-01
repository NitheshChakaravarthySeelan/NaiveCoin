use sha2::{Sha256, Digest};
use hex;
use serde_derive::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
struct Transaction {
    id: String,
    tx_type: String,
    sender: String,
    receiver: String,
    amount: u64,
    timestamp: u64,
    signature: String,
}
pub struct Block {
    index: u64,
    hash: String,
    previousHash: String,
    timestamp: u64,
    data: Vec<Transaction>,
    merkleRoot: String, // Compressed version of the data field
    difficulty: u64,
    nonce: u64,
    minterBalence: u64,
    minter_address: String,
}

impl Block {
    fn new(index: u64,
            hash: String,
            previousHash: String,
            timestamp: u64,
            data: Vec<Transaction>,
            difficulty: u64,
            minterBalence: u64,
            minter_address: String
        ) -> Block {
            Block {
                index,
                hash,
                previousHash,
                timestamp,
                merkleRoot: calculateMerkleRoot(&data),
                data,
                difficulty,
                nonce: 0,
                minterBalence,
                minter_address,
            }
    }


}

pub fn calculateHash(index: u64, previousHash: &str, timestamp: u64, data: Vec<Transaction>,merkle_root: &str, nonce: u64, difficulty: u64, minterBalence: u64, minter_address: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(index.to_be_bytes());
    hasher.update(previousHash.as_bytes());
    hasher.update(timestamp.to_be_bytes());
    hasher.update(merkle_root.as_bytes());
    hasher.update(nonce.to_be_bytes());
    hasher.update(difficulty.to_be_bytes());
    hasher.update(minterBalence.to_be_bytes());
    hasher.update(minter_address.as_bytes());
    let result = hasher.finalize();
    hex::encode(result.as_slice())
}

pub fn calculateMerkleRoot(data: &Vec<Transaction>) -> String {
    let mut hasher = Sha256::new();
    for item in data {
        let serialized = serde_json::to_string(item).unwrap();
        hasher.update(serialized.as_bytes());
    }
    let result = hasher.finalize();
    hex::encode(result.as_slice())
}