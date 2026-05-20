use std::collections::HashMap;

enum MemPoolTiers {
    High,
    Medium,
    Low,
    Dust
}

struct MemPoolEntry {
    transactionId: [u8; 32],
    fee: f64,
    entryTimestamp: u64,
    currentTier: MemPoolTier
}

struct MemoryPool {
   memPoolQueues: HashMap<MemPoolTier, vecDeque<MemPoolEntry>>,
   transactions: HashMap<[u8; 32], Transaction>,
   capacity: usize,
   boostRate: u64
}
