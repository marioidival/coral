use itertools::{multipeek, MultiPeek};
use std::str;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Boolean(bool),
    Str(String),
    None,

    Identifier(String),

    Bang,
    BraceClose,
    BraceOpen,
    BracketClose,
    BracketOpen,
    Colon,
    Comma,
    Comment,
    Dot,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Minus,
    ParenClose,
    ParenOpen,
    Plus,
    Separator,
    Slash,
    Star,

    Def,
    Class,
    If,
    Else,
    For,
    In,
    While,
    Return,
    True,
    False,
    Not,
    And,
    Or,
    Break,
    Print,

    WhiteSpace,
    EOF,
}

#[derive(Debug)]
struct Scanner<'a> {
    position: usize,
    current_lexeme: String,
    source: MultiPeek<str::Chars<'a>>,
}

fn is_alpha(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn is_alphanumeric(c: char) -> bool {
    is_alpha(c) || c.is_numeric()
}

fn is_digit(c: char) -> bool {
    c.is_numeric()
}

impl<'a> Scanner<'a> {
    pub fn init(source: &'a str) -> Self {
        Scanner {
            position: 0,
            current_lexeme: "".into(),
            source: multipeek(source.chars()),
        }
    }

    pub fn advance(&mut self) -> Option<char> {
        let current = self.source.next();

        if let Some(c) = current {
            self.current_lexeme.push(c);
        }
        current
    }

    fn peek_check(&mut self, check: &Fn(char) -> bool) -> bool {
        self.source.reset_peek();

        match self.source.peek() {
            Some(&c) => check(c),
            None => false,
        }
    }

    pub fn peek_two(&mut self, cond1: &Fn(char) -> bool, cond2: &Fn(char) -> bool) -> bool {
        self.source.reset_peek();

        match self.source.peek() {
            Some(&c1) => match self.source.peek() {
                Some(&c2) => cond1(c1) && cond2(c2),
                None => false,
            },
            None => false,
        }
    }

    pub fn advance_while(&mut self, condition: &Fn(char) -> bool) -> () {
        while self.peek_check(condition) {
            self.advance();
        }
    }

    fn advance_if_match(&mut self, expected: char) -> bool {
        if self.peek_check(&|c| c == expected) {
            self.advance();
            true
        } else {
            false
        }
    }
    fn number(&mut self) -> Option<Token> {
        self.advance_while(&is_digit);

        if self.peek_two(&|c1| c1 == '.', &is_digit) {
            self.advance();
            self.advance_while(&is_digit)
        }
        let value = self.current_lexeme.parse::<f64>().unwrap();
        Some(Token::Number(value))
    }

    fn string(&mut self) -> Option<Token> {
        self.advance_while(&|c| c != '"' && c != '\n');

        // TODO: raise error if missing close "
        if !self.advance_if_match('"') {
            unimplemented!()
        }

        let literal_len = self.current_lexeme.len() - 2;
        let literal = self
            .current_lexeme
            .chars()
            .skip(1)
            .take(literal_len)
            .collect();
        Some(Token::Str(literal))
    }

    fn identifier(&mut self) -> Option<Token> {
        self.advance_while(&is_alphanumeric);

        match self.current_lexeme.as_ref() {
            "True" => Some(Token::Boolean(true)),
            "False" => Some(Token::Boolean(false)),
            "not" => Some(Token::Not),
            "and" => Some(Token::And),
            "or" => Some(Token::Or),
            "for" => Some(Token::For),
            "in" => Some(Token::In),
            "if" => Some(Token::If),
            "else" => Some(Token::Else),
            "while" => Some(Token::While),
            "return" => Some(Token::Return),
            "break" => Some(Token::Break),
            "def" => Some(Token::Def),
            "print" => Some(Token::Print),
            "class" => Some(Token::Class),
            identifier => Some(Token::Identifier(identifier.into())),
        }
    }

    pub fn scan_next(&mut self) -> Option<Token> {
        self.current_lexeme.clear();
        let next_char = match self.advance() {
            Some(c) => c,
            None => return None,
        };

        match next_char {
            '!' => Some(Token::Bang),
            '=' => {
                if self.advance_if_match('=') {
                    Some(Token::EqualEqual)
                } else {
                    Some(Token::Equal)
                }
            }
            '<' => {
                if self.advance_if_match('=') {
                    Some(Token::LessEqual)
                } else {
                    Some(Token::Less)
                }
            }
            '>' => {
                if self.advance_if_match('=') {
                    Some(Token::GreaterEqual)
                } else {
                    Some(Token::Greater)
                }
            }
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Star),
            '/' => Some(Token::Slash),
            '(' => Some(Token::ParenOpen),
            ')' => Some(Token::ParenClose),
            '[' => Some(Token::BracketOpen),
            ']' => Some(Token::BracketClose),
            '{' => Some(Token::BraceOpen),
            '}' => Some(Token::BraceClose),
            '#' => {
                self.advance_while(&|c| c != '\n');
                Some(Token::Comment)
            }
            '.' => Some(Token::Dot),
            ',' => Some(Token::Comma),
            ':' => Some(Token::Colon),
            '"' => self.string(),
            c if c.is_whitespace() => Some(Token::WhiteSpace),
            c if c.is_numeric() => self.number(),
            c if is_alpha(c) => self.identifier(),
            _ => Some(Token::EOF),
        }
    }
}

struct TokensIterator<'a> {
    scanner: Scanner<'a>,
}

impl<'a> Iterator for TokensIterator<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.scanner.scan_next()
    }
}

fn scan_into_iterator<'a>(source: &'a str) -> impl Iterator<Item = Token> + 'a {
    TokensIterator {
        scanner: Scanner::init(source),
    }
}

pub fn scan(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for result in scan_into_iterator(source) {
        match result {
            Token::WhiteSpace | Token::Comment => {}
            _ => tokens.push(result),
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advance_source() {
        let mut scanner = Scanner::init("1 + 1");
        assert_eq!(Some('1'), scanner.advance());
        assert_eq!(Some(' '), scanner.advance());
        assert_eq!(Some('+'), scanner.advance());
        assert_eq!(Some(' '), scanner.advance());
        assert_eq!(Some('1'), scanner.advance());
    }

    #[test]
    fn test_scan_source_basic_operator() {
        let mut scn = Scanner::init(r#":+-*/().,![]{}>< "#);
        assert_eq!(Some(Token::Colon), scn.scan_next());
        assert_eq!(Some(Token::Plus), scn.scan_next());
        assert_eq!(Some(Token::Minus), scn.scan_next());
        assert_eq!(Some(Token::Star), scn.scan_next());
        assert_eq!(Some(Token::Slash), scn.scan_next());
        assert_eq!(Some(Token::ParenOpen), scn.scan_next());
        assert_eq!(Some(Token::ParenClose), scn.scan_next());
        assert_eq!(Some(Token::Dot), scn.scan_next());
        assert_eq!(Some(Token::Comma), scn.scan_next());
        assert_eq!(Some(Token::Bang), scn.scan_next());
        assert_eq!(Some(Token::BracketOpen), scn.scan_next());
        assert_eq!(Some(Token::BracketClose), scn.scan_next());
        assert_eq!(Some(Token::BraceOpen), scn.scan_next());
        assert_eq!(Some(Token::BraceClose), scn.scan_next());
        assert_eq!(Some(Token::Greater), scn.scan_next());
        assert_eq!(Some(Token::Less), scn.scan_next());
        assert_eq!(Some(Token::WhiteSpace), scn.scan_next());
    }

    #[test]
    fn test_scan_source_comment() {
        let mut scn = Scanner::init("# this is a comment");
        assert_eq!(Some(Token::Comment), scn.scan_next());
        assert_eq!(None, scn.scan_next());
    }

    #[test]
    fn test_scan_source_double_operator() {
        let mut scn = Scanner::init("==");
        assert_eq!(Some(Token::EqualEqual), scn.scan_next());

        scn = Scanner::init(">=");
        assert_eq!(Some(Token::GreaterEqual), scn.scan_next());

        scn = Scanner::init("<=");
        assert_eq!(Some(Token::LessEqual), scn.scan_next());
    }

    #[test]
    fn test_scan_source_numbers() {
        let mut scn1 = Scanner::init("122");
        assert_eq!(Some(Token::Number(122.0f64)), scn1.scan_next());

        let mut scn2 = Scanner::init("12.2");
        assert_eq!(Some(Token::Number(12.2f64)), scn2.scan_next())
    }

    #[test]
    fn test_scan_strings() {
        let mut scn = Scanner::init(r#""122""#);
        assert_eq!(Some(Token::Str(String::from("122"))), scn.scan_next());
    }

    #[test]
    fn test_scan_identifiers() {
        let mut scn = Scanner::init("True");
        assert_eq!(Some(Token::Boolean(true)), scn.scan_next());

        scn = Scanner::init("False");
        assert_eq!(Some(Token::Boolean(false)), scn.scan_next());

        scn = Scanner::init("and");
        assert_eq!(Some(Token::And), scn.scan_next());

        scn = Scanner::init("or");
        assert_eq!(Some(Token::Or), scn.scan_next());

        scn = Scanner::init("for");
        assert_eq!(Some(Token::For), scn.scan_next());

        scn = Scanner::init("if");
        assert_eq!(Some(Token::If), scn.scan_next());

        scn = Scanner::init("else");
        assert_eq!(Some(Token::Else), scn.scan_next());

        scn = Scanner::init("while");
        assert_eq!(Some(Token::While), scn.scan_next());

        scn = Scanner::init("return");
        assert_eq!(Some(Token::Return), scn.scan_next());

        scn = Scanner::init("break");
        assert_eq!(Some(Token::Break), scn.scan_next());

        scn = Scanner::init("break");
        assert_eq!(Some(Token::Break), scn.scan_next());

        scn = Scanner::init("def");
        assert_eq!(Some(Token::Def), scn.scan_next());

        scn = Scanner::init("class");
        assert_eq!(Some(Token::Class), scn.scan_next());

        scn = Scanner::init("not");
        assert_eq!(Some(Token::Not), scn.scan_next());

        scn = Scanner::init("in");
        assert_eq!(Some(Token::In), scn.scan_next());

        scn = Scanner::init("print");
        assert_eq!(Some(Token::Print), scn.scan_next());

        scn = Scanner::init("a");
        assert_eq!(Some(Token::Identifier("a".into())), scn.scan_next());
    }
}

#[cfg(test)]
mod scan {
    use super::*;
    #[test]
    fn test_scan_sum() {
        let tokens = scan("1 + 1");
        assert_eq!(Token::Number(1.0f64), tokens[0]);
        assert_eq!(Token::Plus, tokens[1]);
        assert_eq!(Token::Number(1.0f64), tokens[2]);
    }

    #[test]
    fn test_scan_without_comment() {
        let tokens = scan("1 + 1 # this a comment");
        assert_eq!(Token::Number(1.0f64), tokens[0]);
        assert_eq!(Token::Plus, tokens[1]);
        assert_eq!(Token::Number(1.0f64), tokens[2]);
        assert_eq!(3, tokens.len())
    }

    #[test]
    fn test_scan_string_literal() {
        let tokens = scan(r#"name = "coral""#);
        assert_eq!(Token::Identifier("name".into()), tokens[0]);
        assert_eq!(Token::Equal, tokens[1]);
        assert_eq!(Token::Str("coral".into()), tokens[2]);
    }

    #[test]
    fn test_scan_multiline() {
        let tokens = scan(
            r#"name = "coral"
            # this is a comment
            age = 90"#,
        );
        assert_eq!(Token::Identifier("name".into()), tokens[0]);
        assert_eq!(Token::Equal, tokens[1]);
        assert_eq!(Token::Str("coral".into()), tokens[2]);
        assert_eq!(Token::Identifier("age".into()), tokens[3]);
        assert_eq!(Token::Equal, tokens[4]);
        assert_eq!(Token::Number(90.0f64), tokens[5]);
    }
}
