use rustchain::blockchain::BlockChain;

fn main() {
    let difficulty = 0x0000ffffffffffffffffffffffffffff;
    let mut chain = BlockChain::new(difficulty);

    let payload = String::from("This is my first block.");
    chain
        .add_block(payload)
        .expect("Failed to add my first block");

    let payload = String::from("This is my second block.");
    chain
        .add_block(payload)
        .expect("Failed to add my second block");

    let payload = String::from("The Block Of Three");
    chain
        .add_block(payload)
        .expect("Failed to add my third block");

    for i in &chain.blocks {
        println!("{:?}", i);
    }
}
