mod parser;
use parser::*;

mod lex;
use lex::*;
mod syntax;

fn main() {
    println!("{:?}", ExprParser::new().parse(Lexer::new("((((22))))")));
}
