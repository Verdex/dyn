
use super::data::Token;

enum AccumState {
    Consume,
    SingleQuoteString(Vec<char>),
    DoubleQuoteString(Vec<char>),
    Number(Vec<char>),
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
        (_, c) if c.is_whitespace() => TokenAccum{tokens: tokens, state: AccumState::Consume},
        (_, c) if c.is_digit(10) => TokenAccum{tokens: tokens, state: AccumState::Number(vec![])},
        (_, ',') => ps(Token::Comma, tokens, AccumState::Consume),
        (_, ';') => ps(Token::SemiColon, tokens, AccumState::Consume),
        (_, '(') => ps(Token::LParen, tokens, AccumState::Consume),
        (_, ')') => ps(Token::RParen, tokens, AccumState::Consume),
        (_, '[') => ps(Token::LSquare, tokens, AccumState::Consume),
        (_, ']') => ps(Token::RSquare, tokens, AccumState::Consume),
        (_, '{') => ps(Token::LCurly, tokens, AccumState::Consume),
        (_, '}') => ps(Token::RCurly, tokens, AccumState::Consume),
        (_, '\'') => TokenAccum{tokens: tokens, state: AccumState::SingleQuoteString(vec![])},
        (_, '"') => TokenAccum{tokens: tokens, state: AccumState::DoubleQuoteString(vec![])},
        _ => panic!("unknown token"),
    }
}

fn consume_until(mut buffer : Vec<char>, 
                 char_index : (usize, char), 
                 mut tokens : Vec<Token>,
                 stop : impl Fn(char) -> bool,
                 cons : impl Fn(String) -> Token,
                 next : impl Fn(Vec<Token>, Vec<char>) -> TokenAccum ) -> TokenAccum {

    let (_,c) = char_index;
    if stop(c) {
        tokens.push(cons(buffer.into_iter().collect()));
        TokenAccum{ tokens: tokens, state: AccumState::Consume }
    }
    else {
        buffer.push(c);
        next(tokens, buffer)
    }
}

fn lex_next(ta : TokenAccum, char_index : (usize, char)) -> TokenAccum {
    match ta.state {
        AccumState::Consume => consume(ta.tokens, char_index),
        AccumState::SingleQuoteString(buffer) => 
            consume_until(buffer, 
                          char_index, 
                          ta.tokens,
                          |c| c == '\'',
                          |s| Token::TString(s),
                          |ts, cs| TokenAccum {tokens : ts, state: AccumState::SingleQuoteString(cs)} ),
        AccumState::DoubleQuoteString(buffer) => 
            consume_until(buffer, 
                          char_index, 
                          ta.tokens,
                          |c| c == '"',
                          |s| Token::TString(s),
                          |ts, cs| TokenAccum {tokens : ts, state: AccumState::DoubleQuoteString(cs)} ),
        AccumState::Number(buffer) => 
            consume_until(buffer, 
                          char_index, 
                          ta.tokens,
                          |c| !c.is_digit(10) && c != '.',
                          |s| Token::Number(s),
                          |ts, cs| TokenAccum {tokens : ts, state: AccumState::Number(cs)} ),
    }
}

pub fn lex(text : &str) -> Vec<Token> {
    let words = text.char_indices();
    let o = words.fold(TokenAccum::new(), lex_next);

    o.tokens
}