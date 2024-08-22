use modules::blockchain::Blockchain;
mod tests;

mod modules;
fn main() {

    let mut chain = Blockchain::new();
    chain.add_block(String::from("B1 data"));
    chain.add_block(String::from("B2 data"));
    chain.add_block(String::from("B3 data"));
    chain.add_block(String::from("B3 data"));
    chain.add_block(String::from("B3 data"));
    println!("{}", Blockchain::is_valid_chain(&chain));
}
