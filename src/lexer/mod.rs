use std::fmt;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    InvalidCharacter(char, i32, i32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidCharacter(c, pos, line) => write!(
                f,
                "Invalid character '{}' at position {}, line {}",
                c, pos, line
            ),
        }
    }
}

// TokenType for brainfuck lexer
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TokenType {
    // Initialize a one-dimensional array of 30,000 elements with all the values set to 0.
    PStart,
    // End of file
    EOF,
    // Newline
    NewLine,
    // > Increment the data pointer (to point to the next cell to the right)
    IncPtr,
    // < Decrement the data pointer (to point to the next cell to the left)
    DecPtr,
    // + Increment (increase by one) the byte at the data pointer
    IncByte,
    // - Decrement (decrease by one) the byte at the data pointer
    DecByte,
    // . Output the byte at the data pointer
    WriteByte,
    // , Accept one byte of input, storing its value in the byte at the data pointer
    ReadByte,
    // [ Jump to the matching ] if the byte at the data pointer is zero
    LoopStart,
    // ] Jump to the matching [ if the byte at the data pointer is nonzero
    LoopEnd,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub(crate) token_type: TokenType,
    line: i32,
    pos: i32,
}

#[derive(Debug)]
pub struct Tokenizer {
    input: String,
    ptr: usize,
    pos: i32,
    line: i32,
    cur_tok: TokenType,
    pub(crate) tokens: Vec<Token>,
}

pub(crate) trait Lexer {
    fn new(input: &File) -> Self;
    fn pos(&self) -> i32;
    fn line(&self) -> i32;
    fn next(&mut self) -> Result<TokenType, Error>;
    fn send(&mut self);
    fn lex(&mut self) -> Result<(), Error>;
}

impl Lexer for Tokenizer {
    fn new(mut input: &File) -> Self {
        let mut strinput = String::new();
        input.read_to_string(&mut strinput).unwrap();
        Tokenizer {
            input: strinput,
            ptr: 0,
            pos: 0,
            line: 1,
            cur_tok: TokenType::PStart,
            tokens: Vec::new(),
        }
    }

    fn pos(&self) -> i32 {
        self.pos
    }

    fn line(&self) -> i32 {
        self.line
    }

    fn next(&mut self) -> Result<TokenType, Error> {
        let c = self.input.chars().nth(self.ptr).unwrap();
        let tok: TokenType;
        match c {
            '>' => {
                tok = TokenType::IncPtr;
                self.pos += 1;
            }
            '<' => {
                tok = TokenType::DecPtr;
                self.pos += 1;
            }
            '+' => {
                tok = TokenType::IncByte;
                self.pos += 1;
            }
            '-' => {
                tok = TokenType::DecByte;
                self.pos += 1;
            }
            '.' => {
                tok = TokenType::WriteByte;
                self.pos += 1;
            }
            ',' => {
                tok = TokenType::ReadByte;
                self.pos += 1;
            }
            '[' => {
                tok = TokenType::LoopStart;
                self.pos += 1;
            }
            ']' => {
                tok = TokenType::LoopEnd;
                self.pos += 1;
            }
            '\n' => {
                tok = TokenType::NewLine;
            }
            _ => {
                return Err(Error::InvalidCharacter(c, self.pos, self.line));
            }
        }
        self.ptr += 1;
        Ok(tok)
    }

    fn send(&mut self) {
        if self.cur_tok == TokenType::NewLine {
            self.line += 1;
            self.pos = 0;
        } else {
            let tok = Token {
                token_type: self.cur_tok,
                line: self.line,
                pos: self.pos,
            };
            self.tokens.push(tok)
        }
    }

    fn lex(&mut self) -> Result<(), Error> {
        while self.ptr < self.input.len() {
            let tok = self.next();
            match tok {
                Ok(tok) => self.cur_tok = tok,
                Err(e) => return Err(e)
            };
            self.send();
        }
        self.tokens.push(Token {
            token_type: TokenType::EOF,
            line: self.line+1,
            pos: 1,
        });
        Ok(())
    }
}
