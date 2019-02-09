
extern crate regex;

mod data;
mod lexer;
mod parser;

fn main() {
    println!("{:?}", lexer::lex(r#",,;;[](
         {}) ))
         'blar x " ()'
         " blah ' othe 4"
         123.4
         symbol
         blarg ikky
         _123
         //12 blah /
         *+
         +
         _blah_13blah
         "#));
}
