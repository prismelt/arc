#![cfg(test)]

use crate::lexer::lexer::Lexer;
use crate::lexer::token::TokenKind;

#[test]
fn test_basic_tokenization_1() {
    let source = "Hello World".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::String);
    assert_eq!(tokens[0].value, Some("Hello World".to_string()));
    assert_eq!(tokens[1].kind, TokenKind::EOF);
}

#[test]
fn test_basic_tokenization_2() {
    let source = "Hello World\n".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].kind, TokenKind::String);
    assert_eq!(tokens[0].value, Some("Hello World".to_string()));
    assert_eq!(tokens[1].kind, TokenKind::EndOfLine);
    assert_eq!(tokens[2].kind, TokenKind::EOF);
}

#[test]
fn test_basic_tokenization_3() {
    let source = "Hello World\nHello World\n\n".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].kind, TokenKind::String);
    assert_eq!(tokens[0].value, Some("Hello World".to_string()));
    assert_eq!(tokens[1].kind, TokenKind::EndOfLine);
    assert_eq!(tokens[2].kind, TokenKind::String);
    assert_eq!(tokens[2].value, Some("Hello World".to_string()));
    assert_eq!(tokens[3].kind, TokenKind::EndOfLine);
    assert_eq!(tokens[4].kind, TokenKind::EndOfLine);
    assert_eq!(tokens[5].kind, TokenKind::EOF);
}

#[test]
fn test_basic_tokenization_4() {
    let source = "Hello World\nHello World\n\nHello World".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    assert_eq!(tokens.len(), 7);
    assert_eq!(tokens[0].kind, TokenKind::String);
    assert_eq!(tokens[0].value, Some("Hello World".to_string()));
    assert_eq!(tokens[1].kind, TokenKind::EndOfLine);
    assert_eq!(tokens[2].kind, TokenKind::String);
    assert_eq!(tokens[2].value, Some("Hello World".to_string()));
    assert_eq!(tokens[3].kind, TokenKind::EndOfLine);
    assert_eq!(tokens[4].kind, TokenKind::EndOfLine);
    assert_eq!(tokens[5].kind, TokenKind::String);
    assert_eq!(tokens[5].value, Some("Hello World".to_string()));
    assert_eq!(tokens[6].kind, TokenKind::EOF);
}

#[test]
fn test_comment_handling() {
    let source = "Hello World // This is a comment\nHello World".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].kind, TokenKind::String);
    assert_eq!(tokens[0].value, Some("Hello World ".to_string()));
    assert_eq!(tokens[1].kind, TokenKind::EndOfLine);
    assert_eq!(tokens[2].kind, TokenKind::String);
    assert_eq!(tokens[2].value, Some("Hello World".to_string()));
    assert_eq!(tokens[3].kind, TokenKind::EOF);
}

#[test]
fn test_comment_stability() {
    let source = "some text\n// comment with a ) and \\( \n some text".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].kind, TokenKind::String);
    assert_eq!(tokens[0].value, Some("some text".to_string()));
    assert_eq!(tokens[1].kind, TokenKind::EndOfLine);
    assert_eq!(tokens[2].kind, TokenKind::String);
    assert_eq!(tokens[2].value, Some("some text".to_string()));
    assert_eq!(tokens[3].kind, TokenKind::EOF);
}

#[test]
fn test_comment_newline_consume() {
    let source = "some text\n// a comment".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::String);
    assert_eq!(tokens[0].value, Some("some text".to_string()));
    assert_eq!(tokens[1].kind, TokenKind::EOF);
}

#[test]
fn test_meta_tokenization() {
    let source = "<meta name=My Document key=value />".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::MetaData);
    assert_eq!(
        tokens[0].value,
        Some("name=My Document key=value ".to_string())
    );
    assert_eq!(tokens[1].kind, TokenKind::EOF);
}

#[test]
fn test_complex_structure_tokenization_1() {
    let source = "Hello World\n<meta name=My Document key=value />\nHello World".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].kind, TokenKind::String);
    assert_eq!(tokens[0].value, Some("Hello World".to_string()));
    assert_eq!(tokens[1].kind, TokenKind::EndOfLine);
    assert_eq!(tokens[2].kind, TokenKind::MetaData);
    assert_eq!(
        tokens[2].value,
        Some("name=My Document key=value ".to_string())
    );
    assert_eq!(tokens[3].kind, TokenKind::EndOfLine);
    assert_eq!(tokens[5].kind, TokenKind::EOF);
}

#[test]
fn test_complex_structure_tokenization_2() {
    let source = "&[www.google.com/path/to/page] some char \n %[::red] some text next **bold**\n@[term] 'definition'".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    assert_eq!(tokens.len(), 9);
    assert_eq!(tokens[0].kind, TokenKind::Link);
    assert_eq!(
        tokens[0].value,
        Some("www.google.com/path/to/page".to_string())
    );
    assert_eq!(tokens[1].kind, TokenKind::String);
    assert_eq!(tokens[1].value, Some("some char ".to_string()));
    assert_eq!(tokens[2].kind, TokenKind::EndOfLine);
    assert_eq!(tokens[3].kind, TokenKind::CharacterStyle);
    assert_eq!(tokens[3].value, Some("::red".to_string()));
    assert_eq!(tokens[4].kind, TokenKind::String);
    assert_eq!(tokens[4].value, Some("some text next ".to_string()));
    assert_eq!(tokens[5].kind, TokenKind::Bold);
    assert_eq!(tokens[5].value, Some("bold".to_string()));
    assert_eq!(tokens[6].kind, TokenKind::EndOfLine);
    assert_eq!(tokens[7].kind, TokenKind::Definition);
    assert_eq!(tokens[7].value, Some("term-@[]definition".to_string()));
    assert_eq!(tokens[8].kind, TokenKind::EOF);
}

#[test]
fn test_complex_structure_tokenization_3() {
    let source = r#"text \( text \( text \( text \( text )"#.to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    assert_eq!(tokens.len(), 11);
}

#[test]
fn test_nested_parentheses() {
    let source = r#"text \( text \( text ) text ) text"#.to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    assert_eq!(tokens.len(), 10);
    assert_eq!(tokens[0].kind, TokenKind::String);
    assert_eq!(tokens[0].value, Some("text ".to_string()));
    assert_eq!(tokens[1].kind, TokenKind::BackSlashLeftParenthesisInline);
    assert_eq!(tokens[2].kind, TokenKind::String);
    assert_eq!(tokens[2].value, Some("text ".to_string()));
    assert_eq!(tokens[3].kind, TokenKind::BackSlashLeftParenthesisInline);
    assert_eq!(tokens[4].kind, TokenKind::String);
    assert_eq!(tokens[4].value, Some("text ".to_string()));
    assert_eq!(tokens[5].kind, TokenKind::RightParenthesis);
    assert_eq!(tokens[6].kind, TokenKind::String);
    assert_eq!(tokens[6].value, Some("text ".to_string()));
    assert_eq!(tokens[7].kind, TokenKind::RightParenthesis);
    assert_eq!(tokens[8].kind, TokenKind::String);
    assert_eq!(tokens[8].value, Some("text".to_string()));
    assert_eq!(tokens[9].kind, TokenKind::EOF);
}

#[test]
fn test_bold_tokenization() {
    let source = "This is **bold text** here".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].kind, TokenKind::String);
    assert_eq!(tokens[0].value, Some("This is ".to_string()));
    assert_eq!(tokens[1].kind, TokenKind::Bold);
    assert_eq!(tokens[1].value, Some("bold text".to_string()));
    assert_eq!(tokens[2].kind, TokenKind::String);
    assert_eq!(tokens[2].value, Some("here".to_string()));
    assert_eq!(tokens[3].kind, TokenKind::EOF);
}

#[test]
fn test_italic_tokenization_1() {
    let source = String::from("~Some Text");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].kind, TokenKind::Italic);
    assert_eq!(tokens[1].kind, TokenKind::String);
    assert_eq!(tokens[1].value, Some("Some Text".to_string()));
    assert_eq!(tokens[2].kind, TokenKind::EOF);
}

#[test]
fn test_tokenization_stability_1() {
    let source = String::from("~[red:16:(");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].kind, TokenKind::Italic);
    assert_eq!(tokens[1].kind, TokenKind::String);
    assert_eq!(tokens[1].value, Some("[red:16:(".to_string()));
    assert_eq!(tokens[2].kind, TokenKind::EOF);
}

#[test]
fn test_tokenization_stability_2() {
    let source = String::from("~[red:16:(255, 0, 0)] some text next");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].kind, TokenKind::Italic);
    assert_eq!(tokens[1].kind, TokenKind::String);
    assert_eq!(tokens[1].value, Some("[red:16:(255, 0, 0".to_string()));
    assert_eq!(tokens[2].kind, TokenKind::RightParenthesis);
    assert_eq!(tokens[3].kind, TokenKind::String);
    assert_eq!(tokens[3].value, Some("] some text next".to_string()));
    assert_eq!(tokens[4].kind, TokenKind::EOF);
}

#[test]
fn test_heading_with_escape() {
    let source = String::from("# Hello World");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    assert_ne!(tokens.len(), 2);
}

// #![cfg(test)]

// use crate::lexer::lexer::Lexer;
// use crate::lexer::token::{self, TokenKind};

// #[test]
// fn test_basic_tokenization() {
//     let source = "Hello World".to_string();
//     let lexer = Lexer::new(source);
//     let tokens = lexer.tokenize();

//     assert_eq!(tokens.len(), 2);
//     assert_eq!(tokens[0].kind, TokenKind::String);
//     assert_eq!(tokens[0].value, Some("Hello World".to_string()));
//     assert_eq!(tokens[1].kind, TokenKind::EOF);
// }

// #[test]
// fn test_special_characters() {
//     let source = "[ ] ( ) \\ / # @ & % ~ = < > , : ' - \\(".to_string();
//     let lexer = Lexer::new(source);
//     let tokens = lexer.tokenize();

//     assert_eq!(tokens.len(), 20);
//     assert_eq!(tokens[0].kind, TokenKind::LeftBracket);
//     assert_eq!(tokens[1].kind, TokenKind::RightBracket);
//     assert_eq!(tokens[2].kind, TokenKind::LeftParenthesis);
//     assert_eq!(tokens[3].kind, TokenKind::RightParenthesis);
//     assert_eq!(tokens[4].kind, TokenKind::BackSlash);
//     assert_eq!(tokens[5].kind, TokenKind::ForwardSlash);
//     assert_eq!(tokens[6].kind, TokenKind::Octothorpe);
//     assert_eq!(tokens[7].kind, TokenKind::At);
//     assert_eq!(tokens[8].kind, TokenKind::Ampersand);
//     assert_eq!(tokens[9].kind, TokenKind::Percent);
//     assert_eq!(tokens[10].kind, TokenKind::Tilde);
//     assert_eq!(tokens[11].kind, TokenKind::Equal);
//     assert_eq!(tokens[12].kind, TokenKind::LeftAngleBracket);
//     assert_eq!(tokens[13].kind, TokenKind::RightAngleBracket);
//     assert_eq!(tokens[14].kind, TokenKind::Comma);
//     assert_eq!(tokens[15].kind, TokenKind::Colon);
//     assert_eq!(tokens[16].kind, TokenKind::SingleQuote);
//     assert_eq!(tokens[17].kind, TokenKind::Hyphen);
//     assert_eq!(tokens[18].kind, TokenKind::BackSlashLeftParenthesisInline);
//     assert_eq!(tokens[19].kind, TokenKind::EOF);
// }

// #[test]
// fn test_double_asterisk() {
//     let source = "**bold text**".to_string();
//     let lexer = Lexer::new(source);
//     let tokens = lexer.tokenize();

//     assert_eq!(tokens.len(), 3); // ** + "bold text**"  + EOF
//     assert_eq!(tokens[0].kind, TokenKind::DoubleAsterisk);
//     assert_eq!(tokens[1].kind, TokenKind::String);
//     assert_eq!(tokens[1].value, Some("bold text**".to_string()));
//     assert_eq!(tokens[2].kind, TokenKind::EOF);
// }

// #[test]
// fn test_comment_stability() {
//     let source = "some text // comment with a ) and \\( ".to_string();
//     let lexer = Lexer::new(source);
//     let tokens = lexer.tokenize();
//     assert_eq!(tokens.len(), 2);
// }

// #[test]
// fn test_empty_source() {
//     let source = "".to_string();
//     let lexer = Lexer::new(source);
//     let tokens = lexer.tokenize();

//     assert_eq!(tokens.len(), 1); // Just EOF
//     assert_eq!(tokens[0].kind, TokenKind::EOF);
// }

// #[test]
// fn test_backslash_left_parenthesis() {
//     let source = r"\(inline code)".to_string();
//     let lexer = Lexer::new(source);
//     let tokens = lexer.tokenize();

//     assert_eq!(tokens.len(), 4); // BackSlashLeftParenthesisInline + "inline code" + ) + EOF
//     assert_eq!(tokens[0].kind, TokenKind::BackSlashLeftParenthesisInline);
//     assert_eq!(tokens[1].kind, TokenKind::String);
//     assert_eq!(tokens[1].value, Some("inline code".to_string()));
//     assert_eq!(tokens[2].kind, TokenKind::RightParenthesis);
//     assert_eq!(tokens[3].kind, TokenKind::EOF);
// }

// #[test]
// fn test_string_parse_ability() {
//     let source = "this is a ) string".to_string();
//     let lexer = Lexer::new(source);
//     let tokens = lexer.tokenize();
//     // "this is a " + ) + " string"
//     assert_eq!(tokens.len(), 4);
// }

// #[test]
// fn test_newline_regex() {
//     use std::fs;
//     let source = fs::read_to_string("dev/test/newline.txt").unwrap();
//     let lexer = Lexer::new(source);
//     let tokens = lexer.tokenize();
//     assert_eq!(tokens.len(), 9);
//     assert_eq!(tokens[0].kind, TokenKind::EndOfLine);
//     assert_eq!(tokens[1].kind, TokenKind::EndOfLine);
//     assert_eq!(tokens[2].kind, TokenKind::EndOfLine);
//     assert_eq!(tokens[tokens.len() - 1].kind, TokenKind::EOF);
// }

// #[test]
// fn test_multiple_tokens_in_sequence() {
//     let source = "**bold** and *italic*".to_string();
//     let lexer = Lexer::new(source);
//     let tokens = lexer.tokenize();

//     assert_eq!(tokens.len(), 3); // ** + "bold** and *italic*" + EOF
//     assert_eq!(tokens[0].kind, TokenKind::DoubleAsterisk);
//     assert_eq!(tokens[1].kind, TokenKind::String);
//     assert_eq!(tokens[1].value, Some("bold** and *italic*".to_string()));
// }

// #[test]
// fn test_consecutive_special_characters() {
//     let source = "===>".to_string();
//     let lexer = Lexer::new(source);
//     let tokens = lexer.tokenize();

//     assert_eq!(tokens.len(), 5); // = + = + = + > + EOF
//     assert_eq!(tokens[0].kind, TokenKind::Equal);
//     assert_eq!(tokens[1].kind, TokenKind::Equal);
//     assert_eq!(tokens[2].kind, TokenKind::Equal);
//     assert_eq!(tokens[3].kind, TokenKind::RightAngleBracket);
// }

// #[test]
// fn test_mixed_content() {
//     let source = "Text with (parentheses) and [brackets]".to_string();
//     let lexer = Lexer::new(source);
//     let tokens = lexer.tokenize();

//     assert!(tokens.len() == 4); // Multiple tokens expected
//     assert_eq!(tokens[0].kind, TokenKind::String);
//     assert_eq!(tokens[0].value, Some("Text with (parentheses".to_string()));
// }

// #[test]
// fn test_comment_removal() {
//     let source = "Text before // Comment\nText after".to_string();
//     let lexer = Lexer::new(source);
//     let tokens = lexer.tokenize();

//     // Should only have tokens for "Text before ", "\n", "Text after", and EOF
//     assert!(tokens.len() <= 4);
//     assert_eq!(tokens[0].kind, TokenKind::String);
//     assert_eq!(tokens[0].value, Some("Text before ".to_string()));
// }

// #[test]
// fn test_multiline_content() {
//     let source = "Line 1\nLine 2\nLine 3".to_string();
//     let lexer = Lexer::new(source);
//     let tokens = lexer.tokenize();

//     // Should tokenize each line with EndOfLine tokens between them
//     assert!(tokens.len() > 4); // At least lines + EOLs + EOF
//     let eol_count = tokens
//         .iter()
//         .filter(|t| t.kind == TokenKind::EndOfLine)
//         .count();
//     assert_eq!(eol_count, 2); // 2 newlines
// }

// #[test]
// fn test_escaped_characters() {
//     let source = r"Text with \ backslash and \( inline".to_string();
//     let lexer = Lexer::new(source);
//     let tokens = lexer.tokenize();

//     assert_eq!(tokens.len(), 4); // string + \( + string + EOF
//     assert_eq!(tokens[0].kind, TokenKind::String);
//     assert_eq!(tokens[1].kind, TokenKind::BackSlashLeftParenthesisInline);
//     assert_eq!(tokens[3].kind, TokenKind::EOF);
// }

// #[test]
// fn parse_meta() {
//     let source = "<meta name=My Document key=value />".to_string();
//     let lexer = Lexer::new(source);
//     let tokens = lexer.tokenize();

//     for token in &tokens {
//         token.debug();
//     }
// }
