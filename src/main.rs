#![feature(collections)]

use std::str::FromStr;

// Token Type Enumeration
//
// This enum holds all of the different values we will produce when
// tokenising a buffer
#[derive(Debug)]
#[allow(dead_code)]
enum Token {
    Plus,
    Assign,
    Var(String),
    Num(usize)
}

fn tokenise(string: &str) -> Result<Vec<Token>,String> {
    let mut tokens = vec!();
    for part in string.split(' ') {
        tokens.push(match part.char_at(0) {
            '0'...'9' => {
                let num = FromStr::from_str(part).unwrap();
                Token::Num(num)
            },
            'a'...'z' => Token::Var(part.to_string()),
            _ => {
                return Err(format!("unrecognised token {0}", part));
            } 
        });
    }
    Ok(tokens)
}

fn main() {
    let mut token = Token::Plus;
    println!("{:?}", token);
    token = Token::Var("Hello".to_string());
    println!("{:?}", token);
    println!("{:?}", tokenise("some 123 string"));
}
