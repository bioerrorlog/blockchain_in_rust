use crate::BlockHash;

pub struct BlockChain {
    pub blocks: Vec<BlockHash>;
}

impl BlockChain {
    pub fn new (&self) -> Self {
        BlockChain {
            blocks: vec![],
        }
    }
}
