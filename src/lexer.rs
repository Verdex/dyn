
use super::data::Token;

enum AccumState {
    Consume,
    //SingleQuoteString(Vec<char>),
    //DoubleQuoteString,
    //Number,
}

struct TokenAccum {
    tokens : Vec<Token>,
    state : AccumState,
}

impl TokenAccum {
    fn new() -> TokenAccum {
        TokenAccum { tokens: vec![], state: AccumState::Consume }
    }
}

fn consume(tokens : Vec<Token>, char_index : (usize, char)) -> TokenAccum {
    let ps = |t : Token , mut ts : Vec<Token> , s : AccumState| -> TokenAccum {
        ts.push(t); 
        TokenAccum {tokens: ts, state: s}
    };
    
    match char_index {
        (_, ' ') => TokenAccum{tokens: tokens, state: AccumState::Consume},
        (_, '\t') => TokenAccum{tokens: tokens, state: AccumState::Consume},
        (_, '\n') => TokenAccum{tokens: tokens, state: AccumState::Consume},
        (_, '\r') => TokenAccum{tokens: tokens, state: AccumState::Consume},
        (_, ',') => ps(Token::Comma, tokens, AccumState::Consume),
        (_, ';') => ps(Token::SemiColon, tokens, AccumState::Consume),
        (_, '(') => ps(Token::LParen, tokens, AccumState::Consume),
        (_, ')') => ps(Token::RParen, tokens, AccumState::Consume),
        (_, '[') => ps(Token::LSquare, tokens, AccumState::Consume),
        (_, ']') => ps(Token::RSquare, tokens, AccumState::Consume),
        (_, '{') => ps(Token::LCurly, tokens, AccumState::Consume),
        (_, '}') => ps(Token::RCurly, tokens, AccumState::Consume),
        //(_, '\'') => TokenAccum{tokens: tokens, state: AccumState::SingleQuoteString([])},
        _ => panic!("unknown token"),
    }
}

fn lex_next(ta : TokenAccum, char_index : (usize, char)) -> TokenAccum {
    match ta.state {
        AccumState::Consume => consume(ta.tokens, char_index),
        //AccumState::SingleQuoteString => consume(ta.tokens, char_index),

    }
}

pub fn lex(text : &str) -> Vec<Token> {
    let words = text.char_indices();
    let o = words.fold(TokenAccum::new(), lex_next);

    o.tokens
}