mod types;
mod transactions;
mod consensus;

use types::block::Block;
use types::chain::BlockChain;
use consensus::block_selection::find_block;
use consensus::difficulty_consensus::get_difficulty;
use consensus::helper::is_valid_timestamp;
use consensus::chain_selection::replace_chain;
use transactions::transaction::*;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use hex;

fn short(s: &str) -> &str {
    if s.len() > 16 { &s[..16] } else { s }
}

fn print_block(block: &Block) {
    println!("  ┌─────────────────────────────────────────────┐");
    println!("  │ Block #{}", block.index);
    println!("  │ Hash:  {}...", short(&block.hash));
    println!("  │ Prev:  {}...", short(&block.previous_hash));
    println!("  │ Time:  {}", block.timestamp);
    println!("  │ Nonce: {}", block.nonce);
    println!("  │ Diff:  {}", block.difficulty);
    println!("  │ Txns:  {}", block.data.len());
    println!("  └─────────────────────────────────────────────┘");
}

fn main() {
    println!("========================================");
    println!("     NaiveCoin - Blockchain Demo");
    println!("========================================\n");

    let mut chain = BlockChain::new();
    println!("[1] Genesis block created");
    print_block(chain.get_latest_block());

    for i in 1..=3 {
        println!("\n[2] Mining block {}...", i);
        let prev_hash = chain.get_latest_block().hash.clone();
        let difficulty = get_difficulty(&chain.blocks);
        let block = find_block(i, &prev_hash, vec![], difficulty);
        chain.add_block(block);
        println!("     Block {} mined!", i);
        print_block(chain.get_latest_block());
    }

    println!("\n[3] Validating chain integrity...");
    for i in 1..chain.blocks.len() {
        let valid = is_valid_timestamp(&chain.blocks[i], &chain.blocks[i - 1]);
        if !valid {
            println!("     Timestamp check failed at block {}", i);
        }
    }
    if chain.is_valid_chain() {
        println!("     Blockchain is VALID! ({} blocks)", chain.blocks.len());
    } else {
        println!("     Blockchain is INVALID!");
    }

    println!("\n[4] UTXO Transaction Demo");
    println!("     Generating keypair...");

    let secp = Secp256k1::signing_only();
    let secret_key = SecretKey::from_byte_array([0x01; 32]).expect("valid secret key");
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    let address = hex::encode(public_key.serialize().to_vec());
    let priv_key_hex = hex::encode([0x01; 32]);

    println!("     Address: {}...", &address[..20]);

    let tx_outs = vec![TxOut::new(address.clone(), COINBASE_AMOUNT)];
    let tx_ins = vec![TxIn::new("0".repeat(64), 0, String::new())];
    let mut tx = Transaction::new(String::new(), tx_ins, tx_outs);
    tx.id = get_transaction_id(&tx);

    println!("     Coinbase TX ID: {}...", &tx.id[..16]);

    if validate_coinbase_tx(&tx, 1) {
        println!("     Coinbase transaction valid");
    }

    println!("\n[5] Spending transaction with ECDSA signature...");
    let utxo_set = vec![UnspentTxOut::new(tx.id.clone(), 0, address.clone(), COINBASE_AMOUNT)];

    let mut spend_tx = Transaction::new(
        String::new(),
        vec![TxIn::new(tx.id.clone(), 0, String::new())],
        vec![
            TxOut::new("recipient".to_string(), 25),
            TxOut::new(address.clone(), 25),
        ],
    );
    spend_tx.id = get_transaction_id(&spend_tx);

    let sig = sign_tx_in(&spend_tx, 0, &priv_key_hex, &utxo_set);
    spend_tx.tx_ins[0].signature = sig;

    println!("     Signature: {}...", &spend_tx.tx_ins[0].signature[..20]);
    println!("     Amount: 25 -> recipient, 25 -> change");

    if is_valid_transaction_structure(&spend_tx) {
        println!("     Transaction structure valid");
    }
    if validate_transaction(&spend_tx, &utxo_set) {
        println!("     Full validation passed (signature + balance)");
    }

    println!("\n[6] Updating UTXO set after spending...");
    let updated_utxos = update_unspent_tx_outs(&[spend_tx.clone()], &utxo_set);
    println!("     UTXOs before: {} -> after: {}", utxo_set.len(), updated_utxos.len());

    println!("\n[7] Testing chain replacement (same length, no-op)...");
    let chain_copy = chain.blocks.clone();
    replace_chain(&mut chain.blocks, chain_copy);
    println!("     Chain unchanged (blocks: {})", chain.blocks.len());

    println!("\n========================================");
    println!("  Blockchain fundamentals demonstrated:");
    println!("  - SHA-256 hashing & block chaining");
    println!("  - Merkle root compression");
    println!("  - Proof-of-Stake consensus");
    println!("  - Difficulty adjustment");
    println!("  - UTXO transaction model");
    println!("  - ECDSA digital signatures");
    println!("========================================");
}
