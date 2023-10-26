use std::env;

use calculator::{fractions::Fraction, parse_tree::ParseTree};


fn main() {
    let args: Vec<String> = env::args().collect();
    let tree = ParseTree::<Fraction>::from_string(&args[1]);

    match tree {
        Ok(tree) => {
            println!("{}",tree.eval());
        }
        Err(e) => {
            println!("{}",e);
        }
    }
}
