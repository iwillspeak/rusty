#![feature(collections)]
#![feature(core)]

use std::str::FromStr;

// Token Type Enumeration
//
// This enum holds all of the different values we will produce when
// tokenising a buffer
#[derive(Debug,PartialEq)]
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

#[cfg(not(test))]
fn main() {
    let mut token = Token::Plus;
    println!("{:?}", token);
    token = Token::Var("Hello".to_string());
    println!("{:?}", token);
    println!("{:?}", tokenise("some 123 string"));
}

#[test]
fn test_tokenise_vars_returns_vars() {
    let tokens = tokenise("hello world").unwrap();
    assert_eq!(2, tokens.len());
    {
        match tokens[0] {
            Token::Var(ref name) => assert_eq!("hello", name.as_slice()),
            _ => panic!("Expected Token::Var")
        }
    }
    {
        match tokens[1] {
            Token::Var(ref name) => assert_eq!("world", name.as_slice()),
            _ => panic!("Expected Token::Var")
        }
    }
}

#[test]
fn test_tokenise_numbers_returns_nums() {
    let tokens = tokenise("1 0 1337").unwrap();
    assert_eq!(3, tokens.len());
    {
        let ref token = tokens[0];
        assert_eq!(Token::Num(1), *token);
    }
    {
        let ref token = tokens[1];
        assert_eq!(Token::Num(0), *token);
    }
    {
        let ref token = tokens[2];
        assert_eq!(Token::Num(1337), *token);
    }
}
