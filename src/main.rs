use blockchain::block::Block;

fn main() {
    let block = Block::genesis();
    println!("{:?}", &block);
}
