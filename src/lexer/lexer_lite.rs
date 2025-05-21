use super::patterns::RegexPattern;
use super::token::{Token, TokenKind};
use super::traits::LexerTrait;

pub struct LexerLite {
    pub token: Vec<Token>,
    source: String,
    position: usize,
}

impl LexerTrait for LexerLite {
    fn new(source: String) -> Self {
        Self {
            token: Vec::new(),
            source,
            position: 0,
        }
    }

    fn tokenize(mut self) -> Vec<Token> {
        let patterns = RegexPattern::<LexerLite>::get_inline_regex();

        'outer: while !self.at_eof() {
            let reminder = self.reminder();

            for pattern in &patterns {
                if let Ok(Some(loc)) = pattern.regex.find(&reminder) {
                    if loc.start() == 0 {
                        let matched_str = &pattern.regex;
                        if matched_str.as_str().len() == 0 {
                            panic!("Lexer: tokenize: zero length match");
                        }
                        // println!("Matched: {:?}", pattern.regex);
                        (pattern.handler)(&mut self, matched_str);
                        continue 'outer;
                    }
                }
            }
            panic!(
                "No pattern matched at position {}, reminder: {}",
                self.position, reminder
            );
        }
        self.push(Token::new(TokenKind::EOF, None));
        self.token
    }
    fn reminder(&self) -> &str {
        &self.source[self.position..]
    }
    fn advance_n(&mut self, n: usize) {
        self.position += n;
    }
    fn push(&mut self, token: Token) {
        self.token.push(token);
    }
}

impl LexerLite {
    fn at_eof(&self) -> bool {
        self.position >= self.source.len()
    }
}
