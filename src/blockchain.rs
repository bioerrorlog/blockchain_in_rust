use crate::block::Block;

pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain { blocks: vec![] }
    }

    pub fn get_last_block(&self) -> Option<&Block> {
        self.blocks.last()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_initial_chain() {
        let chain = BlockChain::new();
        assert_eq!(chain.get_last_block(), None);
    }
}
