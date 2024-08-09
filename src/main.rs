use modules::block::Block;

mod modules;
fn main() {
    let b1 = Block::genesis();
    let data = String::from("Mined block");
    let b2 = Block::mine_block(&b1, data);
    println!("{}", b2);
}
