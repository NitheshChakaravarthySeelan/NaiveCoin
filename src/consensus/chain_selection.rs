use crate::types::block::Block;
use crate::types::chain::is_valid_chain;

pub fn replace_chain(chain: &mut Vec<Block>, new_chain: Vec<Block>) {
    if is_valid_chain(&new_chain) && chain.len() < new_chain.len() {
        *chain = new_chain;
    }
}
