
mod data;
mod lexer;

fn main() {
    println!("{:?}", lexer::lex(r#",,;;[](
         {}) ))
         'blar x " ()'
         "#));
}
