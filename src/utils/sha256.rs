#![allow(non_snake_case)]

use sha2::{Sha256, Digest as Sha2Digest};
use sha3::{Keccak256, Digest as Sha3Digest};

use crate::block::block::BlockHeader;
use crate::block::transaction::Transaction;

pub trait Hasher {
    fn hash(data: &[u8]) -> [u8; 32];
}

pub struct Sha256Hasher;

impl Hasher for Sha256Hasher {
    fn hash(data: &[u8]) -> [u8; 32] {
        let result = Sha256::digest(data);
        result.into()
    }
}

impl Sha256Hasher {
    /// Double SHA-256 — what Bitcoin actually uses.
    /// Hashing twice defends against length extension attacks.
    pub fn doubleHash(data: &[u8]) -> [u8; 32] {
        let first  = Sha256::digest(data);
        let second = Sha256::digest(&first);
        second.into()
    }
}

pub struct Keccak256Hasher;

impl Hasher for Keccak256Hasher {
    fn hash(data: &[u8]) -> [u8; 32] {
        let result = Keccak256::digest(data);
        result.into()
    }
}

/// Serialise a BlockHeader into raw bytes, then double-SHA-256 hash it.
/// This is what miners brute force when searching for a valid nonce.
/// Field order is fixed — changing it breaks chain compatibility.
pub fn hashBlockHeader(header: &BlockHeader) -> [u8; 32] {
    let bytes = serializeHeader(header);
    Sha256Hasher::doubleHash(&bytes)
}

/// Serialise a BlockHeader into a fixed-layout byte array.
/// Every field is written in big-endian order.
/// Total size: 32 + 32 + 8 + 4 + 8 = 84 bytes (excluding index)
///           + 8 bytes for index = 92 bytes total.
pub fn serializeHeader(header: &BlockHeader) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(92);

    bytes.extend_from_slice(&header.index.to_be_bytes());

    bytes.extend_from_slice(&header.previousHash);

    bytes.extend_from_slice(&header.merkleRoot);

    bytes.extend_from_slice(&header.timestamp.to_be_bytes());

    bytes.extend_from_slice(&header.difficulty.to_be_bytes());

    bytes.extend_from_slice(&header.nonce.to_be_bytes());

    bytes
}

/// Hash any arbitrary byte slice with SHA-256.
/// Used for transaction IDs, merkle nodes, and anything else.
pub fn hashBytes(data: &[u8]) -> [u8; 32] {
    Sha256Hasher::hash(data)
}

/// Hash any arbitrary byte slice with double SHA-256.
pub fn doubleHashBytes(data: &[u8]) -> [u8; 32] {
    Sha256Hasher::doubleHash(data)
}

/// Convert a [u8; 32] hash to a lowercase hex string for display.
pub fn hashToHex(hash: &[u8; 32]) -> String {
    hex::encode(hash)
}

/// Convert a hex string back to a [u8; 32] hash.
/// Returns None if the string is not valid hex or wrong length.
pub fn hexToHash(hexStr: &str) -> Option<[u8; 32]> {
    let bytes = hex::decode(hexStr).ok()?;
    if bytes.len() != 32 {
        return None;
    }
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&bytes);
    Some(hash)
}
 
pub fn meetsDifficulty(hash: &[u8; 32], difficulty: u32) -> bool {
    let requiredZeroBytes = (difficulty / 8) as usize;  
    let requiredZeroBits  = (difficulty % 8) as u8;     

    for i in 0..requiredZeroBytes {
        if hash[i] != 0 {
            return false;
        }
    }

    if requiredZeroBits > 0 {
        let mask = 0xFF >> requiredZeroBits;
        if hash[requiredZeroBytes] & !mask != 0 {
            return false;
        }
    }

    true
}

pub fn hashTransaction(tx: &Transaction) -> [u8; 32] {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&tx.sender);
    bytes.extend_from_slice(&tx.receiver);
    bytes.extend_from_slice(&tx.amount.to_be_bytes());
    bytes.extend_from_slice(&tx.signature);
    doubleHashBytes(&bytes)
}
