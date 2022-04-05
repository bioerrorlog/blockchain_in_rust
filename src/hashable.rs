use crypto_hash::{self, Algorithm}
use crate::BlockHash;

pub trait Hashable {
    fn bytes (&self) -> Vec<u8>;
    fn hash (&self) -> BlockHash {
        crypto_hash::digest(Algorithm::SHA256, &self.bytes())
    }
}
