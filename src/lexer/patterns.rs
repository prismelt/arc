use super::lexer::Lexer;
use super::token::{Token, TokenKind};
use super::traits::LexerTrait;
use crate::types::constants::*;
use fancy_regex::Regex;

pub struct RegexPattern<L: LexerTrait> {
    pub regex: Regex,
    pub handler: Box<dyn Fn(&mut L, &Regex)>,
}

impl<L: LexerTrait> RegexPattern<L> {
    pub fn new(regex: Regex, handler: Box<dyn Fn(&mut L, &Regex)>) -> Self {
        Self { regex, handler }
    }

    pub fn non_capture_handler(kind: TokenKind) -> Box<dyn Fn(&mut L, &Regex)> {
        Box::new(move |lexer: &mut L, regex: &Regex| {
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

    pub fn capture_handler(kind: TokenKind) -> Box<dyn Fn(&mut L, &Regex)> {
        Box::new(move |lexer: &mut L, regex: &Regex| {
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

    pub fn skip_handler() -> Box<dyn Fn(&mut Lexer, &Regex)> {
        Box::new(move |lexer: &mut Lexer, regex: &Regex| {
            let matched = regex.find(lexer.reminder()).unwrap().unwrap();
            if matched.range().len() == 0 {
                panic!("Lexer: skip_handler: zero length match");
            }
            lexer.advance_n(matched.range().len());
        })
    }

    pub fn definition_handler(delimiter: String) -> Box<dyn Fn(&mut Lexer, &Regex)> {
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

    pub fn string_handler() -> Box<dyn Fn(&mut L, &Regex)> {
        Box::new(move |lexer: &mut L, regex: &Regex| {
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

    pub fn get_full_regex() -> [RegexPattern<Lexer>; 19] {
        [
            RegexPattern::new(
                Regex::new(NEWLINE_REGEX).unwrap(),
                RegexPattern::non_capture_handler(TokenKind::EndOfLine),
            ),
            RegexPattern::new(
                Regex::new(TABLE_CONTAINER_REGEX).unwrap(),
                RegexPattern::capture_handler(TokenKind::Table),
            ),
            RegexPattern::new(
                Regex::new(BLOCK_MATH_REGEX).unwrap(),
                RegexPattern::capture_handler(TokenKind::BlockMath),
            ),
            RegexPattern::new(
                Regex::new(INLINE_MATH_REGEX).unwrap(),
                RegexPattern::capture_handler(TokenKind::InlineMath),
            ),
            RegexPattern::new(
                Regex::new(WHITESPACE_REGEX).unwrap(),
                RegexPattern::<Lexer>::skip_handler(),
            ),
            RegexPattern::new(
                Regex::new(LINK_REGEX).unwrap(),
                RegexPattern::capture_handler(TokenKind::Link),
            ),
            RegexPattern::new(
                Regex::new(DEFINITION_REGEX).unwrap(),
                RegexPattern::<Lexer>::definition_handler(String::from("-@[]")),
            ),
            RegexPattern::new(
                Regex::new(CHARACTER_STYLE_REGEX).unwrap(),
                RegexPattern::capture_handler(TokenKind::CharacterStyle),
            ),
            RegexPattern::new(
                Regex::new(META_DATA_REGEX_LONG).unwrap(),
                RegexPattern::capture_handler(TokenKind::MetaData),
            ),
            RegexPattern::new(
                Regex::new(META_DATA_REGEX_SHORT).unwrap(),
                RegexPattern::capture_handler(TokenKind::MetaData),
            ),
            RegexPattern::new(
                Regex::new(LITERAL_RIGHT_PARENTHESIS_REGEX).unwrap(),
                RegexPattern::non_capture_handler(TokenKind::LiteralRightParenthesis),
            ),
            RegexPattern::new(
                Regex::new(BACKSLASH_LEFT_PARENTHESIS_INLINE_REGEX).unwrap(),
                RegexPattern::non_capture_handler(TokenKind::BackSlashLeftParenthesisInline),
            ),
            RegexPattern::new(
                Regex::new(BOLD_REGEX).unwrap(),
                RegexPattern::capture_handler(TokenKind::Bold),
            ),
            RegexPattern::new(
                Regex::new(HEADING_REGEX).unwrap(),
                RegexPattern::capture_handler(TokenKind::Heading),
            ),
            RegexPattern::new(
                Regex::new(ORDERED_LIST_REGEX).unwrap(),
                RegexPattern::non_capture_handler(TokenKind::OrderedList),
            ),
            RegexPattern::new(
                Regex::new(UNORDERED_LIST_REGEX).unwrap(),
                RegexPattern::non_capture_handler(TokenKind::UnorderedList),
            ),
            RegexPattern::new(
                Regex::new(ITALIC_REGEX).unwrap(),
                RegexPattern::non_capture_handler(TokenKind::Italic),
            ),
            RegexPattern::new(
                Regex::new(RIGHT_PARENTHESIS_REGEX).unwrap(),
                RegexPattern::non_capture_handler(TokenKind::RightParenthesis),
            ),
            RegexPattern::new(
                Regex::new(STRING_REGEX).unwrap(),
                RegexPattern::string_handler(),
            ),
        ]
    }

    pub fn get_inline_regex() -> [RegexPattern<L>; 11] {
        [
            RegexPattern::new(
                Regex::new(NEWLINE_REGEX).unwrap(),
                RegexPattern::non_capture_handler(TokenKind::EndOfLine),
            ),
            RegexPattern::new(
                Regex::new(INLINE_MATH_REGEX).unwrap(),
                RegexPattern::capture_handler(TokenKind::InlineMath),
            ),
            RegexPattern::new(
                Regex::new(BLOCK_MATH_REGEX).unwrap(),
                RegexPattern::capture_handler(TokenKind::BlockMath),
            ),
            RegexPattern::new(
                Regex::new(LINK_REGEX).unwrap(),
                RegexPattern::capture_handler(TokenKind::Link),
            ),
            RegexPattern::new(
                Regex::new(CHARACTER_STYLE_REGEX).unwrap(),
                RegexPattern::capture_handler(TokenKind::CharacterStyle),
            ),
            RegexPattern::new(
                Regex::new(LITERAL_RIGHT_PARENTHESIS_REGEX).unwrap(),
                RegexPattern::non_capture_handler(TokenKind::LiteralRightParenthesis),
            ),
            RegexPattern::new(
                Regex::new(BACKSLASH_LEFT_PARENTHESIS_INLINE_REGEX).unwrap(),
                RegexPattern::non_capture_handler(TokenKind::BackSlashLeftParenthesisInline),
            ),
            RegexPattern::new(
                Regex::new(BOLD_REGEX).unwrap(),
                RegexPattern::capture_handler(TokenKind::Bold),
            ),
            RegexPattern::new(
                Regex::new(ITALIC_REGEX).unwrap(),
                RegexPattern::non_capture_handler(TokenKind::Italic),
            ),
            RegexPattern::new(
                Regex::new(RIGHT_PARENTHESIS_REGEX).unwrap(),
                RegexPattern::non_capture_handler(TokenKind::RightParenthesis),
            ),
            RegexPattern::new(
                Regex::new(STRING_REGEX).unwrap(),
                RegexPattern::string_handler(),
            ),
        ]
    }
}
