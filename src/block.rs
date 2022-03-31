use std::fmt::{self, Debug, Formatter};

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Block")
    }
}

impl Block {}
