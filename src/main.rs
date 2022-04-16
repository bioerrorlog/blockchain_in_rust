use rustchain::blockchain::BlockChain;

fn main() {
    let difficulty = 0x0000ffffffffffffffffffffffffffff;
    let mut chain = BlockChain::new(difficulty);

    let payload = String::from("This is my first block payload");
    chain
        .add_block(payload)
        .expect("Failed to add my first block");

    let payload = String::from("This is my second block payload");
    chain
        .add_block(payload)
        .expect("Failed to add my second block");

    let payload = String::from("This is my third block payload");
    chain
        .add_block(payload)
        .expect("Failed to add my third block");

    println!("\nShow blocks:\n");
    for i in &chain.blocks {
        println!("{:?}", i);
    }

    println!("\nIs this chain valid?: {}", chain.is_valid());
}
