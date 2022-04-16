use rustchain::blockchain::BlockChain;

fn main() {
    let mut chain = BlockChain::new(0x0000ffffffffffffffffffffffffffff);

    let payload = String::from("This is my first block.");
    chain
        .add_block(payload)
        .expect("Failed to add my first block");

    let payload = String::from("This is my second block.");
    chain
        .add_block(payload)
        .expect("Failed to add my first block");
}
