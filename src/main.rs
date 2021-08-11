mod token;
use core::panic;
use std::{env::args, fs};

use token::{match_identifier, Token};

mod lex;
use lex::Lex;

mod compilation_engine;
use compilation_engine::CompilationEngine;

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
        Token::Var,
        Token::Int,
        Token::Identifier("num".into()),
        Token::SemiColon,
        Token::Let,
        Token::Identifier("num".into()),
        Token::Equal,
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
        Token::Class,
        Token::Identifier("Person".into()),
        Token::LeftBrace,
        Token::Field,
        Token::Int,
        Token::Identifier("age".into()),
        Token::SemiColon,
        Token::Static,
        Token::Int,
        Token::Identifier("num".into()),
        Token::SemiColon,
        Token::Constructor,
        Token::Identifier("Person".into()),
        Token::Identifier("new".into()),
        Token::LeftParen,
        Token::Int,
        Token::Identifier("age".into()),
        Token::RightParen,
        Token::LeftBrace,
        Token::Let,
        Token::Identifier("age".into()),
        Token::Equal,
        Token::Identifier("age".into()),
        Token::SemiColon,
        Token::RightBrace,
        Token::RightBrace,
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
