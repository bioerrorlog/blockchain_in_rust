use blockchain::block::Block;

fn main() {
    let block = Block::new(0, 0, vec![0; 32], 0, "This is genesis block".to_owned());
    println!("{:?}", &block);
}
