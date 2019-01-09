
use super::data::Token;


pub fn lex(text : &str) -> Vec<Token> {
    let words = text.char_indices();

    for x in words {
        println!( "{:?}", x);
    }

    vec! [Token::Comma]
}