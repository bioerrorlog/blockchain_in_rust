use crate::block::Block;

pub struct BlockChain {
    pub blocks: Vec<Block>,
    pub difficulty: u128,
}

impl BlockChain {
    pub fn new(difficulty: u128) -> Self {
        BlockChain {
            blocks: vec![Block::genesis()],
            difficulty,
        }
    }

    pub fn add_block(&mut self, payload: String) -> Result<(), String> {
        let new_block = Block::mine(
            self.blocks.len() as u32,
            self.blocks.last().unwrap().hash.clone(),
            payload,
            self.difficulty,
        );
        self.blocks.push(new_block.unwrap());

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn genesis_chain() {
        let chain = BlockChain::new(0x000fffffffffffffffffffffffffffff);
        assert_eq!(chain.blocks.len(), 1);
    }

    #[test]
    fn add_block_to_chain() {
        let mut chain = BlockChain::new(0x000fffffffffffffffffffffffffffff);
        let payload = String::from("This is second block.");
        assert_eq!(chain.add_block(payload), Ok(()));
        assert_eq!(chain.blocks.len(), 2);
    }
}
