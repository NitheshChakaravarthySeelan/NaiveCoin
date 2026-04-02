use secp256k1::{Secp256k1, SecretKey, Message, PublicKey};
use secp256k1::ecdsa::Signature;
use sha2::{Sha256, Digest};
use hex;
use std::collections::HashSet;

pub const COINBASE_AMOUNT: i32 = 50;

#[derive(Clone, Debug)]
pub struct TxOut {
    pub address: String,
    pub amount: i32,
}

#[derive(Clone, Debug)]
pub struct TxIn {
    pub tx_out_id: String,
    pub tx_out_index: u32,
    pub signature: String,
}

#[derive(Clone, Debug)]
pub struct Transaction {
    pub id: String,
    pub tx_ins: Vec<TxIn>,
    pub tx_outs: Vec<TxOut>,
}

#[derive(Clone, Debug)]
pub struct UnspentTxOut {
    pub tx_out_id: String,
    pub tx_out_index: u32,
    pub address: String,
    pub amount: i32,
}

impl TxOut {
    pub fn new(address: String, amount: i32) -> TxOut {
        TxOut { address, amount }
    }
}

impl Transaction {
    pub fn new(id: String, tx_ins: Vec<TxIn>, tx_outs: Vec<TxOut>) -> Transaction {
        Transaction { id, tx_ins, tx_outs }
    }
}

impl UnspentTxOut {
    pub fn new(tx_out_id: String, tx_out_index: u32, address: String, amount: i32) -> UnspentTxOut {
        UnspentTxOut { tx_out_id, tx_out_index, address, amount }
    }
}

pub fn get_transaction_id(tx: &Transaction) -> String {
    let mut hasher = Sha256::new();

    for tx_in in &tx.tx_ins {
        hasher.update(tx_in.tx_out_id.as_bytes());
        hasher.update(tx_in.tx_out_index.to_string().as_bytes());
    }

    for tx_out in &tx.tx_outs {
        hasher.update(tx_out.address.as_bytes());
        hasher.update(tx_out.amount.to_string().as_bytes());
    }

    let result = hasher.finalize();
    hex::encode(result)
}

fn hash_tx_id(tx_id: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(tx_id.as_bytes());
    let bytes = hasher.finalize();
    let mut result = [0u8; 32];
    result.copy_from_slice(&bytes);
    result
}

pub fn find_unspent_tx_out(tx_in: &TxIn, unspent_outputs: &[UnspentTxOut]) -> UnspentTxOut {
    unspent_outputs
        .iter()
        .find(|utxo| utxo.tx_out_id == tx_in.tx_out_id && utxo.tx_out_index == tx_in.tx_out_index)
        .expect("referenced unspent txout not found")
        .clone()
}

pub fn sign_tx_in(tx: &Transaction, tx_in_index: usize, private_key: &str, unspent_outputs: &[UnspentTxOut]) -> String {
    let tx_in = &tx.tx_ins[tx_in_index];
    let _referenced_unspent_tx_out = find_unspent_tx_out(tx_in, unspent_outputs);

    let secret_key_bytes = hex::decode(private_key).expect("invalid private key hex");
    let secret_key = SecretKey::from_slice(&secret_key_bytes).expect("invalid private key");

    let secp = Secp256k1::signing_only();
    let message = Message::from_slice(&hash_tx_id(&tx.id)).expect("invalid message length");
    let sig = secp.sign_ecdsa(&message, &secret_key);
    hex::encode(sig.serialize_der())
}

pub fn get_tx_in_amount(tx_in: &TxIn, unspent_outputs: &[UnspentTxOut]) -> i32 {
    unspent_outputs
        .iter()
        .find(|utxo| utxo.tx_out_id == tx_in.tx_out_id && utxo.tx_out_index == tx_in.tx_out_index)
        .expect("referenced unspent txout not found")
        .amount
}

pub fn verify_signature(utxo_address: &str, tx_id: &str, signature: &str) -> bool {
    let secp = Secp256k1::verification_only();

    let result = hash_tx_id(tx_id);
    let message = Message::from_slice(&result).unwrap();

    let public_key_bytes = hex::decode(utxo_address).ok();
    if public_key_bytes.is_none() { return false; }
    let public_key = PublicKey::from_slice(&public_key_bytes.unwrap());
    if public_key.is_err() { return false; }

    let signature_bytes = hex::decode(signature).ok();
    if signature_bytes.is_none() { return false; }
    let signature = Signature::from_slice(&signature_bytes.unwrap());
    if signature.is_err() { return false; }

    secp.verify_ecdsa(&message, &signature.unwrap(), &public_key.unwrap()).is_ok()
}

pub fn validate_tx_in(tx: &Transaction, tx_in: &TxIn, unspent_outputs: &[UnspentTxOut]) -> bool {
    let referenced_utx_out = unspent_outputs
        .iter()
        .find(|utxo| utxo.tx_out_id == tx_in.tx_out_id && utxo.tx_out_index == tx_in.tx_out_index);

    if let Some(utxo) = referenced_utx_out {
        verify_signature(&utxo.address, &tx.id, &tx_in.signature)
    } else {
        false
    }
}

pub fn is_valid_transaction_structure(tx: &Transaction) -> bool {
    if tx.tx_ins.is_empty() {
        return false;
    }

    if tx.tx_outs.is_empty() {
        return false;
    }

    for tx_in in &tx.tx_ins {
        if tx_in.tx_out_id.len() != 64 {
            return false;
        }
        if tx_in.signature.len() == 0 {
            return false;
        }
    }

    for tx_out in &tx.tx_outs {
        if tx_out.address.len() == 0 {
            return false;
        }
        if tx_out.amount <= 0 {
            return false;
        }
    }

    true
}

pub fn validate_transaction(tx: &Transaction, unspent_outputs: &[UnspentTxOut]) -> bool {
    if get_transaction_id(tx) != tx.id {
        return false;
    }

    if !is_valid_transaction_structure(tx) {
        return false;
    }

    let mut total_input_values: i32 = 0;
    let mut total_output_values: i32 = 0;
    let mut used_tx_outs = HashSet::new();

    for tx_in in &tx.tx_ins {
        if !validate_tx_in(tx, tx_in, unspent_outputs) {
            return false;
        }

        if !used_tx_outs.insert((tx_in.tx_out_id.clone(), tx_in.tx_out_index)) {
            return false;
        }

        total_input_values += get_tx_in_amount(tx_in, unspent_outputs);
    }

    for tx_out in &tx.tx_outs {
        total_output_values += tx_out.amount;
    }

    total_input_values == total_output_values
}

pub fn update_unspent_tx_outs(new_txs: &[Transaction], a_unspent_tx_outs: &[UnspentTxOut]) -> Vec<UnspentTxOut> {
    let new_unspent_tx_outs: Vec<UnspentTxOut> = new_txs
        .iter()
        .flat_map(|tx| {
            tx.tx_outs
                .iter()
                .enumerate()
                .map(|(index, tx_out)| UnspentTxOut::new(tx.id.clone(), index as u32, tx_out.address.clone(), tx_out.amount))
                .collect::<Vec<_>>()
        })
        .collect();

    let consumed_tx_outs: Vec<UnspentTxOut> = new_txs
        .iter()
        .flat_map(|tx| {
            tx.tx_ins
                .iter()
                .map(|tx_in| UnspentTxOut::new(tx_in.tx_out_id.clone(), tx_in.tx_out_index, String::new(), 0))
                .collect::<Vec<_>>()
        })
        .collect();

    let mut resulting: Vec<UnspentTxOut> = a_unspent_tx_outs
        .iter()
        .filter(|u_tx_o| {
            !consumed_tx_outs.iter().any(|c| c.tx_out_id == u_tx_o.tx_out_id && c.tx_out_index == u_tx_o.tx_out_index)
        })
        .cloned()
        .collect();

    resulting.extend(new_unspent_tx_outs);
    resulting
}

pub fn validate_coinbase_tx(tx: &Transaction, block_index: u32) -> bool {
    if tx.tx_ins.len() != 1 {
        return false;
    }
    if tx.tx_outs.len() != 1 {
        return false;
    }
    if tx.tx_ins[0].tx_out_id != String::from("0".repeat(64)) {
        return false;
    }
    if tx.tx_ins[0].tx_out_index != 0 {
        return false;
    }
    if tx.tx_outs[0].amount != COINBASE_AMOUNT {
        return false;
    }
    return true;
}