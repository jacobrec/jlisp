
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,

    Identifier(String),
    Number(isize),
    Str(String),
}

#[derive(Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub line: usize,
}

pub struct Lexer {
    line: usize,
    chars: Box<Iterator<Item=char>>,
    cur: Option<char>,
}

pub trait Tokener {
    fn next_token(&mut self) -> Option<Token>;
}

pub fn new(src: &'static str) -> Lexer {
    let mut x = Lexer {
        line: 1,
        chars: Box::from(src.chars()),
        cur: None,
    };
    x.next();
    x
}


impl Tokener for Lexer {
    fn next_token(&mut self) -> Option<Token> {
        if let Some(c) = self.cur_no_white() {
            if c == '(' {
                self.next();
                Some(self.make_token(TokenType::LeftParen))
            } else if c == ')' {
                self.next();
                Some(self.make_token(TokenType::RightParen))
            } else if c.is_ascii_digit() {
                self.next_number()
            } else if c == '"' {
                self.next_string()
            } else {
                self.next_identifier_or_keyword()
            }
        } else {
            None
        }
    }

}

impl Lexer {
    fn next_number(&mut self) -> Option<Token> {
        let s = self.get_string_to(|c| c.is_ascii_digit());
        if let Ok(num) = s.parse::<isize>() {
            return Some(self.make_token(TokenType::Number(num)))
        }
        None
    }

    fn next_string(&mut self) -> Option<Token> {
        self.next();
        let s = self.get_string_to(|c| c != '"');
        let x = Some(self.make_token(TokenType::Str(s)));
        self.next();
        x
    }

    fn next_identifier_or_keyword(&mut self) -> Option<Token> {
        let s = self.get_string_to(|c| {
            return !c.is_whitespace() &&
                c != '(' && c != ')'
        });
        Some(self.make_token(TokenType::Identifier(s)))
    }

    fn get_string_to(&mut self, f: fn(char) -> bool) -> String {
        let mut s = String::new();
        while let Some(c) = self.cur {
            if f(c) {
                s.push(c);
                self.next();
            } else {
                break;
            }
        }
        return s
    }

    fn make_token(&self, tok: TokenType) -> Token {
        Token {
            line: self.line,
            ttype: tok,
        }
    }

    fn next(&mut self) -> Option<char> {
        self.cur = self.chars.next();
        if self.cur == Some('\n') {
            self.line += 1;
        }
        self.cur
    }

    fn cur_no_white(&mut self) -> Option<char> {
        if let Some(c) = self.cur {
            if c.is_whitespace() {
                self.next();
                return self.cur_no_white()
            }
        }
        self.cur
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::TokenType::*;

    #[test]
    fn test_1() {
        let input = "1 2 3";
        let expected = vec![Number(1), Number(2), Number(3)];
        token_test(input, expected);
    }

    #[test]
    fn test_2() {
        let input = "(+ 1 2)";
        let expected = vec![LeftParen, Identifier(String::from("+")), Number(1), Number(2), RightParen];
        token_test(input, expected);
    }

    #[test]
    fn test_3() {
        let input = r#"(+ (+ "Hello, " "World!") "...")"#;
        let expected = vec![LeftParen, Identifier(String::from("+")),
            LeftParen, Identifier(String::from("+")), Str(String::from("Hello, ")),
            Str(String::from("World!")), RightParen, Str(String::from("...")), RightParen];
        token_test(input, expected);
    }

    fn token_test(input: &'static str, expected: Vec<TokenType>) {
        let mut lex = new(input);
        for x in expected {
            assert_eq!(lex.next_token().expect("A token").ttype, x);
        }
        assert!(lex.next_token().is_none());
    }

}
