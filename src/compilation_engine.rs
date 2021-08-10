use std::fmt::write;
use std::{iter::Peekable, slice::Iter};

use crate::lex::Lex;
use crate::token::Token;

enum VarDec {
    VarName(String, Type),
}

enum Type {
    Int,
    Char,
    Boolean,
    ClassName,
}

enum ParseError {
    GeneralParsingError,
}

pub struct CompilationEngine<'a> {
    iter: &'a mut Peekable<Iter<'a, Token>>,
}

impl<'a> CompilationEngine<'a> {
    pub fn new(iter: &'a mut Peekable<Iter<'a, Token>>) -> Self {
        CompilationEngine { iter }
    }

    pub fn parse(&mut self) -> Result<(), ParseError> {
        while let Some(token) = self.iter.next() {
            match token.clone() {
                Token::Keyword(val) if val == "var" => {
                    self.compileVarDec();
                }
                _ => {
                    panic!("Token does not exist: {:?} ", token.clone());
                }
            }
        }

        Ok(())
    }

    fn compileClass(&mut self) {}

    fn compileClassVarDec(&mut self) {}

    fn compileSubroutine(&mut self) {}

    fn compileParameterList(&mut self) {}

    fn compileVarDec(&mut self) {
        println!("<varDec>");
        println!("<keyword>");
        println!("var");
        println!("</keyword>");
        println!("<type>");
        if let Some(token) = self.iter.next() {
            match token {
                Token::Keyword(t) if t == "int" || t == "char" || t == "boolean" => {
                    println!("{:?}", t);
                }
                _ => panic!("Token does not exist"),
            }
        }
        println!("</type>");
        println!("<varName>");
        if let Some(token) = self.iter.next() {
            match token {
                Token::Identifier(i) => {
                    println!("{:?}", i);
                }
                _ => panic!("Token does not exist"),
            }
        }
        println!("</varName>");
        while let Some(token) = self.iter.next_if(|token| **token != Token::Symbol(';')) {
            match token {
                Token::Symbol(s) if *s == ',' => {
                    println!("<symbol>");
                    println!("{:?}", s);
                    println!("</symbol>");
                }
                Token::Identifier(i) => {
                    println!("<varName>");
                    println!("{:?}", i);
                    println!("</varName>");
                }
                _ => panic!("Token does not exist"),
            }
        }
        let _token = self.iter.next();
        println!("</varDec>")
    }

    fn compileStatements(&mut self) {}

    fn compileLet(&mut self) {}

    fn compileIf(&mut self) {}

    fn compileWhile(&mut self) {}

    fn compileDo(&mut self) {}

    fn compileReturn(&mut self) {}

    fn compileExpression(&mut self) {}

    fn compileTerm(&mut self) {}

    fn compileExpressionList(&mut self) {}
}

#[test]
fn test_compilation_engine() {
    let input = "var int age;";
    let mut lex = Lex::new(input.into());
    let tokens = lex.tokenize().unwrap();
    let mut engine = CompilationEngine::new(&mut tokens.iter().peekable());

    engine.parse();
}
