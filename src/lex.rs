use std::{array::IntoIter, vec};

use crate::token::{match_identifier, Token};

#[derive(Debug)]
pub enum TokenError {
    MissingInputError,
    MissingTokensError,
}

pub struct Lex {
    input: String,
}

impl Lex {
    pub fn new(input: String) -> Self {
        Lex { input }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, TokenError> {
        let mut tokens: Vec<Token> = vec![];

        for line in self.input.lines() {
            let line = line.trim();
            let mut char_indices = line.char_indices().peekable();

            while let Some((_pos, ch)) = char_indices.next() {
                let token = match ch {
                    ' ' => continue,
                    '+' | '-' | '*' | '/' | '&' | '|' | '<' | '>' | '=' | '{' | '}' | '(' | ')'
                    | '[' | ']' | '.' | ',' | ';' | '~' => Token::Symbol(ch),
                    ch if ch.is_ascii_alphabetic() || ch == '_' => {
                        let mut iden: String = ch.to_string();

                        while let Some((_pos_, ch)) =
                            char_indices.next_if(|(_pos_, ch)| ch.is_ascii_alphabetic())
                        {
                            iden.push(ch);
                        }
                        //
                        match_identifier(&iden).unwrap()
                    }

                    ch if ch.is_ascii_digit() => {
                        let mut digit = ch.to_string();

                        while let Some((_pos, ch)) =
                            char_indices.next_if(|(_pos, ch)| ch.is_ascii_digit())
                        {
                            digit.push(ch);
                        }

                        let parsed: i32 = digit.parse().unwrap();
                        Token::Integer(parsed)
                    }

                    '"' => {
                        let mut s = "".to_string();
                        while let Some((_pos, ch)) = char_indices.next_if(|(_pos, ch)| *ch != '"') {
                            s.push(ch);
                        }

                        let quotation_mark = char_indices.next();
                        match quotation_mark {
                            Some((_pos, ch)) if ch == '"' => Token::String(s),
                            _ => Token::Invalid("Unterminated literal".to_string()),
                        }
                    }

                    _ => Token::Invalid(format!("{}", ch)),
                };

                tokens.push(token);
            }
        }

        Ok(tokens)
    }
}
