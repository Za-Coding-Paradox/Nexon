fn create_genesis_block() -> Block {
    Block {
        header: BlockHeader {
            index: 0,
            previous_hash: [0u8; 32],  // all zeros, nothing before it
            merkle_root: [0u8; 32],    // no transactions
            timestamp: 1716201600,     // hardcoded moment in time
            difficulty: 1,             // very easy, no real mining needed
            nonce: 0,                  // not brute forced
        },
        body: BlockBody {
            transactions: vec![],      // empty, or one founder transaction
        }
    }
}
