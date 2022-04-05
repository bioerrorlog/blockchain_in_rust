use crate::BlockHash;
use crypto_hash::{digest, Algorithm};

pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;
    fn hash(&self) -> BlockHash {
        digest(Algorithm::SHA256, &self.bytes())
    }
}
