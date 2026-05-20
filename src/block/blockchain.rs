struct BlockChain {
    chain: vec<Block>,
    memPool: MemPool
}

impl BlockChain {
    fn new() -> BlockChain {}
    fn addBlock (&mut self, block: Block) {}
    fn isValidChain (&self) -> bool {}
    fn getLastBlock(&self) -> Block {}
    fn addToMemPool(&mut self, transaction: Transaction) {}
}

