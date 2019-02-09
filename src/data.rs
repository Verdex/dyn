
#[derive(Debug)]
pub enum Token {
    Comma,
    SemiColon,
    LParen,
    RParen,
    LCurly,
    RCurly,
    LSquare,
    RSquare,
    String(String),
    Number(String),
    Symbol(String),
    Op(String),
}