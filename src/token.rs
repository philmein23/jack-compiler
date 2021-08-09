#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(String),
    Symbol(char),
    Integer(i32),
    String(String),
    Identifier(String),
    Invalid(String),
}

pub fn match_identifier(iden: &str) -> Option<Token> {
    match iden {
        "class" | "constructor" | "function" | "method" | "field" | "static" | "var" | "int"
        | "char" | "boolean" | "void" | "true" | "false" | "null" | "this" | "let" | "do"
        | "if" | "else" | "while" | "return" => Some(Token::Keyword(iden.to_string())),

        _ => Some(Token::Identifier(iden.to_string())),
    }
}
