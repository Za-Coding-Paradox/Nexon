struct BlockHeader {
    index: u64,
    previous_hash: [u8; 32], // SHA-256 is always 32 bytes
    merkle_root: [u8; 32],    
    timestamp: u64,
    difficulty: u32,
    nonce: u64,
}

struct Transaction {
    sender: [u8; 33],        // public signature key is always 33 bytes 
    receiver: [u8; 33],
    amount: u64,
    signature: [u8; 64],       
}

struct BlockBody {
    transactions: Vec<Transaction>,   
}

struct Block {
    header: BlockHeader,
    body: BlockBody,
}
