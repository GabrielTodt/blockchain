extern crate chrono;
use core::fmt;
use sha2::{ Digest, Sha256};

use chrono::prelude::*;

pub struct Block{
    pub timestamp: DateTime<Utc>, 
    pub last_hash: String,
    pub hash: String,
    pub data: String
}

impl Block {
    pub fn new(timestamp: DateTime<Utc>, last_hash: String, hash: String, data: String) -> Block {
        Block{timestamp,last_hash, hash, data}
    } 

    pub fn genesis() -> Block {
        Block::new(Utc.with_ymd_and_hms(2002, 1, 26, 12, 0, 0).unwrap(), String::from("Genesis last hash"), String::from("Genesis hash"), String::from("Genesis data"))
    }

    pub fn mine_block(last_blok: &Block, data: String) -> Block {
        let new_block_timestamp = Utc::now();
        let last_hash = last_blok.hash.clone();
        let hash = Block::generate_hash(&new_block_timestamp, &last_hash, &data);
        Block::new(new_block_timestamp, last_hash, hash, data)
    }

    pub fn hash_block(&self) -> String{
        let mut hasher = Sha256::new();
        let  input = format!("{}{}{}",self.timestamp.to_string(),self.last_hash,self.data);
        hasher.update(input.as_bytes());
        let hash = hasher.finalize();
        format!("{:x}",hash)
    }

    pub fn generate_hash(timestamp: &DateTime<Utc>, last_hash: &str, data: &str) -> String{
        let mut hasher = Sha256::new();
        let input = format!("{}{}{}",timestamp.to_string(),last_hash, data);
        hasher.update(input.as_bytes());
        let hash = hasher.finalize();
        format!("{:x}",hash)
    }

    pub fn clone(&self) -> Block {
        Block::new(self.timestamp, self.last_hash.clone(), self.hash.clone(), self.data.clone())
    }

    pub fn to_string(&self) -> String{
        let string = format!("{} {} {} {}", self.timestamp, self.last_hash, self.hash, self.data);
        string
    }
    


    #[cfg(test)]
    pub fn set_hash(&mut self, hash: &str){
        self.hash = hash.to_string();
    }
    #[cfg(test)]
    pub fn tamper_data(&mut self, data:&str){
        self.data = data.to_string();
    }

}


impl fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Date:      {}\nHash:      {}\nLast Hash: {}\nData:      {}", self.timestamp, self.hash, self.last_hash, self.data)
    }
}