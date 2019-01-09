
mod data;
mod lexer;

fn main() {
    println!("{:?}", lexer::lex("bla 1233 *"));
}
