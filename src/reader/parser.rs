use crate::ast::{ASTList, ASTAtom, Atom, List};
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

    fn parse_sexp(&mut self) -> ASTAtom {
        if let TokenType::LeftParen = self.cur.ttype {
            self.next();
            return (self.parse_sexp_inner(), self.cur.line)
        } else {
            return (self.parse_atom(), self.cur.line)
        }
    }

    fn parse_sexp_inner(&mut self) -> Atom {
        let mut l = List::new();
        loop {
            match self.cur.ttype {
                TokenType::RightParen => return Atom::AList(l),
                _ => {
                    l = l.append(self.parse_sexp().0);
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
        let input = vec![LeftParen, Identifier(String::from("+")), Number(1), Number(2), RightParen];
        let output = List::new()
            .append((AList(List::new()
                .append(AIdentifier(String::from("+")))
                .append(AInteger(1))
                .append(AInteger(2))
               ), 0));
        do_test(input, output);
    }

    #[test]
    fn test_2() {
        use TokenType::*;
        use crate::ast::Atom::*;
        let input = vec![LeftParen, Identifier(String::from("+")),
            LeftParen, Identifier(String::from("*")), Number(3), Number(2), RightParen,
            Number(1), RightParen];
        let output = List::new()
            .append((AList(List::new()
                .append(AIdentifier(String::from("+")))
                .append(AList(List::new()
                    .append(AIdentifier(String::from("*")))
                    .append(AInteger(3))
                    .append(AInteger(2))
                   ))
                .append(AInteger(1))
               ), 0));
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

        compare_astlist(output, ast);
    }

    fn build_test(input: Vec<TokenType>) -> Option<ASTList> {
        let l = FakeLexer { tokens: input, i: 0 };
        let mut p = new(Box::from(l));
        p.parse()
    }

    fn compare_ast(a: List<Atom>, b: List<Atom>) {
        assert_eq!(a.len(), b.len());

        let mut ai = a.iter();
        let mut bi = b.iter();
        while let Some(av) = ai.next() {
            if let Some(bv) = bi.next() {
                compare_atom((*av).clone(), (*bv).clone());
            } else {
                panic!("If a has a value, so should b");
            }
        }
    }
    fn compare_astlist(a: ASTList, b: ASTList) {
        assert_eq!(a.len(), b.len());

        let mut ai = a.iter();
        let mut bi = b.iter();
        while let Some(av) = ai.next() {
            if let Some(bv) = bi.next() {
                compare_atom((*av).clone().0, (*bv).clone().0);
            } else {
                panic!("If a has a value, so should b");
            }
        }
    }

    fn compare_atom(a: Atom, b: Atom) {
        use crate::ast::Atom::*;
        match (a, b) {
            (AList(av), AList(bv)) => compare_ast(av, bv),
            (AString(av), AString(bv)) => assert_eq!(av, bv),
            (AInteger(av), AInteger(bv)) => assert_eq!(av, bv),
            (AIdentifier(av), AIdentifier(bv)) => assert_eq!(av, bv),
            (ATrue, ATrue) => (),
            (AFalse, AFalse) => (),
            _ => panic!("not equal"),
        }
    }
}
