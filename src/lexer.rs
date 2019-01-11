
use super::data::Token;

enum AccumState {
    Consume,
    Other,
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

fn consume(mut tokens : Vec<Token>, char_index : (usize, char)) -> TokenAccum {
    let z = |token| {
        tokens.push(token); 
        TokenAccum {tokens: tokens, state: AccumState::Consume}
    };
    match char_index {
        (_, ',') => {
            //tokens.push(Token::Comma); 
            //TokenAccum {tokens: tokens, state: AccumState::Consume}
            z(Token::Comma) 
        }
        _ => panic!("unknown token"),
    }
}

fn lex_next(ta : TokenAccum, char_index : (usize, char)) -> TokenAccum {
    match ta.state {
        AccumState::Consume => consume(ta.tokens, char_index),
        AccumState::Other => consume(ta.tokens, char_index),
    }
}

pub fn lex(text : &str) -> Vec<Token> {
    let words = text.char_indices();
    let o = words.fold(TokenAccum::new(), lex_next);

    vec! [Token::Comma]
}