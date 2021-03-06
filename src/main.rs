use std::fs::File;
use crate::lexer::{Lexer, Tokenizer};

mod lexer;
mod ir;

fn main() {
    let f = File::open("./hello_world.bf").unwrap();
    let mut lexer :lexer::Tokenizer = lexer::Tokenizer::new(&f);
    let e = lexer.lex();
    match e {
        Ok(_e) => println!("{:?}", lexer.tokens),
        Err(e) => println!("{:?}", e),
    }
}
