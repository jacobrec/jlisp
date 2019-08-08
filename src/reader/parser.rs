use crate::ast::{ASTList, AtomOrList, Atom};
use super::lexer::{Token, TokenType, Tokener};

pub struct Parser {
    lexer: Box<Tokener>,
    cur: Token,
    next: Option<Token>,
}
pub fn new(mut lex: Box<Tokener>) -> Parser {
    let cur = lex.next_token();
    let next = lex.next_token();
    Parser {
        lexer: lex,
        cur: cur.expect("input should have at least one token"),
        next,
    }
}

impl Parser {
    pub fn parse (&mut self) -> Option<ASTList> {
        return Some(ASTList::new().append(self.parse_sexp()))
    }

    fn parse_sexp(&mut self) -> AtomOrList {
        if let TokenType::LeftParen = self.cur.ttype {
            self.next();
            return AtomOrList::List(self.parse_sexp_inner(), self.cur.line)
        } else {
            return AtomOrList::Atom(self.parse_atom(), self.cur.line)
        }
    }

    fn parse_sexp_inner(&mut self) -> ASTList {
        let mut l = ASTList::new();
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

#[cfg(test)]
mod test {
    use super::*;

    struct FakeLexer {
        tokens: Vec<TokenType>,
        i: usize
    }
    impl Tokener for FakeLexer  {
        fn next_token(&mut self) -> Option<Token> {
            if self.i < self.tokens.len() {
                self.i += 1;
                let t = &self.tokens[self.i - 1];
                Some(Token { ttype: (*t).clone(), line: 0 })
            } else {
                None
            }
        }
    }

    #[test]
    fn test_1() {
        use TokenType::*;
        use crate::ast::Atom::*;
        use crate::ast::AtomOrList::*;
        let input = vec![LeftParen, Identifier(String::from("+")), Number(1), Number(2), RightParen];
        let output = ASTList::wrap(List(ASTList::new()
            .append(Atom(AIdentifier(String::from("+")), 0))
            .append(Atom(AInteger(1), 0))
            .append(Atom(AInteger(2), 0)), 0));
        do_test(input, output);
    }

    #[test]
    fn test_2() {
        use TokenType::*;
        use crate::ast::Atom::*;
        use crate::ast::AtomOrList::*;
        let input = vec![LeftParen, Identifier(String::from("+")),
            LeftParen, Identifier(String::from("*")), Number(3), Number(2), RightParen,
            Number(1), RightParen];
        let output = ASTList::wrap(List(ASTList::new()
            .append(Atom(AIdentifier(String::from("+")), 0))
            .append(List(ASTList::new()
                         .append(Atom(AIdentifier(String::from("*")), 0))
                         .append(Atom(AInteger(3), 0))
                         .append(Atom(AInteger(2), 0)), 0))
            .append(Atom(AInteger(1), 0)), 0));
        do_test(input, output);
    }

    fn do_test(input: Vec<TokenType>, output: ASTList) {
        let out = build_test(input);
        let ast = out.expect("should have parsed");
        println!(" ===============
expected:
{:?}
===============
got:
{:?}
===============
", output, ast);

        compare_ast(output, ast);
    }

    fn build_test(input: Vec<TokenType>) -> Option<ASTList> {
        let l = FakeLexer { tokens: input, i: 0 };
        let mut p = new(Box::from(l));
        p.parse()
    }

    fn compare_ast(a: ASTList, b: ASTList) {
        assert_eq!(a.len(), b.len());

        let mut ai = a.iter();
        let mut bi = b.iter();
        while let Some(av) = ai.next() {
            if let Some(bv) = bi.next() {
                compare_atom_or_list((*av).clone(), (*bv).clone());
            } else {
                panic!("If a has a value, so should b");
            }
        }
    }

    fn compare_atom_or_list(a: AtomOrList, b: AtomOrList) {
        match (a, b) {
            (AtomOrList::Atom(av, _), AtomOrList::Atom(bv, _)) => compare_atom(av, bv),
            (AtomOrList::List(av, _), AtomOrList::List(bv, _)) => compare_ast(av, bv),
            _ => panic!("an atom and a list are not equal")
        }
    }
    fn compare_atom(a: Atom, b: Atom) {
        use crate::ast::Atom::*;
        match (a, b) {
            (AString(av), AString(bv)) => assert_eq!(av, bv),
            (AInteger(av), AInteger(bv)) => assert_eq!(av, bv),
            (AIdentifier(av), AIdentifier(bv)) => assert_eq!(av, bv),
            (ATrue, ATrue) => (),
            (AFalse, AFalse) => (),
            (AInteger(av), AInteger(bv)) => assert_eq!(av, bv),
            (AList(av), AList(bv)) => unimplemented!(), // compare_ast(av, bv),
            _ => panic!("not equal"),
        }
    }
}
