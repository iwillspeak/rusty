#![feature(plugin)]
#![feature(io)]
#![feature(collections)]
#![feature(box_syntax)]

#![feature(core,test)]

#![plugin(rustlex)]
#[allow(plugin_as_library)]
extern crate rustlex;

use std::str::FromStr;

// Token Type Enumeration
//
// This enum holds all of the different values we will produce when
// tokenising a buffer
#[derive(Debug,PartialEq)]
#[allow(dead_code)]
pub enum Token {
    Plus,
    Assign,
    Var(String),
    Num(usize)
}

rustlex! SimpleLexer {
    // expression definitions
    let A = 'a';

    // then rules
    A => |lexer: &mut SimpleLexer<R>| Some(Token::Var(lexer.yystr()))
}

pub fn tokenise(string: &str) -> Result<Vec<Token>,String> {
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


#[cfg(test)]
mod tests
{
    use super::*;
    extern crate test;

    #[test]
    fn tokenise_vars_returns_vars() {
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
    fn tokenise_numbers_returns_nums() {
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

    #[bench]
    fn bench_tokenise(b: &mut test::Bencher) {
        b.iter(|| {
           tokenise("identifier 1 12 1000010010010 some fooo bars")
        });
    }
}
