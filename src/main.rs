use rustchain::block::Block;
use rustchain::blockchain::BlockChain;

fn main() {
    let block = Block::genesis();
    println!("{:?}", &block);

    let mut chain = BlockChain::new(0x00000fffffffffffffffffffffffffff);
    let payload = String::from("This is second block.");
    chain.add_block(payload)
}
