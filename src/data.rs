
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
    TString(String),
    Number(f64),
}