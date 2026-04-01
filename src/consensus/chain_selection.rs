mod chain;

use crate::types::block::Block;
use crate::types::chain::isValidChain;

/// Longest number of block in the chain
pub fn replace_chain(chain: &mut Vec<Block>, newChain: Vec<Block>) {
    if isValidChain(&newChain) && chain.len() < newChain.len() {
        *chain = newChain;
    }
}