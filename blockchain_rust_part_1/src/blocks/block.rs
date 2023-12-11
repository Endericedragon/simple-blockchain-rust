use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::utils::{hash_to_str, serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockHeader {
    timestamp: i64,
    prev_hash: String,
    nonce: usize,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Block {
    header: BlockHeader,
    data: String,
    header_hash: String,
}

impl Block {
    pub fn new(data: &str, prev_hash: &str) -> Self {
        let mut block = Block {
            header: BlockHeader {
                timestamp: Utc::now().timestamp(),
                prev_hash: prev_hash.into(),
                nonce: 0,
            },
            data: data.into(),
            header_hash: String::new(),
        };
        block.calc_header_hash();

        block
    }

    pub fn create_genesis_block() -> Self {
        Self::new("创世区块", "")
    }

    pub fn get_header_hash(&self) -> String {
        self.header_hash.clone()
    }

    fn calc_header_hash(&mut self) {
        if let Ok(serialized) = serialize(&self.header) {
            self.header_hash = hash_to_str(&serialized)
        }
    }
}
