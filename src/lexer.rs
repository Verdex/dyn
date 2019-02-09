
use super::data::Token;
use regex::Regex;

#[derive(Debug)]
enum AccumState {
    Consume,
    SingleQuoteString(Vec<char>),
    DoubleQuoteString(Vec<char>),
    Number(Vec<char>),
    Symbol(Vec<char>),
    BinOp(Vec<char>),
}

#[derive(Debug)]
struct TokenAccum {
    tokens : Vec<Token>,
    state : AccumState,
}

impl TokenAccum {
    fn new() -> TokenAccum {
        TokenAccum { tokens: vec![], state: AccumState::Consume }
    }
}

const BIN_OP_CHARS : [char;19] = 
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
                                , '#'
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
        let t = cons(buffer.into_iter().collect());
        tokens.push(t);
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
                          |s| Token::String(s),
                          |ts, cs| TokenAccum {tokens : ts, state: AccumState::SingleQuoteString(cs)} ),
        AccumState::DoubleQuoteString(buffer) => 
            consume_until(buffer, 
                          char_index, 
                          ta.tokens,
                          |c| c == '"',
                          |s| Token::String(s),
                          |ts, cs| TokenAccum {tokens : ts, state: AccumState::DoubleQuoteString(cs)} ),
        AccumState::Number(buffer) => 
            consume_until(buffer, 
                          char_index, 
                          ta.tokens,
                          |c| !c.is_digit(10) && c != '.',
                          |s| Token::Number(s),
                          |ts, cs| TokenAccum {tokens : ts, state: AccumState::Number(cs)} ),
        AccumState::Symbol(buffer) => 
            consume_until(buffer, 
                          char_index, 
                          ta.tokens,
                          |c| !c.is_alphanumeric() && c != '_',
                          |s| Token::Symbol(s),
                          |ts, cs| TokenAccum {tokens : ts, state: AccumState::Symbol(cs)} ),
        AccumState::BinOp(buffer) => 
            consume_until(buffer, 
                          char_index, 
                          ta.tokens,
                          |c| !BIN_OP_CHARS.iter().any(|boc| *boc == c), 
                          |s| Token::BinOp(s),
                          |ts, cs| TokenAccum {tokens : ts, state: AccumState::BinOp(cs)} ),
    }
}

pub fn lex(mut text : String) -> Vec<Token> {
    text.push(' ');
    let clear_line_comment = Regex::new(r"(?:(?m)//.*?$)|(?:/\*(?:.|\n|\r)*?\*/)").unwrap();
    let t2 = clear_line_comment.replace_all(&text, "");
    let cis = t2.char_indices();
    let o = cis.fold(TokenAccum::new(), lex_next);
    let ts = o.tokens;
    ts
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_have_stuff() {
        let text = String::from(r#",,;;[](
{}) ))
'blar x " ()'
" blah ' othe 4"
123.4
symbol
blarg ikky
_123
//12 blah /
/* bla
blah*/
*+
+
_blah_13blah
"#);
        let o = lex(text);
        assert_eq!(o.len(), 22);
    }

    #[test]
    fn should_consume_last_item_in_file() {
        let text = String::from(r#"symbol1"#);

        let o = lex(text);
        assert_eq!(o.len(), 1);
    }

    #[test]
    fn should_handle_block_comment_on_multiple_line() {
        let text = String::from(r#"symbol1
            /* symbol not present 
            symbol2*/
            "#);

        let o = lex(text);
        assert_eq!(o.len(), 1);
    }

    #[test]
    fn should_handle_block_comment() {
        let text = String::from(r#"symbol1
            /* symbol not present */
            symbol2
            "#);

        let o = lex(text);
        assert_eq!(o.len(), 2);
    }

    #[test]
    fn should_handle_line_comment() {
        let text = String::from(r#"symbol1
            // symbol not present
            symbol2
            "#);

        let o = lex(text);
        assert_eq!(o.len(), 2);
    }
}