
mod data;
mod lexer;

fn main() {
    println!("{:?}", lexer::lex(r#",,;;[](
         {}) ))
         'blar x " ()'
         " blah ' othe 4"
         123.4
         "#));
}
