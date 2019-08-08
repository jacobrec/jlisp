use crate::ast;

mod lexer;
mod parser;

pub fn read(input: &'static str) -> Option<ast::ASTList> {
    let lex = lexer::new(input);
    let mut parser = parser::new(lex);
    parser.parse()
}
