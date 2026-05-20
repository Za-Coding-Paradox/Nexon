use crate::block::transaction::Transaction;

#[derive(Clone, Debug)]
pub struct BlockHeader {
    pub index: u64,
    pub previousHash: [u8; 32],   // SHA-256 is always 32 bytes
    pub merkleRoot: [u8; 32],
    pub timestamp: u64,
    pub difficulty: u32,
    pub nonce: u64,
}

#[derive(Clone, Debug)]
pub struct BlockBody {
    pub transactions: Vec<Transaction>,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub body: BlockBody,
}
