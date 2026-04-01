mod block;

pub struct BlockChain {
    blocks: Vec<Block>,
}

impl BlockChain {
    fn new() -> BlockChain {
        const genesis_block: Block = Block::new(0,"816534932c2b7154836da6afc367695e6337db8a921823784c14378abed4f7d7",null,Utc.now().timestamp(),"my genesis block!!");
        BlockChain {
            blocks: vec![genesis_block],
        }
    }

    fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    fn  generate_next_block(&self, block_data: String) -> Block {
        const prev_block: Block = self.blocks.last().unwrap();
        let next_index = prev_block.get_index() + 1;
        let next_timestamp = Utc::now().timestamp();
        const next_hash: String = calculateHash(next_index, prev_block.get_hash(), next_timestamp, block_data);
        Block.new(next_index, next_hash, prev_block.get_hash(), next_timestamp, block_data)
    }
}

pub fn isValidNewBlock(new_block: Block, previous_block: Block) -> bool {
    if previous_block.get_index() + 1 != new_block.get_index() {
        return false;
    }
    if previous_block.get_hash() != new_block.get_previous_hash() {
        return false;
    }
    if calculateHash(new_block.get_index(), new_block.get_previous_hash(), new_block.get_timestamp(), new_block.get_data()) != new_block.get_hash() {
        return false;
    }
    true
}

pub fn isValidChain(chain: Vec<Block>) -> bool {
    let mut last_block = chain[0];
    let mut i = 1;
    while i < chain.len() {
        if !isValidNewBlock(chain[i], last_block) {
            return false;
        }
        last_block = chain[i];
        i += 1;
    }
    true
}
