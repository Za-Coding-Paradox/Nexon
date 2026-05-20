#![allow(non_snake_case)]

mod block;
mod utils;

use block::blockchain::BlockChain;

fn main() {
    let chain = BlockChain::new();
    println!("Nexon blockchain initialised.");
    println!("Genesis block hash index: {}", chain.getLastBlock().header.index);
}
