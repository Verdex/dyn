
use super::data::Token;

enum AccumState {
    Ja,
}

struct TokenAccum {
    tokens : Vec<Token>,
    state : AccumState,
}

impl TokenAccum {
}

fn lex_next(ta : TokenAccum, charIndex : (usize, char)) -> TokenAccum {
        
    TokenAccum { tokens: ta.tokens, state: AccumState::Ja }
}

pub fn lex(text : &str) -> Vec<Token> {
    let words = text.char_indices();
    let o = words.fold(TokenAccum{}, lex_next);

    vec! [Token::Comma]
}