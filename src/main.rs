#[macro_use]
extern crate lalrpop_util;
mod ast;

lalrpop_mod!(pub parser); // synthesized by LALRPOP

fn main() {
    println!("{:?}", parser::TopParser::new().parse("22 - 2 + 2"));
}
