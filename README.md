# NaiveCoin

A minimal blockchain implementation in Rust demonstrating core blockchain concepts:
**SHA-256 hashing, Proof-of-Stake consensus, UTXO transactions, ECDSA signatures,
Merkle trees, difficulty adjustment, and chain validation.**

```
========================================
     NaiveCoin - Blockchain Demo
========================================

[1] Genesis block created
  ┌─────────────────────────────────────────────┐
  │ Block #0
  │ Hash:  ab69a43304201e5a...
  │ Prev:  0...
  │ Time:  1782476289
  │ Nonce: 0
  │ Diff:  1
  │ Txns:  0
  └─────────────────────────────────────────────┘

[2] Mining block 1...
     Block 1 mined!

[3] Validating chain integrity...
     Blockchain is VALID! (4 blocks)

[4] UTXO Transaction Demo
[5] Spending transaction with ECDSA signature...
     Full validation passed (signature + balance)

========================================
  Blockchain fundamentals demonstrated:
  - SHA-256 hashing & block chaining
  - Merkle root compression
  - Proof-of-Stake consensus
  - Difficulty adjustment
  - UTXO transaction model
  - ECDSA digital signatures
========================================
```

---

## Table of Contents

- [Quick Start](#quick-start)
- [Architecture](#architecture)
- [How It Works — Step by Step](#how-it-works--step-by-step)
  - [1. The Block](#1-the-block)
  - [2. Hashing and Chaining](#2-hashing-and-chaining)
  - [3. Chain Validation](#3-chain-validation)
  - [4. Proof-of-Stake Mining](#4-proof-of-stake-mining)
  - [5. Difficulty Adjustment](#5-difficulty-adjustment)
  - [6. UTXO Transactions](#6-utxo-transactions)
  - [7. ECDSA Signatures](#7-ecdsa-signatures)
  - [8. Merkle Root](#8-merkle-root)
  - [9. Chain Selection (Fork Resolution)](#9-chain-selection-fork-resolution)
- [Code Map](#code-map)
- [Dependencies](#dependencies)

---

## Quick Start

```bash
git clone <repo-url>
cd NaiveCoin
cargo run
```

Requires Rust 2021+. The demo runs end-to-end, printing each step to the console.

---

## Architecture

```
src/
├── main.rs                Entry point — runs the 7-step demo
├── types/
│   ├── block.rs           Block struct, SHA-256 hashing, Merkle root
│   └── chain.rs           Blockchain container, chain validation rules
├── transactions/
│   └── transaction.rs     UTXO model: TxIn, TxOut, signing, verification
└── consensus/
    ├── block_selection.rs  Proof-of-Stake staking condition and block mining
    ├── chain_selection.rs  Longest-chain fork resolution rule
    ├── difficulty_consensus.rs  Difficulty adjustment algorithm
    └── helper.rs           Timestamp utility functions
```

---

## How It Works — Step by Step

### 1. The Block

Every block in NaiveCoin is a struct containing:

```rust
pub struct Block {
    pub index: u64,           // Position: 0 = genesis block
    pub hash: String,         // SHA-256 fingerprint of all fields
    pub previous_hash: String,// Hash of the parent block
    pub timestamp: u64,       // Unix epoch seconds when mined
    pub data: Vec<Transaction>, // Transactions in this block
    pub merkle_root: String,  // SHA-256 hash of all transactions combined
    pub difficulty: u64,      // Mining difficulty at this height
    pub nonce: u64,           // Reserved (always 0 in this PoS impl)
    pub minter_balance: u64,  // Staker's balance for PoS target calculation
    pub minter_address: String, // Public key of who mined this block
}
```

The **genesis block** is special — it has `index = 0` and `previous_hash = "0"`
(since there is no parent). It is hardcoded in `BlockChain::new()`.

---

### 2. Hashing and Chaining

Every block's `hash` is the **SHA-256** digest of every other field concatenated:

```
hash = SHA256(
    index (8 bytes big-endian)
  + previous_hash (UTF-8 bytes)
  + timestamp (8 bytes big-endian)
  + merkle_root (UTF-8 bytes)
  + nonce (8 bytes big-endian)
  + difficulty (8 bytes big-endian)
  + minter_balance (8 bytes big-endian)
  + minter_address (UTF-8 bytes)
)
```

This is implemented in `calculate_hash()` (`src/types/block.rs:47`).

**Why this makes the chain tamper-proof:**

Each block's hash is computed from its contents. The next block stores that hash
in `previous_hash`. If anyone modifies a historical block (changing a transaction,
altering the timestamp, etc.), that block's hash changes. This breaks the link
to the next block (whose `previous_hash` still points to the old hash). The
discrepancy propagates forward through every subsequent block. Changing a single
byte in block 1 invalidates the entire chain from block 1 onward.

```
Block 0                  Block 1                  Block 2
┌──────────────┐        ┌──────────────┐        ┌──────────────┐
│ index: 0     │        │ index: 1     │        │ index: 2     │
│ hash: 0xAB   │───────>│ prev: 0xAB  │───────>│ prev: 0xCD  │
│ prev: "0"    │        │ hash: 0xCD  │        │ hash: 0xEF  │
│ data: [...]  │        │ data: [...] │        │ data: [...] │
└──────────────┘        └──────────────┘        └──────────────┘
```

---

### 3. Chain Validation

`is_valid_new_block()` (`src/types/chain.rs:43`) checks three conditions for
every consecutive block pair `(prev, next)`:

| Condition | What it prevents |
|-----------|-----------------|
| `prev.index + 1 == next.index` | Skipped/missing blocks |
| `prev.hash == next.previous_hash` | Broken chain link (tampered data) |
| `calculate_hash(next) == next.hash` | Block content was modified after mining |

`is_valid_chain()` (`src/types/chain.rs:64`) iterates all pairs and returns
`false` if any link is broken.

---

### 4. Proof-of-Stake Mining

NaiveCoin uses a custom Proof-of-Stake (PoS) consensus, not Proof-of-Work.

**The staking condition** (`is_block_staking_valid()` at `src/consensus/block_selection.rs:10`):

```
target = 2^256 * balance / difficulty

hash = SHA256(previous_hash + minter_address + timestamp)

block is valid  iff  hash (as 256-bit integer) <= target
```

**How it works:**

1. The target is proportional to the **staker's balance** — more coins means
   a larger target and higher probability of finding a valid block.
2. The target is inversely proportional to **difficulty** — higher difficulty
   shrinks the target, making blocks harder to find.
3. Every second, the miner checks if the current timestamp produces a hash
   that satisfies the inequality.

**`find_block()`** (`src/consensus/block_selection.rs:43`) runs an infinite
loop that:

1. Gets the current Unix timestamp
2. If the timestamp changed since last check (avoids redundant hashing):
   - Computes `hash = calculate_hash(index, prev_hash, timestamp, data, merkle_root, ...)`
   - Tests the hash (used as "minter address") against the staking condition
   - If valid, returns the new `Block`
3. Repeats until a valid timestamp is found

Since difficulty starts at 1 and the first 10 blocks get `balance = 1`,
a valid block is typically found within 1–2 seconds in the demo.

---

### 5. Difficulty Adjustment

The network targets a **10-second block interval**. Every 10 blocks,
`get_difficulty()` (`src/consensus/difficulty_consensus.rs:6`) adjusts:

```
If last 10 blocks took < 5 sec  → difficulty += 1  (harder)
If last 10 blocks took > 20 sec → difficulty -= 1  (easier)
Otherwise                       → difficulty unchanged
```

This ensures the chain produces blocks at a stable rate regardless of
how many stakers are participating.

---

### 6. UTXO Transactions

NaiveCoin uses the **UTXO (Unspent Transaction Output) model**, same as Bitcoin.

**Core types:**

| Type | Purpose |
|------|---------|
| `TxOut` | Locks an amount of coins to an address (public key) |
| `TxIn` | References a previous `TxOut` to spend it (includes a signature proving ownership) |
| `Transaction` | A list of `TxIn`s (inputs) and `TxOut`s (outputs), plus an `id` (SHA-256 of all inputs/outputs) |
| `UnspentTxOut` | A `TxOut` that hasn't been spent yet — the UTXO set |

**Transaction flow:**

```
Sender has UTXO: { amount: 50, address: AlicePubKey }
                                          │
                                          ▼
Alice creates a transaction:
  Inputs:  [ TxIn { ref: UTXO_0, signature: ___ } ]
  Outputs: [ TxOut { amount: 25, address: Bob },      // payment
             TxOut { amount: 25, address: Alice } ]    // change
                                          │
                      ┌───────────────────┴───────────────────┐
                      ▼                                       ▼
              UTXO set removes                    UTXO set adds:
              { amount:50, Alice }                { amount:25, Bob }
                                                  { amount:25, Alice }
```

**Validation rules** (`validate_transaction()`, `src/transactions/transaction.rs:179`):

1. **Hash integrity** — `tx.id` must equal `SHA256(tx.tx_ins + tx.tx_outs)`
2. **Structure** — non-empty inputs/outputs, valid address length, positive amounts
3. **Signature** — every input's signature must verify against the public key
   stored in the referenced UTXO's address field
4. **No double-spend** — each UTXO is referenced at most once (tracked via `HashSet`)
5. **Balance** — `sum(input values) == sum(output values)` (no money created or destroyed)

**Coinbase transaction** — a special transaction that mints new coins. It has
exactly 1 input with `tx_out_id = "0000..."` (64 zeros, signaling no real source)
and exactly 1 output with `amount = 50`. This is how new coins enter the system.

---

### 7. ECDSA Signatures

Transactions are signed using the **secp256k1 elliptic curve** (same as Bitcoin).

**Signing** (`sign_tx_in()`, `src/transactions/transaction.rs:96`):

```
message = SHA256(tx.id)       // 32-byte message digest
signature = ECDSA_sign(private_key, message)
```

The signature is DER-encoded and stored in `TxIn.signature`.

**Verification** (`verify_signature()`, `src/transactions/transaction.rs:119`):

```
message = SHA256(tx.id)
public_key = decode_hex(utxo.address)
result = ECDSA_verify(public_key, message, signature)
```

The receiver's address IS the public key (hex-encoded). To spend a UTXO,
you must provide a signature that verifies against the public key stored
in that UTXO. Only the holder of the corresponding private key can produce
such a signature, proving ownership.

---

### 8. Merkle Root

The `merkle_root` field compresses all transactions in a block into a single
64-character hash (`calculate_merkle_root()`, `src/types/block.rs:71`):

```
merkle_root = SHA256(serialize(tx[0]) + serialize(tx[1]) + ... + serialize(tx[n]))
```

Each transaction is serialized to JSON and fed into SHA-256. The resulting
digest is stored in the block's `merkle_root`. Since `calculate_hash()` includes
the merkle root, changing a single transaction changes the merkle root, which
changes the block hash, which breaks the chain. This means you can't tamper
with any transaction without detection.

---

### 9. Chain Selection (Fork Resolution)

When two miners produce blocks at the same height, the chain forks.
`replace_chain()` (`src/consensus/chain_selection.rs:4`) resolves forks
with the **longest-chain rule**:

```
if is_valid_chain(new_chain) AND len(current_chain) < len(new_chain):
    current_chain = new_chain
```

Only replace your chain if the incoming chain is both valid AND longer.
This ensures the network converges on a single history — the one with
the most accumulated work/stake.

---

## Code Map

| File | Lines | What it does |
|------|-------|-------------|
| `main.rs` | 129 | Demo harness — runs 7 steps to show every feature |
| `types/block.rs` | 79 | `Block` struct, `calculate_hash`, `calculate_merkle_root` |
| `types/chain.rs` | 74 | `BlockChain` struct, `is_valid_new_block`, `is_valid_chain` |
| `transactions/transaction.rs` | 262 | `TxIn`, `TxOut`, `Transaction`, `UnspentTxOut`, signing, verification, UTXO set management |
| `consensus/block_selection.rs` | 67 | `is_block_staking_valid`, `find_block` (PoS mining loop) |
| `consensus/chain_selection.rs` | 8 | `replace_chain` (longest-chain rule) |
| `consensus/difficulty_consensus.rs` | 28 | `get_difficulty`, `get_adjusted_difficulty` |
| `consensus/helper.rs` | 14 | `current_timestamp`, `is_valid_timestamp` |

---

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `sha2` | 0.11 | SHA-256 hashing for blocks, transactions, Merkle roots |
| `secp256k1` | 0.31 | ECDSA signatures on the secp256k1 curve |
| `serde` + `serde_json` | 1.0 | JSON serialization for Merkle root computation |
| `hex` | 0.4 | Hex encoding/decoding for hashes, keys, signatures |
| `chrono` | 0.4 | System timestamps for block creation |
| `num-bigint` | 0.4 | 256-bit integer arithmetic for PoS target comparison |
