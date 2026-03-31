struct Block {
    index: i32,
    hash: String,
    previousHash: String,
    timestamp: i32,
    data: String,
}

impl Block {
    fn new(index: i32, hash: String, previousHash: String, timestamp: i32, data: String) -> Block {
        Block {
            index,
            hash,
            previousHash,
            timestamp,
            data,
        }
    }

    fn calculateHash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.index.to_string());
        hasher.update(self.previousHash.as_bytes());
        hasher.update(self.timestamp.to_string());
        hasher.update(self.data.as_bytes());
        /// After this the hasher can't be used cause its finalized
        let result = hasher.finalize();
        // Convert the bytes to a string
        hex::encode(result.as_slice())
    }

    fn get_previous_hash(&self) -> String {
        &self.previousHash
    }

}

pub const genesisBlock: Block = Block::new(0, "816534932c2b7154836da6afc367695", null, 1231006505, "My first block!");
