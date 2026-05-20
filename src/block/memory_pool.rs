#![allow(non_snake_case)]

use std::collections::{HashMap, VecDeque};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::block::transaction::Transaction;
use crate::utils::time::currentTime;

#[derive(EnumIter, Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Debug)]
pub enum MemPoolTier {
    Dust   = 0,
    Low    = 1,
    Medium = 2,
    High   = 3,
}

impl MemPoolTier {
    pub fn fromScore(score: u64) -> MemPoolTier {
        match score {
            s if s >= 10000 => MemPoolTier::High,
            s if s >= 5000  => MemPoolTier::Medium,
            s if s >= 1000  => MemPoolTier::Low,
            _               => MemPoolTier::Dust,
        }
    }

    pub fn fromFee(fee: u64) -> MemPoolTier {
        match fee {
            f if f >= 100 => MemPoolTier::High,
            f if f >= 50  => MemPoolTier::Medium,
            f if f >= 10  => MemPoolTier::Low,
            _             => MemPoolTier::Dust,
        }
    }

    pub fn nextTier(&self) -> MemPoolTier {
        match self {
            MemPoolTier::Dust   => MemPoolTier::Low,
            MemPoolTier::Low    => MemPoolTier::Medium,
            MemPoolTier::Medium => MemPoolTier::High,
            MemPoolTier::High   => MemPoolTier::High,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MemPoolEntry {
    pub transactionId: [u8; 32],
    pub fee: u64,               
    pub entryTimestamp: u64,    // unix seconds
    pub currentTier: MemPoolTier,
}
pub struct MemPool {
    pub memPoolQueues: HashMap<MemPoolTier, VecDeque<MemPoolEntry>>,
    pub transactions: HashMap<[u8; 32], Transaction>,
    pub capacity: usize,
    pub boostRate: u64,
}

impl MemPool {
    /// Creates a new empty mempool with all tier queues initialised.
    pub fn new(capacity: usize, boostRate: u64) -> Self {
        let mut memPoolQueues: HashMap<MemPoolTier, VecDeque<MemPoolEntry>> = HashMap::new();

        for tier in MemPoolTier::iter() {
            memPoolQueues.insert(tier, VecDeque::new());
        }

        MemPool {
            memPoolQueues,
            transactions: HashMap::new(),
            capacity,
            boostRate,
        }
    }

    /// Add a new transaction to the mempool.
    /// Returns false if it is a duplicate or the mempool is full
    /// and nothing could be evicted to make room.
    pub fn addTransaction(
        &mut self,
        tx: Transaction,
        txId: [u8; 32],
        fee: u64,
    ) -> bool {
        // reject duplicates
        if self.transactions.contains_key(&txId) {
            return false;
        }

        self.ageMemPoolEntries();

        while self.transactions.len() >= self.capacity {
            if !self.evictLowestPriority() {
                return false; 
            }
        }

        let tier = MemPoolTier::fromFee(fee);

        let entry = MemPoolEntry {
            transactionId: txId,
            fee,
            entryTimestamp: currentTime(),
            currentTier: tier.clone(),
        };

        self.memPoolQueues
            .get_mut(&tier)
            .unwrap()           
            .push_back(entry);

        self.transactions.insert(txId, tx);

        true
    }

    /// Pull the top `count` transactions out of the mempool for mining.
    /// Ages entries first, then drains from highest tier downward.
    pub fn getTransactionsForMining(&mut self, count: usize) -> Vec<Transaction> {
        self.ageMemPoolEntries();

        let mut result: Vec<Transaction> = Vec::with_capacity(count);

        for tier in MemPoolTier::iter() {
            if result.len() >= count {
                break;
            }

            let queue = self.memPoolQueues.get_mut(&tier).unwrap();

            while result.len() < count {
                match queue.pop_front() {
                    Some(entry) => {
                        if let Some(tx) = self.transactions.remove(&entry.transactionId) {
                            result.push(tx);
                        }
                    }
                    None => break, 
                }
            }
        }

        result
    }

    /// Walk all tiers bottom-up and promote entries whose priority
    /// score has grown past their current tier threshold.
    /// Bottom-up ensures an entry can jump multiple tiers in one pass.
    pub fn ageMemPoolEntries(&mut self) {
        let now = currentTime();

        for tier in MemPoolTier::iter() {
            let toPromote = self.collectPromotableEntries(&tier, now);

            if !toPromote.is_empty() {
                self.insertIntoTier(toPromote, tier.nextTier());
            }
        }
    }

    /// Get a read-only view of all entries in a specific tier.
    pub fn getEntriesForTier(&self, tier: &MemPoolTier) -> Option<&VecDeque<MemPoolEntry>> {
        self.memPoolQueues.get(tier)
    }

    /// Calculate a priority score for an entry.
    /// Score grows over time — higher fee and older age both increase it.
    fn calculateThreshold(entry: &MemPoolEntry, currentTime: u64, boostRate: u64) -> u64 {
        let age = currentTime.saturating_sub(entry.entryTimestamp); // saturating prevents underflow
        entry.fee.saturating_mul(boostRate).saturating_mul(age)
    }

    /// Pull all entries out of a tier whose score has grown past
    /// their current tier. Entries that stay are left in place,
    /// preserving insertion order.
    fn collectPromotableEntries(
        &mut self,
        tier: &MemPoolTier,
        currentTime: u64,
    ) -> Vec<MemPoolEntry> {
        let mut toPromote: Vec<MemPoolEntry> = Vec::new();
        let boostRate = self.boostRate;

        if let Some(queue) = self.memPoolQueues.get_mut(tier) {
            queue.retain(|entry| {
                let score = Self::calculateThreshold(entry, currentTime, boostRate);
                let newTier = MemPoolTier::fromScore(score);

                if newTier > entry.currentTier {
                    toPromote.push(entry.clone());
                    false   
                } else {
                    true    
                }
            });
        }

        toPromote
    }

    /// Insert a batch of entries into the target tier,
    /// updating each entry's recorded tier before insertion.
    fn insertIntoTier(&mut self, entries: Vec<MemPoolEntry>, targetTier: MemPoolTier) {
        if let Some(queue) = self.memPoolQueues.get_mut(&targetTier) {
            for mut entry in entries {
                entry.currentTier = targetTier.clone();
                queue.push_back(entry);
            }
        }
    }

    /// Evict the oldest entry from the lowest non-empty tier.
    /// Returns true if something was evicted, false if mempool is empty.
    fn evictLowestPriority(&mut self) -> bool {
        for tier in MemPoolTier::iter() {
            if let Some(queue) = self.memPoolQueues.get_mut(&tier) {
                if let Some(entry) = queue.pop_front() {
                    self.transactions.remove(&entry.transactionId);
                    return true;
                }
            }
        }
        false
    }
}
