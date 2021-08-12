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

    pub fn parse(&mut self) {
        while let Some(token) = self.iter.next() {
            match token {
                Token::Var => {
                    self.compileVarDec();
                }

                Token::Integer(_) => {
                    self.compileTerm(token);
                }
                Token::String(_) => {
                    self.compileTerm(token);
                }
                _ => {
                    panic!("Token does not exist: {:?} ", token.clone());
                }
            }
        }
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
                Token::Int => {
                    println!("int");
                }

                Token::Char => {
                    println!("char");
                }
                Token::Boolean => {
                    println!("boolean");
                }
                _ => panic!("Token does not exist"),
            }
        }
        println!("</type>");
        println!("<varName>");
        if let Some(token) = self.iter.next() {
            match token {
                Token::Identifier(i) => {
                    println!("{}", i);
                }
                _ => panic!("Token does not exist"),
            }
        }
        println!("</varName>");
        while let Some(token) = self.iter.next_if(|token| **token != Token::SemiColon) {
            match token {
                Token::Comma => {
                    println!("<symbol>");
                    println!(",");
                    println!("</symbol>");
                }
                Token::Identifier(i) => {
                    println!("<varName>");
                    println!("{}", i);
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

    fn compileExpression(&mut self) -> Result<String, ParseError> {
        todo!()
    }

    fn compileTerm(&mut self, token: &Token) -> Result<String, ParseError> {
        match token {
            Token::Integer(i) => {
                let xml = format!("<term><integerConstant>{}</integerConstant></term>", i);
                Ok(xml)
            }
            Token::String(s) => {
                let xml = format!("<term><stringConstant>{}</stringConstant></term>", s);
                Ok(xml)
            }

            Token::True => {
                let xml = format!("<term><keyword>true</keyword></term>");
                Ok(xml)
            }

            Token::False => {
                let xml = format!("<term><keyword>false</keyword></term>");
                Ok(xml)
            }

            Token::This => {
                let xml = format!("<term><keyword>this</keyword></term>");
                Ok(xml)
            }
            Token::Identifier(iden) => {
                // count or count[expression] or num() or num.getValue()
                match self.iter.next_if(|token| {
                    **token == Token::LeftBracket
                        || **token == Token::Dot
                        || **token == Token::LeftParen
                }) {
                    Token::LeftBracket => {}
                    Token::Dot => {}
                    Token::LeftParen => {}
                    None => {
                        let xml = format!("<term><varName>{}</varName></term>", iden);
                        Ok(xml)
                    }
                }
            }

            _ => panic!("Token does not exist."),
        }
    }

    fn compileExpressionList(&mut self) {}
}

fn execute_test(input: &str) {
    let mut lex = Lex::new(input.into());
    let tokens = lex.tokenize().unwrap();
    let mut iter = tokens.iter().peekable();
    let mut engine = CompilationEngine::new(&mut iter);

    engine.parse();
}

#[test]
fn test_compilation_engine() {
    let input = "var int age;";

    execute_test(input);
}

#[test]
fn test_compilation_engine2() {
    let input = "var int age, num, test;";

    execute_test(input);
}
