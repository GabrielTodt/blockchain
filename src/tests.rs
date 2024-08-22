#![cfg(test)]

use crate::modules::blockchain::Blockchain;


#[test]
fn valid_chain(){
    let mut chain = Blockchain::new();
    chain.add_block("b1 data".to_string());
    chain.add_block("b2 data".to_string());
    chain.add_block("b3 data".to_string());
    chain.add_block("b4 data".to_string());
    chain.add_block("b5 data".to_string());
    assert_eq!(true, Blockchain::is_valid_chain(&chain))
}   

#[test]
fn invalid_chain(){
    let mut chain = Blockchain::new();
    chain.add_block("b1 data".to_string());
    chain.add_block("b2 data".to_string());
    chain.add_block("b3 data".to_string());
    chain.add_block("b4 data".to_string());
    chain.add_block("b5 data".to_string());
    chain.blocks.get_mut(2).unwrap().set_hash("Invalid b3 hash");
    assert_eq!(false, Blockchain::is_valid_chain(&chain))
}

#[test]
fn valid_blockchain_genesis(){
    let chain = Blockchain::new();
    assert_eq!(true, Blockchain::is_valid_chain(&chain))
}

#[test]
fn invalid_blockchain_genesis(){
    let mut chain= Blockchain::new();
    chain.blocks.get_mut(0).unwrap().set_hash("Invalid genesis hash");
    assert_eq!(false, Blockchain::is_valid_chain(&chain))   
}

#[test]
fn invalid_block_data(){
    let mut chain = Blockchain::new();
    chain.add_block("b1 data".to_string());
    chain.add_block("b2 data".to_string());
    chain.add_block("b3 data".to_string());
    chain.add_block("b4 data".to_string());
    chain.add_block("b5 data".to_string());
    chain.blocks.get_mut(2).unwrap().tamper_data("Tampered b3 data");
    assert_eq!(false, Blockchain::is_valid_chain(&chain))
}

