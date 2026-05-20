use crate::block::memory_pool::MemPool;
use crate::block::block::Block;
use crate::block::genesis_block::createGenesisBlock;
use crate::block::transaction::Transaction;
use crate::utils::sha256::{hashBlockHeader, hashTransaction, meetsDifficulty};
use std::collections::{HashMap};
struct BlockChain {
    chain: Vec<Block>,                          
    heightIndex: HashMap<u64, usize>,           
    hashIndex: HashMap<[u8; 32], usize>,        
    memPool: MemPool
}

impl BlockChain {
    pub fn addBlock(&mut self, blockBatch: Vec<Block>) -> Vec<bool> {
        let mut results: Vec<bool> = Vec::with_capacity(blockBatch.len());

        for block in blockBatch.into_iter() {
            let lastBlock = self.chain.last().unwrap();

            let valid = self.isValidBlock(&block, lastBlock);

            if valid {
                let blockHash = hashBlockHeader(&block.header);
                let insertPosition = self.chain.len();

                self.hashIndex.insert(blockHash, insertPosition);
                self.heightIndex.insert(block.header.index, insertPosition);

                self.chain.push(block);         
                results.push(true);
            } else {
                results.push(false);
            }
        }

        results
    }

    pub fn isValidChain(&self) -> bool {
        for i in 1..self.chain.len() {
            if !self.isValidBlock(&self.chain[i], &self.chain[i - 1]) {
                return false;
            }
        }
        true
    }
       
    pub fn validateBlockBatch(&self, blockBatch: Vec<&Block>) -> Vec<bool> {
        let mut results: Vec<bool> = Vec::with_capacity(blockBatch.len());

        for i in 0..blockBatch.len() {
            let currentBlock = blockBatch[i];

            let previousBlock = if i == 0 {
                self.getLastBlock()
            } else {
                blockBatch[i - 1] 
            };

            let valid = self.isValidBlock(currentBlock, previousBlock);
            results.push(valid);

            if !valid {
                for _ in (i + 1)..blockBatch.len() {
                    results.push(false);
                }
                break;
            }
        }

        results
    }

    fn isValidBlock(&self, current: &Block, previous: &Block) -> bool {
        if current.header.index != previous.header.index + 1 {
            println!("Invalid index at block {}", current.header.index);
            return false;
        }

        let previousHash = hashBlockHeader(&previous.header);
        if current.header.previousHash != previousHash {
            println!("Broken chain linkage at block {}", current.header.index);
            return false;
        }

        let currentHash = hashBlockHeader(&current.header);
        if !meetsDifficulty(&currentHash, current.header.difficulty) {
            println!("Insufficient proof of work at block {}", current.header.index);
            return false;
        }

        true
    }

    pub fn getLastBlock(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn addToMemPool(&mut self, transactionBatch: Vec<Transaction>, fees: Vec<u64>) -> Vec<bool> {
        let mut results: Vec<bool> = Vec::with_capacity(transactionBatch.len());

        for (tx, fee) in transactionBatch.into_iter().zip(fees.iter()) {
            let txId = hashTransaction(&tx);

            let accepted = self.memPool.addTransaction(tx, txId, *fee);
            results.push(accepted);
        }

        results
    }

    pub fn hashBlockBatch(&self, hashableBlockBatch: Vec<&Block>) -> Vec<[u8; 32]> {
        hashableBlockBatch
            .iter()
            .map(|block| hashBlockHeader(&block.header))
            .collect()
    }
}
