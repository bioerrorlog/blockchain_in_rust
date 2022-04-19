use crate::block::Block;
use crate::hashable::Hashable;

pub struct BlockChain {
    pub blocks: Vec<Block>,
    pub difficulty: u128,
}

impl BlockChain {
    pub fn new(difficulty: u128) -> Self {
        BlockChain {
            blocks: vec![Block::genesis(difficulty)],
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

    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current_block = &self.blocks[i];
            let prev_block = &self.blocks[i - 1];

            if current_block.prev_block_hash != prev_block.hash()
                || current_block.hash != current_block.hash()
            {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn genesis_chain() {
        let chain = BlockChain::new(0x000fffffffffffffffffffffffffffff);
        assert_eq!(chain.blocks.len(), 1);
        assert!(chain.is_valid());
    }

    #[test]
    fn add_block_to_chain() {
        let mut chain = BlockChain::new(0x000fffffffffffffffffffffffffffff);
        let payload = String::from("This is second block.");
        assert_eq!(chain.add_block(payload), Ok(()));
        assert_eq!(chain.blocks.len(), 2);
        assert!(chain.is_valid());
    }

    #[test]
    fn invali_chain_is_invalid() {
        let mut chain = BlockChain::new(0x000fffffffffffffffffffffffffffff);
        chain.blocks.push(Block::new(
            0,
            vec![0; 32],
            vec![0; 32],
            0,
            String::from("this is invalid"),
        ));
        assert!(!chain.is_valid());
    }
}
