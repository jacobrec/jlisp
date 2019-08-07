use crate::ast::{List, AtomOrList, Atom};
use super::lexer::{Lexer, Token, TokenType};

pub struct Parser {
    lexer: Lexer,
    cur: Token,
    next: Option<Token>,
}
pub fn new(mut lex: Lexer) -> Parser {
    let cur = lex.next_token();
    let next = lex.next_token();
    Parser {
        lexer: lex,
        cur: cur.expect("input should have at least one token"),
        next,
    }
}

impl Parser {
    pub fn parse (&mut self) -> Option<List> {
        return Some(List::new().append(self.parse_sexp()))
    }

    fn parse_sexp(&mut self) -> AtomOrList {
        if let TokenType::LeftParen = self.cur.ttype {
            self.next();
            return AtomOrList::List(self.parse_sexp_inner())
        } else {
            return AtomOrList::Atom(self.parse_atom())
        }
    }

    fn parse_sexp_inner(&mut self) -> List {
        let mut l = List::new();
        loop {
            match self.cur.ttype {
                TokenType::RightParen => return l,
                _ => {
                    l = l.append(self.parse_sexp());
                    self.next();
                }
            }
        }
    }

    fn parse_atom(&mut self) -> Atom {
        match &self.cur.ttype {
            TokenType::Str(x) => Atom::AString(x.clone()),
            TokenType::Number(x) => Atom::AInteger(*x),
            TokenType::Identifier(x) => {
                match x.as_ref() {
                    "true" => Atom::ATrue,
                    "false" => Atom::AFalse,
                    _ => Atom::AIdentifier(x.clone())
                }
            }
            _ => panic!("Not an atom"),
        }
    }

    fn next(&mut self) {
        if self.next.is_some() {
            self.cur = std::mem::replace(&mut self.next, self.lexer.next_token()).expect("");
        }
    }
}
