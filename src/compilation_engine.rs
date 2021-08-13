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
    CannotParseToken,
}

pub struct CompilationEngine<'a> {
    iter: &'a mut Peekable<Iter<'a, Token>>,
}

impl<'a> CompilationEngine<'a> {
    pub fn new(iter: &'a mut Peekable<Iter<'a, Token>>) -> Self {
        CompilationEngine { iter }
    }

    pub fn parse(&mut self) -> Result<String, ParseError> {
        let mut result;
        while let Some(token) = self.iter.next() {
            result = match token {
                Token::Var => self.compileVarDec(),
                Token::Integer(_) => self.compileTerm(token),
                Token::String(_) => self.compileTerm(token),
                Token::Let => self.compileLet(),
                _ => Err(ParseError::CannotParseToken),
            };
        }

        result
    }

    fn compileClass(&mut self) {}

    fn compileClassVarDec(&mut self) {}

    fn compileSubroutine(&mut self) {}

    fn compileParameterList(&mut self) {}

    fn compileVarDec(&mut self) -> Result<String, ParseError> {
        // var int num, num2, num3;

        let typeVal = match self.iter.next() {
            Some(Token::Int) => {
                format!("int")
            }

            Some(Token::Char) => {
                format!("char")
            }
            Some(Token::Boolean) => {
                format!("boolean")
            }
            None | _ => "".to_string(),
        };

        let varName = match self.iter.next() {
            Some(Token::Identifier(i)) => {
                format!("{}", i)
            }
            _ | None => "".to_string(),
        };

        let mut concatenated = "".to_string();
        while let Some(token) = self.iter.next_if(|token| **token != Token::SemiColon) {
            match token {
                Token::Comma => {
                    let symbol = format!("<symbol>,</symbol>");
                    concatenated.push_str(symbol.as_str());
                }
                Token::Identifier(i) => {
                    let varName = format!("<varName>{}</varName>", i);
                    concatenated.push_str(varName.as_str());
                }
                _ => panic!("Token does not exist"),
            }
        }
        let _token = self.iter.next();

        let xml = format!(
            "<varDec>
            <keyword>
                var
            </keyword>
            <type>
            {}
            </type>
        </varDec>
        <varName>
        {}
        </varName>
        {}
        ",
            typeVal, varName, concatenated
        );

        Ok(xml)
    }

    fn compileStatements(&mut self) {}

    fn compileLet(&mut self) -> Result<String, ParseError> {
        // let num = 5;
        // let age = 1 + 2;

        let varName = match self.iter.next() {
            Some(Token::Identifier(iden)) => {
                format!("<varName>{}</varName>", iden)
            }
            None | _ => "".to_string(),
        };

        let symbol = match self.iter.next() {
            Some(Token::Equal) => {
                format!("<symbol>==</symbol>")
            }
            None | _ => "".to_string(),
        };

        let expression_result = self.compileExpression()?;

        let xml = format!(
            "<letStatement><keyword>let</keyword>{}{}{}<symbol>;</symbol></letStatement>",
            varName, symbol, expression_result
        );

        Ok(xml)
    }

    fn compileIf(&mut self) {}

    fn compileWhile(&mut self) {}

    fn compileDo(&mut self) {}

    fn compileReturn(&mut self) {}

    fn compileExpression(&mut self) -> Result<String, ParseError> {
        let token = self.iter.next().unwrap();

        self.compileTerm(token)
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
                // count or count[expression] or num() or num.getValue();
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
