use super::lexer::Lexer;
use super::token::{Token, TokenKind};
use super::traits::LexerTrait;
use crate::utilities::constants::*;
use fancy_regex::Regex;

pub struct RegexPattern<L: LexerTrait> {
    pub regex: Regex,
    pub handler: Box<dyn Fn(&mut L, &Regex) -> Result<(), String>>,
}

impl<L: LexerTrait> RegexPattern<L> {
    pub fn new(regex: Regex, handler: Box<dyn Fn(&mut L, &Regex) -> Result<(), String>>) -> Self {
        Self { regex, handler }
    }

    pub fn non_capture_handler(
        kind: TokenKind,
    ) -> Box<dyn Fn(&mut L, &Regex) -> Result<(), String>> {
        Box::new(move |lexer: &mut L, regex: &Regex| {
            let reminder = lexer.reminder();
            let matched = regex
                .find(reminder)
                .map_err(|e| format!("Lexer: non_capture_handler receive a regex error: {}", e))?
                .expect("Lexer: passed content should contain valid structure.");
            let length = matched.range().len();
            if length == 0 {
                return Err(String::from(
                    "Lexer: non_capture_handler: zero length match",
                ));
            }
            lexer.push(Token::new(kind.clone(), None));
            lexer.advance_n(length);
            Ok(())
        })
    }

    pub fn capture_handler(kind: TokenKind) -> Box<dyn Fn(&mut L, &Regex) -> Result<(), String>> {
        Box::new(move |lexer: &mut L, regex: &Regex| {
            let reminder = lexer.reminder();
            let matched = regex
                .find(reminder)
                .map_err(|e| format!("Lexer: capture_handler receive a regex error: {}", e))?
                .expect("Lexer: passed content should contain valid structure.");
            let length = matched.range().len();
            if length == 0 {
                return Err(String::from("Lexer: capture_handler: zero length match"));
            }
            let matched = matched.as_str();
            let capture = regex
                .captures(matched)
                .map_err(|e| format!("Lexer: capture_handler receive a regex error: {}", e))?
                .expect("Lexer: passed content should contain valid structure.")
                .get(1)
                .expect("Lexer: hard coded regex pattern should have a capture group.")
                .as_str();
            lexer.push(Token::new(kind.clone(), Some(capture.to_string())));
            lexer.advance_n(length);
            Ok(())
        })
    }

    pub fn skip_handler() -> Box<dyn Fn(&mut Lexer, &Regex) -> Result<(), String>> {
        Box::new(move |lexer: &mut Lexer, regex: &Regex| {
            let matched = regex
                .find(lexer.reminder())
                .map_err(|e| format!("Lexer: skip_handler receive a regex error: {}", e))?
                .expect("Lexer: passed content should contain valid structure.");
            if matched.range().len() == 0 {
                return Err(String::from("Lexer: skip_handler: zero length match"));
            }
            lexer.advance_n(matched.range().len());
            Ok(())
        })
    }

    pub fn definition_handler(
        delimiter: String,
    ) -> Box<dyn Fn(&mut Lexer, &Regex) -> Result<(), String>> {
        Box::new(move |lexer: &mut Lexer, regex: &Regex| {
            let reminder = lexer.reminder();
            let matched = regex
                .find(reminder)
                .map_err(|e| format!("Lexer: definition_handler receive a regex error: {}", e))?
                .expect("Lexer: passed content should contain valid structure.");
            let length = matched.range().len();
            if length == 0 {
                return Err(String::from("Lexer: definition_handler: zero length match"));
            }
            let matched = matched.as_str();
            let captures = regex
                .captures(matched)
                .map_err(|e| format!("Lexer: definition_handler receive a regex error: {}", e))?
                .expect("Lexer: passed content should contain valid structure.");
            let words = captures
                .get(1)
                .expect("Lexer: hard coded definition pattern should have a capture group.")
                .as_str();
            let expression = captures
                .get(2)
                .expect(
                    "Lexer: hard coded definition pattern should have the second capture group.",
                )
                .as_str();
            lexer.push(Token::new(
                TokenKind::Definition,
                Some(format!("{}{}{}", words, delimiter, expression)),
            ));
            lexer.advance_n(length);
            Ok(())
        })
    }

    pub fn string_handler() -> Box<dyn Fn(&mut L, &Regex) -> Result<(), String>> {
        Box::new(move |lexer: &mut L, regex: &Regex| {
            let reminder = lexer.reminder();
            let match_result = regex
                .find(reminder)
                .map_err(|e| format!("Lexer: string_handler receive a regex error: {}", e))?
                .expect("Lexer: passed content should contain valid structure.");
            let matched_slice = match_result.as_str();
            let advance_len = matched_slice.len();
            if advance_len == 0 {
                return Err(String::from("Lexer: string_handler: zero length match"));
            }
            lexer.push(Token::new(
                TokenKind::String,
                Some(matched_slice.to_string()),
            ));
            lexer.advance_n(advance_len);
            Ok(())
        })
    }

    pub fn code_handler() -> Box<dyn Fn(&mut Lexer, &Regex) -> Result<(), String>> {
        Box::new(move |lexer: &mut Lexer, regex: &Regex| {
            let reminder = lexer.reminder();
            let matched = regex
                .find(reminder)
                .map_err(|e| format!("Lexer: code_handler receive a regex error: {}", e))?
                .expect("Lexer: passed content should contain valid structure.");
            let length = matched.range().len();
            if length == 0 {
                return Err(String::from("Lexer: code_handler: zero length match"));
            }
            let matched = matched.as_str();
            let captures = regex
                .captures(matched)
                .map_err(|e| format!("Lexer: code_handler receive a regex error: {}", e))?
                .expect("Lexer: passed content should contain valid structure.");
            let language = captures
                .get(1)
                .expect("Lexer: hard coded code block pattern should have a capture group.")
                .as_str();
            let content = captures
                .get(2)
                .expect(
                    "Lexer: hard coded code block pattern should have the second capture group.",
                )
                .as_str();
            lexer.push(Token::new(
                TokenKind::CodeBlock,
                Some(format!("{}{}{}", language, "\n", content)),
            ));
            lexer.advance_n(length);
            Ok(())
        })
    }

    pub fn get_full_regex() -> [RegexPattern<Lexer>; 22] {
        [
            RegexPattern::new(
                Regex::new(NEWLINE_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::non_capture_handler(TokenKind::EndOfLine),
            ),
            RegexPattern::new(
                Regex::new(TABLE_CONTAINER_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::capture_handler(TokenKind::Table),
            ),
            RegexPattern::new(
                Regex::new(HTML_CONTAINER_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::capture_handler(TokenKind::HTMLContainer),
            ),
            RegexPattern::new(
                Regex::new(HORIZONTAL_LINE_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::non_capture_handler(TokenKind::HorizontalLine),
            ),
            RegexPattern::new(
                Regex::new(BLOCK_MATH_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::capture_handler(TokenKind::BlockMath),
            ),
            RegexPattern::new(
                Regex::new(CODE_BLOCK_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::<Lexer>::code_handler(),
            ),
            RegexPattern::new(
                Regex::new(INLINE_MATH_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::capture_handler(TokenKind::InlineMath),
            ),
            RegexPattern::new(
                Regex::new(WHITESPACE_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::<Lexer>::skip_handler(),
            ),
            RegexPattern::new(
                Regex::new(LINK_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::capture_handler(TokenKind::Link),
            ),
            RegexPattern::new(
                Regex::new(DEFINITION_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::<Lexer>::definition_handler(String::from("-@[]")),
            ),
            RegexPattern::new(
                Regex::new(CHARACTER_STYLE_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::capture_handler(TokenKind::CharacterStyle),
            ),
            RegexPattern::new(
                Regex::new(META_DATA_REGEX_LONG).expect("Hard coded regex should be valid."),
                RegexPattern::capture_handler(TokenKind::MetaData),
            ),
            RegexPattern::new(
                Regex::new(META_DATA_REGEX_SHORT).expect("Hard coded regex should be valid."),
                RegexPattern::capture_handler(TokenKind::MetaData),
            ),
            RegexPattern::new(
                Regex::new(LITERAL_RIGHT_PARENTHESIS_REGEX)
                    .expect("Hard coded regex should be valid."),
                RegexPattern::non_capture_handler(TokenKind::LiteralRightParenthesis),
            ),
            RegexPattern::new(
                Regex::new(BACKSLASH_LEFT_PARENTHESIS_INLINE_REGEX)
                    .expect("Hard coded regex should be valid."),
                RegexPattern::non_capture_handler(TokenKind::BackSlashLeftParenthesisInline),
            ),
            RegexPattern::new(
                Regex::new(BOLD_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::capture_handler(TokenKind::Bold),
            ),
            RegexPattern::new(
                Regex::new(HEADING_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::capture_handler(TokenKind::Heading),
            ),
            RegexPattern::new(
                Regex::new(ORDERED_LIST_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::non_capture_handler(TokenKind::OrderedList),
            ),
            RegexPattern::new(
                Regex::new(UNORDERED_LIST_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::non_capture_handler(TokenKind::UnorderedList),
            ),
            RegexPattern::new(
                Regex::new(ITALIC_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::non_capture_handler(TokenKind::Italic),
            ),
            RegexPattern::new(
                Regex::new(RIGHT_PARENTHESIS_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::non_capture_handler(TokenKind::RightParenthesis),
            ),
            RegexPattern::new(
                Regex::new(STRING_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::string_handler(),
            ),
        ]
    }

    pub fn get_inline_regex() -> [RegexPattern<L>; 11] {
        [
            RegexPattern::new(
                Regex::new(NEWLINE_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::non_capture_handler(TokenKind::EndOfLine),
            ),
            RegexPattern::new(
                Regex::new(INLINE_MATH_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::capture_handler(TokenKind::InlineMath),
            ),
            RegexPattern::new(
                Regex::new(BLOCK_MATH_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::capture_handler(TokenKind::BlockMath),
            ),
            RegexPattern::new(
                Regex::new(LINK_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::capture_handler(TokenKind::Link),
            ),
            RegexPattern::new(
                Regex::new(CHARACTER_STYLE_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::capture_handler(TokenKind::CharacterStyle),
            ),
            RegexPattern::new(
                Regex::new(LITERAL_RIGHT_PARENTHESIS_REGEX)
                    .expect("Hard coded regex should be valid."),
                RegexPattern::non_capture_handler(TokenKind::LiteralRightParenthesis),
            ),
            RegexPattern::new(
                Regex::new(BACKSLASH_LEFT_PARENTHESIS_INLINE_REGEX)
                    .expect("Hard coded regex should be valid."),
                RegexPattern::non_capture_handler(TokenKind::BackSlashLeftParenthesisInline),
            ),
            RegexPattern::new(
                Regex::new(BOLD_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::capture_handler(TokenKind::Bold),
            ),
            RegexPattern::new(
                Regex::new(ITALIC_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::non_capture_handler(TokenKind::Italic),
            ),
            RegexPattern::new(
                Regex::new(RIGHT_PARENTHESIS_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::non_capture_handler(TokenKind::RightParenthesis),
            ),
            RegexPattern::new(
                Regex::new(STRING_REGEX).expect("Hard coded regex should be valid."),
                RegexPattern::string_handler(),
            ),
        ]
    }
}
