use super::patterns::RegexPattern;
use super::token::{Token, TokenKind};
use super::traits::LexerTrait;
use crate::utilities::constants::{COMMENT_REGEX, CRLF_REGEX};
use fancy_regex::Regex;
use std::time::{Duration, Instant};

pub struct Lexer {
    pub token: Vec<Token>,
    source: String,
    position: usize,
}

impl LexerTrait for Lexer {
    fn new(source: String) -> Self {
        Self {
            token: Vec::new(),
            source,
            position: 0,
        }
    }
    fn tokenize(mut self) -> Result<Vec<Token>, String> {
        let timeout = Duration::from_secs(1);
        let start_time = Instant::now();

        self.preprocess();
        let patterns_start_of_line = RegexPattern::<Lexer>::get_full_regex();
        let patterns_not_start_of_line = RegexPattern::<Lexer>::get_inline_regex();
        let mut previous_token_is_eol = true;

        'outer: while !self.at_eof() {
            if start_time.elapsed() > timeout {
                return Err("Lexer: Tokenization timed out after 1 seconds".to_string());
            }

            let reminder = self.reminder();

            let patterns_clone: &[RegexPattern<Lexer>] = if previous_token_is_eol {
                &patterns_start_of_line
            } else {
                &patterns_not_start_of_line
            };

            for pattern in patterns_clone {
                if let Ok(Some(loc)) = pattern.regex.find(&reminder) {
                    if loc.start() == 0 {
                        let matched_str = &pattern.regex;
                        if matched_str.as_str().len() == 0 {
                            return Err("Lexer: tokenize: zero length match".to_string());
                        };
                        (pattern.handler)(&mut self, matched_str);
                        previous_token_is_eol = self
                            .token
                            .last()
                            // info: if not token in the list,then it should be treated as if it was a new line
                            .unwrap_or(&Token::new(TokenKind::EndOfLine, None))
                            .kind
                            == TokenKind::EndOfLine;
                        continue 'outer;
                    }
                }
            }
            return Err(format!(
                "Lexer: No pattern matched at position {}, reminder: {}",
                self.position, reminder
            ));
        }
        self.push(Token::new(TokenKind::EOF, None));
        Ok(self.token)
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

impl Lexer {
    fn preprocess(&mut self) {
        self.source = self.source.replace("\r\n", "\n").replace("\r", "\n");
        let crlf_regex = Regex::new(CRLF_REGEX).unwrap();
        self.source = crlf_regex.replace_all(&self.source, "").to_string();
        let comment_regex = Regex::new(COMMENT_REGEX).unwrap();
        self.source = comment_regex.replace_all(&self.source, "").to_string();
    }

    fn at_eof(&self) -> bool {
        self.position >= self.source.len()
    }
}
