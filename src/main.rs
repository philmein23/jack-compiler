mod token;
use core::panic;
use std::{env::args, fs};

use token::{match_identifier, Token};

mod lex;
use lex::Lex;
fn main() {
    for arg in args().skip(1) {
        let contents = fs::read_to_string(arg).unwrap();
        let mut lex = Lex::new(contents);
        let tokens = lex.tokenize().unwrap();
        let mut token_iter = tokens.iter().peekable();
    }
}

#[test]
fn token_test() {
    let input = "var int num;\nlet num = 5;";
    let mut lex = Lex::new(input.into());
    let tokens = lex.tokenize().unwrap();
    let mut tokens_iter = tokens.iter().peekable();
    let expected_tokens: Vec<Token> = vec![
        Token::Keyword("var".into()),
        Token::Keyword("int".into()),
        Token::Identifier("num".into()),
        Token::Symbol(';'),
        Token::Keyword("let".into()),
        Token::Identifier("num".into()),
        Token::Symbol('='),
        Token::Integer(5),
    ];

    for token in expected_tokens {
        let result = tokens_iter.next();
        match result {
            Some(t) => {
                assert_eq!(token, *t);
            }
            None => {
                panic!("There are no tokens left");
            }
        }
    }
}

#[test]
fn token_test2() {
    let input = "class Person {\nfield int age;\nstatic int num;\nconstructor Person new(int age) {\nlet age = age;}\n}";
    let mut lex = Lex::new(input.into());
    let tokens = lex.tokenize().unwrap();
    let mut tokens_iter = tokens.iter().peekable();
    let expected_tokens: Vec<Token> = vec![
        Token::Keyword("class".into()),
        Token::Identifier("Person".into()),
        Token::Symbol('{'),
        Token::Keyword("field".into()),
        Token::Keyword("int".into()),
        Token::Identifier("age".into()),
        Token::Symbol(';'),
        Token::Keyword("static".into()),
        Token::Keyword("int".into()),
        Token::Identifier("num".into()),
        Token::Symbol(';'),
        Token::Keyword("constructor".into()),
        Token::Identifier("Person".into()),
        Token::Identifier("new".into()),
        Token::Symbol('('),
        Token::Keyword("int".into()),
        Token::Identifier("age".into()),
        Token::Symbol(')'),
        Token::Symbol('{'),
        Token::Keyword("let".into()),
        Token::Identifier("age".into()),
        Token::Symbol('='),
        Token::Identifier("age".into()),
        Token::Symbol(';'),
        Token::Symbol('}'),
        Token::Symbol('}'),
    ];

    for token in expected_tokens {
        let result = tokens_iter.next();
        match result {
            Some(t) => {
                assert_eq!(token, *t);
            }
            None => {
                panic!("There are no tokens left");
            }
        }
    }
}
