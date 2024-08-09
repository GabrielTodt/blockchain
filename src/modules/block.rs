extern crate chrono;
use core::fmt;
use sha2::{ Digest, Sha256};

use chrono::prelude::*;

pub struct Block{
    pub timestamp: DateTime<Local>, 
    pub last_hash: String,
    pub hash: String,
    pub data: String
}

impl Block {
    pub fn new(timestamp: DateTime<Local>, last_hash: String, hash: String, data: String) -> Block {
        Block{timestamp,last_hash, hash, data}
    } 

    pub fn genesis() -> Block {
        Block::new(Local::now(), String::from("Genesis last hash"), String::from("Genesis hash"), String::from("Genesis data"))
    }

    pub fn mine_block(last_blok: &Block, data: String) -> Block {
        let new_block_timestamp = Local::now();
        let last_hash = last_blok.hash.clone();
        let hash = Block::hash(&new_block_timestamp, &last_hash, &data);
        Block::new(new_block_timestamp, last_hash, hash, data)
    }

    pub fn hash(timestamp: &DateTime<Local>, last_hash: &str, data: &str) -> String{
        let mut hasher = Sha256::new();
        let input = format!("{}{}{}",timestamp.to_string(),last_hash, data);
        hasher.update(input.as_bytes());
        let hash = hasher.finalize();
        let hex_hash = format!("{:x}",hash);
        hex_hash
    }
}


impl fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Date:      {}\nHash:      {}\nLast Hash: {}\nData:      {}", self.timestamp, self.hash, self.last_hash, self.data)
    }
}