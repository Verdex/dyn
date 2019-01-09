
mod data;
mod lexer;

fn x(s : i32, i:&i32) -> i32 {
    s + *i
}

fn main() {
    let z : Vec<i32> = vec![1,2,3];
    println!( "{}", z.iter().fold(0i32,x) ); 
    println!("{:?}", lexer::lex("bla 1233 *"));
}
