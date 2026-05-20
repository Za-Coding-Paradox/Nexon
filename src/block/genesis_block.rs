use crate::block::block::{Block, BlockHeader, BlockBody};

pub fn createGenesisBlock() -> Block {
    Block {
        header: BlockHeader {
            index: 0,
            previousHash: [0u8; 32],  // all zeros, nothing before it
            merkleRoot: [0u8; 32],    // no transactions
            timestamp: 1_716_201_600,// hardcoded moment in time — Nexon epoch
            difficulty: 1,            // trivial difficulty, genesis is not mined
            nonce: 0,                 // not brute forced
        },
        body: BlockBody {
            transactions: vec![],     // empty — no founder allocation yet
        },
    }
}
