use super::token::{Token, TokenKind};
use crate::types::constants::*;
use dyn_clone::{DynClone, clone_box};
use fancy_regex::Regex;
use std::time::{Duration, Instant};

trait LexerHandler: DynClone + Fn(&mut Lexer, &Regex) {}
impl<T> LexerHandler for T where T: DynClone + Fn(&mut Lexer, &Regex) {}
dyn_clone::clone_trait_object!(LexerHandler);

pub struct Lexer {
    pub token: Vec<Token>,
    patterns: Vec<RegexPattern>,
    source: String,
    position: usize,
}

pub struct RegexPattern {
    regex: Regex,
    handler: Box<dyn LexerHandler>,
    start_of_line: bool,
}

impl Clone for RegexPattern {
    fn clone(&self) -> Self {
        RegexPattern {
            regex: self.regex.clone(),
            handler: clone_box(&*self.handler),
            start_of_line: self.start_of_line,
        }
    }
}

impl RegexPattern {
    fn new(regex: Regex, handler: Box<dyn LexerHandler>, start_of_line: bool) -> Self {
        Self {
            regex,
            handler,
            start_of_line,
        }
    }

    fn non_capture_handler(kind: TokenKind) -> Box<dyn LexerHandler> {
        Box::new(move |lexer: &mut Lexer, regex: &Regex| {
            let reminder = lexer.reminder();
            let matched = regex.find(reminder).unwrap().unwrap();
            let length = matched.range().len();
            if length == 0 {
                panic!("Lexer: non_capture_handler: zero length match");
            }
            lexer.push(Token::new(kind.clone(), None));
            lexer.advance_n(length);
        })
    }

    fn capture_handler(kind: TokenKind) -> Box<dyn LexerHandler> {
        Box::new(move |lexer: &mut Lexer, regex: &Regex| {
            let reminder = lexer.reminder();
            let matched = regex.find(reminder).unwrap().unwrap();
            let length = matched.range().len();
            if length == 0 {
                panic!("Lexer: capture_handler: zero length match");
            }
            let matched = matched.as_str();
            let capture = regex
                .captures(matched)
                .unwrap()
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();
            lexer.push(Token::new(kind.clone(), Some(capture.to_string())));
            lexer.advance_n(length);
        })
    }

    fn skip_handler() -> Box<dyn LexerHandler> {
        Box::new(move |lexer: &mut Lexer, regex: &Regex| {
            let matched = regex.find(lexer.reminder()).unwrap().unwrap();
            if matched.range().len() == 0 {
                panic!("Lexer: skip_handler: zero length match");
            }
            lexer.advance_n(matched.range().len());
        })
    }

    fn definition_handler(delimiter: String) -> Box<dyn LexerHandler> {
        Box::new(move |lexer: &mut Lexer, regex: &Regex| {
            let reminder = lexer.reminder();
            let matched = regex.find(reminder).unwrap().unwrap();
            let length = matched.range().len();
            if length == 0 {
                panic!("Lexer: definition_handler: zero length match");
            }
            let matched = matched.as_str();
            let captures = regex.captures(matched).unwrap().unwrap();
            let words = captures.get(1).unwrap().as_str();
            let expression = captures.get(2).unwrap().as_str();
            lexer.push(Token::new(
                TokenKind::Definition,
                Some(format!("{}{}{}", words, delimiter, expression)),
            ));
            lexer.advance_n(length);
        })
    }

    fn string_handler() -> Box<dyn LexerHandler> {
        Box::new(move |lexer: &mut Lexer, regex: &Regex| {
            let reminder = lexer.reminder();
            let match_result = regex.find(reminder).unwrap().unwrap();
            let matched_slice = match_result.as_str();
            let advance_len = matched_slice.len();
            if advance_len == 0 {
                panic!("Lexer: string_handler: zero length match");
            }
            lexer.push(Token::new(
                TokenKind::String,
                Some(matched_slice.to_string()),
            ));

            lexer.advance_n(advance_len);
        })
    }
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            token: Vec::new(),
            patterns: vec![
                RegexPattern::new(
                    Regex::new(NEWLINE_REGEX).unwrap(),
                    RegexPattern::non_capture_handler(TokenKind::EndOfLine),
                    false,
                ),
                // RegexPattern::new(
                //     Regex::new(r"\\[\s]*\n").unwrap(),
                //     RegexPattern::skip_handler(),
                // ),
                RegexPattern::new(
                    Regex::new(WHITESPACE_REGEX).unwrap(),
                    RegexPattern::skip_handler(),
                    true,
                ),
                RegexPattern::new(
                    Regex::new(LINK_REGEX).unwrap(),
                    RegexPattern::capture_handler(TokenKind::Link),
                    false,
                ),
                RegexPattern::new(
                    Regex::new(DEFINITION_REGEX).unwrap(),
                    RegexPattern::definition_handler(String::from("-@[]")),
                    true,
                ),
                RegexPattern::new(
                    Regex::new(CHARACTER_STYLE_REGEX).unwrap(),
                    RegexPattern::capture_handler(TokenKind::CharacterStyle),
                    false,
                ),
                RegexPattern::new(
                    Regex::new(META_DATA_REGEX_LONG).unwrap(),
                    RegexPattern::capture_handler(TokenKind::MetaData),
                    true,
                ),
                RegexPattern::new(
                    Regex::new(META_DATA_REGEX_SHORT).unwrap(),
                    RegexPattern::capture_handler(TokenKind::MetaData),
                    true,
                ),
                RegexPattern::new(
                    Regex::new(LITERAL_RIGHT_PARENTHESIS_REGEX).unwrap(),
                    RegexPattern::non_capture_handler(TokenKind::LiteralRightParenthesis),
                    false,
                ),
                RegexPattern::new(
                    Regex::new(BACKSLASH_LEFT_PARENTHESIS_INLINE_REGEX).unwrap(),
                    RegexPattern::non_capture_handler(TokenKind::BackSlashLeftParenthesisInline),
                    false,
                ),
                RegexPattern::new(
                    Regex::new(BOLD_REGEX).unwrap(),
                    RegexPattern::capture_handler(TokenKind::Bold),
                    false,
                ),
                RegexPattern::new(
                    Regex::new(HEADING_REGEX).unwrap(),
                    RegexPattern::capture_handler(TokenKind::Heading),
                    true,
                ),
                RegexPattern::new(
                    Regex::new(ORDERED_LIST_REGEX).unwrap(),
                    RegexPattern::non_capture_handler(TokenKind::OrderedList),
                    true,
                ),
                RegexPattern::new(
                    Regex::new(UNORDERED_LIST_REGEX).unwrap(),
                    RegexPattern::non_capture_handler(TokenKind::UnorderedList),
                    true,
                ),
                RegexPattern::new(
                    Regex::new(ITALIC_REGEX).unwrap(),
                    RegexPattern::non_capture_handler(TokenKind::Italic),
                    false,
                ),
                RegexPattern::new(
                    Regex::new(RIGHT_PARENTHESIS_REGEX).unwrap(),
                    RegexPattern::non_capture_handler(TokenKind::RightParenthesis),
                    false,
                ),
                RegexPattern::new(
                    Regex::new(STRING_REGEX).unwrap(),
                    RegexPattern::string_handler(),
                    false,
                ),
            ],
            source,
            position: 0,
        }
    }
    pub fn tokenize(mut self) -> Vec<Token> {
        let timeout = Duration::from_secs(1);
        let start_time = Instant::now();

        self.preprocess();
        let patterns_start_of_line = self.patterns.clone();
        let patterns_not_start_of_line = self
            .patterns
            .iter()
            .filter(|p| !p.start_of_line)
            .cloned()
            .collect::<Vec<RegexPattern>>();
        let mut previous_token_is_eol = true;

        'outer: while !self.at_eof() {
            if start_time.elapsed() > timeout {
                panic!("Tokenization timed out after 1 seconds");
            }

            let mut reminder = String::new();
            self.reminder().clone_into(&mut reminder);

            let patterns_clone = if previous_token_is_eol {
                &patterns_start_of_line
            } else {
                &patterns_not_start_of_line
            };

            for pattern in patterns_clone {
                if let Ok(Some(loc)) = pattern.regex.find(&reminder) {
                    if loc.start() == 0 {
                        let matched_str = &pattern.regex;
                        if matched_str.as_str().len() == 0 {
                            panic!("Lexer: tokenize: zero length match");
                        }
                        // println!("Matched: {:?}", pattern.regex);
                        (pattern.handler)(&mut self, matched_str);
                        previous_token_is_eol =
                            self.token.last().unwrap().kind == TokenKind::EndOfLine;
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

    pub fn preprocess(&mut self) {
        self.source = self.source.replace("\r\n", "\n").replace("\r", "\n");
        let crlf_regex = Regex::new(CRLF_REGEX).unwrap();
        self.source = crlf_regex.replace_all(&self.source, "").to_string();
        let comment_regex = Regex::new(COMMENT_REGEX).unwrap();
        self.source = comment_regex.replace_all(&self.source, "").to_string();
    }

    fn advance_n(&mut self, n: usize) {
        self.position += n;
    }
    fn push(&mut self, token: Token) {
        self.token.push(token);
    }
    fn at_eof(&self) -> bool {
        self.position >= self.source.len()
    }
    fn reminder(&self) -> &str {
        &self.source[self.position..]
    }
}

// use super::token::{IdentifierKind, Token, TokenKind};
// use dyn_clone::{DynClone, clone_box};
// use regex;

// // make the RegexPattern cloneable
// trait LexerHandler: DynClone + Fn(&mut Lexer, &regex::Regex) {}

// impl<T> LexerHandler for T where T: DynClone + Fn(&mut Lexer, &regex::Regex) {}

// dyn_clone::clone_trait_object!(LexerHandler);

// pub struct Lexer {
//     pub token: Vec<Token>,
//     patterns: Vec<RegexPattern>,
//     source: String,
//     position: usize,
// }
// pub struct RegexPattern {
//     regex: regex::Regex,
//     handler: Box<dyn LexerHandler>,
// }

// impl RegexPattern {
//     fn new(regex: regex::Regex, handler: Box<dyn LexerHandler>) -> Self {
//         Self { regex, handler }
//     }
//     fn default_handler(kind: TokenKind, value: String) -> Box<dyn LexerHandler> {
//         Box::new(move |lexer: &mut Lexer, regex: &regex::Regex| {
//             let matched = regex.find(lexer.reminder()).unwrap();
//             lexer.advance_n(matched.len());
//             lexer.push(Token::new(kind.clone(), Some(value.clone())));
//         })
//     }
//     fn skip_handler() -> Box<dyn LexerHandler> {
//         Box::new(move |lexer: &mut Lexer, regex: &regex::Regex| {
//             let _ = regex.find(lexer.reminder()).unwrap();
//             lexer.advance_n(regex.as_str().len());
//         })
//     }
//     fn symbol_handler() -> Box<dyn LexerHandler> {
//         Box::new(move |lexer: &mut Lexer, regex: &regex::Regex| {
//             let reminder = lexer.reminder();
//             let matched = regex.find(reminder).unwrap();
//             let length = matched.len();
//             let matched = matched.as_str();
//             let token_kind = TokenKind::Identifier(IdentifierKind::new(matched));
//             lexer.push(Token::new(token_kind, Some(matched.to_string())));
//             lexer.advance_n(length);
//         })
//     }

//     fn string_handler() -> Box<dyn LexerHandler> {
//         Box::new(move |lexer: &mut Lexer, regex: &regex::Regex| {
//             let reminder = lexer.reminder();
//             let match_result = regex.find(reminder).unwrap();
//             let matched_slice = match_result.as_str();
//             let advance_len = matched_slice.len();

//             let token_value = if let Some((value, _)) = matched_slice.split_once("//") {
//                 Some(value.to_string())
//             } else {
//                 Some(matched_slice.to_string())
//             };

//             lexer.push(Token::new(TokenKind::String, token_value));

//             lexer.advance_n(advance_len);
//         })
//     }
// }

// impl Clone for RegexPattern {
//     fn clone(&self) -> Self {
//         RegexPattern {
//             regex: self.regex.clone(),
//             handler: clone_box(&*self.handler),
//         }
//     }
// }

// impl Lexer {
//     pub fn new(source: String) -> Self {
//         Self {
//             token: Vec::new(),
//             patterns: vec![
//                 RegexPattern::new(
//                     regex::Regex::new(r"meta").unwrap(),
//                     RegexPattern::symbol_handler(),
//                 ),
//                 RegexPattern::new(
//                     regex::Regex::new(r"\*\*").unwrap(),
//                     RegexPattern::default_handler(TokenKind::DoubleAsterisk, "**".to_string()),
//                 ),
//                 RegexPattern::new(
//                     regex::Regex::new(r"\\\(").unwrap(),
//                     RegexPattern::default_handler(
//                         TokenKind::BackSlashLeftParenthesisInline,
//                         "\\(".to_string(),
//                     ),
//                 ),
//                 RegexPattern::new(
//                     regex::Regex::new(r" ").unwrap(),
//                     RegexPattern::skip_handler(),
//                 ),
//                 RegexPattern::new(
//                     regex::Regex::new(r"\n").unwrap(),
//                     RegexPattern::default_handler(TokenKind::EndOfLine, r"\n".to_string()),
//                 ),
//                 RegexPattern::new(
//                     regex::Regex::new(r"\[").unwrap(),
//                     RegexPattern::default_handler(TokenKind::LeftBracket, "[".to_string()),
//                 ),
//                 RegexPattern::new(
//                     regex::Regex::new(r"\]").unwrap(),
//                     RegexPattern::default_handler(TokenKind::RightBracket, "]".to_string()),
//                 ),
//                 RegexPattern::new(
//                     regex::Regex::new(r"\(").unwrap(),
//                     RegexPattern::default_handler(TokenKind::LeftParenthesis, "(".to_string()),
//                 ),
//                 RegexPattern::new(
//                     regex::Regex::new(r"\)").unwrap(),
//                     RegexPattern::default_handler(TokenKind::RightParenthesis, ")".to_string()),
//                 ),
//                 RegexPattern::new(
//                     regex::Regex::new(r"\\").unwrap(),
//                     RegexPattern::default_handler(TokenKind::BackSlash, r"\".to_string()),
//                 ),
//                 RegexPattern::new(
//                     regex::Regex::new(r"/").unwrap(),
//                     RegexPattern::default_handler(TokenKind::ForwardSlash, "/".to_string()),
//                 ),
//                 RegexPattern::new(
//                     regex::Regex::new(r"#").unwrap(),
//                     RegexPattern::default_handler(TokenKind::Octothorpe, "#".to_string()),
//                 ),
//                 RegexPattern::new(
//                     regex::Regex::new(r"@").unwrap(),
//                     RegexPattern::default_handler(TokenKind::At, "@".to_string()),
//                 ),
//                 RegexPattern::new(
//                     regex::Regex::new(r"&").unwrap(),
//                     RegexPattern::default_handler(TokenKind::Ampersand, "&".to_string()),
//                 ),
//                 RegexPattern::new(
//                     regex::Regex::new(r"%").unwrap(),
//                     RegexPattern::default_handler(TokenKind::Percent, "%".to_string()),
//                 ),
//                 RegexPattern::new(
//                     regex::Regex::new(r"~").unwrap(),
//                     RegexPattern::default_handler(TokenKind::Tilde, "~".to_string()),
//                 ),
//                 RegexPattern::new(
//                     regex::Regex::new(r"=").unwrap(),
//                     RegexPattern::default_handler(TokenKind::Equal, "=".to_string()),
//                 ),
//                 RegexPattern::new(
//                     regex::Regex::new(r"<").unwrap(),
//                     RegexPattern::default_handler(TokenKind::LeftAngleBracket, "<".to_string()),
//                 ),
//                 RegexPattern::new(
//                     regex::Regex::new(r">").unwrap(),
//                     RegexPattern::default_handler(TokenKind::RightAngleBracket, ">".to_string()),
//                 ),
//                 RegexPattern::new(
//                     regex::Regex::new(r",").unwrap(),
//                     RegexPattern::default_handler(TokenKind::Comma, ",".to_string()),
//                 ),
//                 RegexPattern::new(
//                     regex::Regex::new(r":").unwrap(),
//                     RegexPattern::default_handler(TokenKind::Colon, ":".to_string()),
//                 ),
//                 RegexPattern::new(
//                     regex::Regex::new(r"'").unwrap(),
//                     RegexPattern::default_handler(TokenKind::SingleQuote, "'".to_string()),
//                 ),
//                 RegexPattern::new(
//                     regex::Regex::new(r"-").unwrap(),
//                     RegexPattern::default_handler(TokenKind::Hyphen, "-".to_string()),
//                 ),
//                 RegexPattern::new(
//                     regex::Regex::new(r"(?:[^\n\\)]*\\(?:[^()\n]))*[^\n\\)]*").unwrap(),
//                     RegexPattern::string_handler(),
//                 ),
//             ],
//             source,
//             position: 0,
//         }
//     }

//     fn advance_n(&mut self, n: usize) {
//         self.position += n;
//     }

//     fn push(&mut self, token: Token) {
//         self.token.push(token);
//     }

//     fn at_eof(&self) -> bool {
//         self.position >= self.source.len()
//     }

//     fn reminder(&self) -> &str {
//         &self.source[self.position..]
//     }

//     fn at(&self) -> char {
//         self.source.as_bytes()[self.position] as char
//     }

//     pub fn tokenize(mut self) -> Vec<Token> {
//         self.preprocess();
//         let patterns_clone = self.patterns.clone();
//         while !self.at_eof() {
//             let mut matched = false;
//             let mut reminder = String::new();
//             self.reminder().clone_into(&mut reminder);

//             for pattern in &patterns_clone {
//                 if let Some(loc) = pattern.regex.find(reminder.as_str()) {
//                     if loc.start() == 0 {
//                         matched = true;
//                         let matched_str = &pattern.regex;
//                         (pattern.handler)(&mut self, matched_str);
//                         break;
//                     }
//                 }
//             }
//             if !matched {
//                 panic!(
//                     "No pattern matched at position {}, reminder: {}",
//                     self.position, reminder
//                 );
//             }
//         }
//         self.push(Token::new(TokenKind::EOF, None));
//         self.token
//     }

//     fn preprocess(&mut self) {
//         let lines: Vec<&str> = self.source.lines().collect();
//         let mut preprocessed_file: Vec<&str> = Vec::new();
//         for line in lines {
//             if let Some((var, _)) = line.split_once("//") {
//                 preprocessed_file.push(var);
//             } else {
//                 preprocessed_file.push(line);
//             }
//         }

//         self.source = preprocessed_file.join("\n")
//     }
// }
