use std::fmt::{self, Debug, Formatter};

use crate::hashable::Hashable;
use crate::now;
use crate::BlockHash;
use crate::{difficulty_bytes_as_u128, u128_bytes, u32_bytes, u64_bytes};

#[derive(PartialEq)]
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
    pub fn genesis() -> Self {
        Block {
            index: 0,
            timestamp: now(),
            hash: vec![0; 32],
            prev_block_hash: vec![0; 32],
            nonce: 0,
            payload: "This is genesis block".to_owned(),
        }
    }

    pub fn mine(
        index: u32,
        prev_block_hash: BlockHash,
        payload: String,
        difficulty: u128,
    ) -> Result<Self, String> {
        let mut new_block = Block {
            index,
            timestamp: now(),
            hash: vec![0; 32],
            prev_block_hash,
            nonce: 0,
            payload,
        };

        for nonce_attempt in 0..(u64::max_value()) {
            new_block.nonce = nonce_attempt;
            let hash = new_block.hash();
            if check_difficulty(&hash, difficulty) {
                new_block.nonce = nonce_attempt;
                new_block.hash = hash;
                return Ok(new_block);
            }
        }

        Err(String::from(
            "mining failed: all nonce attempt was invalid.",
        ))
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

pub fn check_difficulty(hash: &BlockHash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash)
}
