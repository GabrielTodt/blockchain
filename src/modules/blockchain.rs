use super::block::Block;

pub struct Blockchain{
    pub blocks: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Blockchain{
        let mut blockchain = Blockchain{
            blocks: Vec::new()
        };
        blockchain.blocks.push(Block::genesis());
        blockchain
    }

    pub fn add_block(&mut self, data: String) -> Option<Block>{
        let last_block = match self.blocks.last() {
            Some(value) => value,
            None => return None
        };
        let new_block = Block::mine_block(last_block, data);
        self.blocks.push(new_block.clone());
        Some(new_block)
    }

    pub fn is_valid_chain(chain: &Blockchain) -> bool{
        let genesis = Block::genesis();
        if chain.blocks.first().unwrap().hash != genesis.hash {
            return false;
        }
        for i in 1..chain.blocks.len(){
            let block = chain.blocks.get(i).unwrap().clone();
            if block.last_hash != chain.blocks.get(i-1).unwrap().hash || block.hash != Block::generate_hash(&block.timestamp, &block.last_hash, &block.data){
                return false;
            }        
        }
        return  true;
    }

    pub fn print_chain(&self){
        let mut count = 0;
        for element in &self.blocks{
            println!("{} --> {}",count, element);
            count+=1;
        }
    }
}