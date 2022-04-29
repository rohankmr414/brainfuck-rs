use crate::lexer::{Lexer, Tokenizer};
use std::fs::File;

mod ir;
mod lexer;

fn main() {
    let f = File::open("./hello_world.bf").unwrap();
    let mut lexer: lexer::Tokenizer = lexer::Tokenizer::new(&f);
    let e = lexer.lex();
    match e {
        Ok(_e) => println!("{:?}", lexer.tokens),
        Err(e) => println!("{:?}", e),
    }
}
