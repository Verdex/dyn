
use super::data::Token;

enum AccumState {
    Consume,
    Comment,
    SingleQuoteString(Vec<char>),
    DoubleQuoteString(Vec<char>),
    Number(Vec<char>),
    Symbol(Vec<char>),
    BinOp(Vec<char>),
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

const BIN_OP_CHARS : [char;18] = 
                                [ '.' 
                                , '*'
                                , '+'
                                , '-'
                                , '/'
                                , '~'
                                , '='
                                , '^'
                                , '%'
                                , '$'
                                , '?'
                                , '&'
                                , '|'
                                , '<'
                                , '>'
                                , ':'
                                , '@'
                                , '!'
                                ];

fn consume(tokens : Vec<Token>, char_index : (usize, char)) -> TokenAccum {
    let ps = |t : Token , mut ts : Vec<Token> , s : AccumState| -> TokenAccum {
        ts.push(t); 
        TokenAccum {tokens: ts, state: s}
    };
    
    let (_,b) = char_index;
    let bin_op = BIN_OP_CHARS.iter().any(|boc| *boc == b);
    
    match char_index {
        (_, c) if bin_op => 
            TokenAccum{tokens: tokens, state: AccumState::BinOp(vec![c])},

        (_, c) if c.is_whitespace() => TokenAccum{tokens: tokens, state: AccumState::Consume},
        (_, c) if c.is_digit(10) => TokenAccum{tokens: tokens, state: AccumState::Number(vec![c])},
        (_, c) if c.is_alphabetic() || c == '_' => 
            TokenAccum{tokens: tokens, state: AccumState::Symbol(vec![c])},

        (_, '#') => TokenAccum{tokens: tokens, state: AccumState::Comment},
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
                 cons : impl Fn(String) -> Option<Token>,
                 next : impl Fn(Vec<Token>, Vec<char>) -> TokenAccum ) -> TokenAccum {

    let (_,c) = char_index;
    if stop(c) {
        let mt = cons(buffer.into_iter().collect());
        match mt {
            Some(t) => tokens.push(t),
            None => (),
        } 
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
                          |s| Some(Token::String(s)),
                          |ts, cs| TokenAccum {tokens : ts, state: AccumState::SingleQuoteString(cs)} ),
        AccumState::DoubleQuoteString(buffer) => 
            consume_until(buffer, 
                          char_index, 
                          ta.tokens,
                          |c| c == '"',
                          |s| Some(Token::String(s)),
                          |ts, cs| TokenAccum {tokens : ts, state: AccumState::DoubleQuoteString(cs)} ),
        AccumState::Number(buffer) => 
            consume_until(buffer, 
                          char_index, 
                          ta.tokens,
                          |c| !c.is_digit(10) && c != '.',
                          |s| Some(Token::Number(s)),
                          |ts, cs| TokenAccum {tokens : ts, state: AccumState::Number(cs)} ),
        AccumState::Symbol(buffer) => 
            consume_until(buffer, 
                          char_index, 
                          ta.tokens,
                          |c| !c.is_alphanumeric() && c != '_',
                          |s| Some(Token::Symbol(s)),
                          |ts, cs| TokenAccum {tokens : ts, state: AccumState::Symbol(cs)} ),
        AccumState::Comment => 
            consume_until(vec![], 
                          char_index, 
                          ta.tokens,
                          |c| c == '\n' || c == '\r',
                          |_s| None,
                          |ts, _cs| TokenAccum {tokens : ts, state: AccumState::Comment} ),
        AccumState::BinOp(buffer) => 
            consume_until(buffer, 
                          char_index, 
                          ta.tokens,
                          |c| !BIN_OP_CHARS.iter().any(|boc| *boc == c), 
                          |s| Some(Token::BinOp(s)),
                          |ts, cs| TokenAccum {tokens : ts, state: AccumState::BinOp(cs)} ),
    }
}

pub fn lex(text : &str) -> Vec<Token> {
    let cis = text.char_indices();
    let o = cis.fold(TokenAccum::new(), lex_next);
    o.tokens
}