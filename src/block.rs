use std::fmt::{self, Debug, Formatter};

use crate::hashable::Hashable;
use crate::BlockHash;
use crate::{u128_bytes, u32_bytes, u64_bytes};

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64,
    pub payload: String,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Block[{}]: {} at: {} with: {}",
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.payload,
        )
    }
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: BlockHash,
        nonce: u64,
        payload: String,
    ) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce,
            payload,
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(self.payload.as_bytes());

        bytes
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_block_bytes() {
        let block = Block::new(0, 0, vec![0; 32], 0, "This is genesis block".to_owned());
        let expected = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 84, 104, 105, 115, 32, 105, 115, 32, 103, 101, 110, 101, 115, 105, 115, 32, 98,
            108, 111, 99, 107,
        ];
        assert_eq!(block.bytes(), expected);
    }

    #[test]
    fn test_block_hash() {
        let block = Block::new(0, 0, vec![0; 32], 0, "This is genesis block".to_owned());
        let expected = vec![
            140, 99, 185, 114, 219, 137, 11, 38, 45, 162, 129, 48, 140, 17, 1, 179, 232, 236, 93,
            5, 19, 109, 72, 161, 154, 24, 215, 174, 65, 240, 163, 115,
        ];
        assert_eq!(block.hash(), expected);
    }
}
