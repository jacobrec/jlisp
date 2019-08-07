use crate::ast;

mod lexer;
mod parser;

pub fn read(input: &'static str) -> Option<ast::List> {
    let lex = lexer::new(input);
    let mut parser = parser::new(lex);
    parser.parse()
}
