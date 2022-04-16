use crate::{block::Block, now};

#[derive(Debug)]
pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}

pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain { blocks: vec![] }
    }

    //  pub fn add_block(&mut self, payload: String) -> Result<(), BlockValidationErr> {
    //      let mut block = Block::new(
    //          self.blocks.len() as u32,
    //          now(),  // will be updated
    //          self.blocks.last().unwrap().hash,
    //          "This is genesis block".to_owned(),
    //      );

    //      Ok(())
    //  }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_initial_chain() {
        let chain = BlockChain::new();
        assert_eq!(chain.blocks.last(), None);
    }
}
