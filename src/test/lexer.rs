#![cfg(test)]
use crate::lexer::lexer::Lexer;
use crate::lexer::token::{Token, TokenKind};
use crate::lexer::traits::LexerTrait;

#[test]
fn test_basic_tokenization_1() {
    let source = "Hello World".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::String);
    assert_eq!(tokens[0].value, Some("Hello World".to_string()));
    assert_eq!(tokens[1].kind, TokenKind::EOF);
}

#[test]
fn test_basic_tokenization_2() {
    let source = "Hello World\n".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

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
    let tokens = lexer.tokenize().unwrap();

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
    let tokens = lexer.tokenize().unwrap();

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
    let source = "Hello World /// This is a comment\nHello World".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

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
    let source = "some text\n/// comment with a ) and \\( \n some text".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
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
    let source = "some text\n/// a comment".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::String);
    assert_eq!(tokens[0].value, Some("some text".to_string()));
    assert_eq!(tokens[1].kind, TokenKind::EOF);
}

#[test]
fn test_meta_tokenization() {
    let source = "<meta name=My Document key=value />".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 2);
    for token in &tokens {
        println!("{:#?}", token);
    }

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
    let tokens = lexer.tokenize().unwrap();

    for token in &tokens {
        println!("{:#?}", token);
    }

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
    let source = "&[www.google.com/path/to/page]  some char \n %[::red] some text next **bold**\n@[term] 'definition'".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 9);
    assert_eq!(tokens[0].kind, TokenKind::Link);
    assert_eq!(
        tokens[0].value,
        Some("www.google.com/path/to/page".to_string())
    );
    assert_eq!(tokens[1].kind, TokenKind::String);
    assert_eq!(tokens[1].value, Some(" some char ".to_string()));
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
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 11);
}

#[test]
fn test_nested_parentheses() {
    let source = r#"text \( text \( text ) text ) text"#.to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 10);
    assert_eq!(tokens[0].kind, TokenKind::String);
    assert_eq!(tokens[0].value, Some("text ".to_string()));
    assert_eq!(tokens[1].kind, TokenKind::BackSlashLeftParenthesisInline);
    assert_eq!(tokens[2].kind, TokenKind::String);
    assert_eq!(tokens[2].value, Some(" text ".to_string()));
    assert_eq!(tokens[3].kind, TokenKind::BackSlashLeftParenthesisInline);
    assert_eq!(tokens[4].kind, TokenKind::String);
    assert_eq!(tokens[4].value, Some(" text ".to_string()));
    assert_eq!(tokens[5].kind, TokenKind::RightParenthesis);
    assert_eq!(tokens[6].kind, TokenKind::String);
    assert_eq!(tokens[6].value, Some(" text ".to_string()));
    assert_eq!(tokens[7].kind, TokenKind::RightParenthesis);
    assert_eq!(tokens[8].kind, TokenKind::String);
    assert_eq!(tokens[8].value, Some(" text".to_string()));
    assert_eq!(tokens[9].kind, TokenKind::EOF);
}

#[test]
fn test_bold_tokenization() {
    let source = "This is **bold text** here".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].kind, TokenKind::String);
    assert_eq!(tokens[0].value, Some("This is ".to_string()));
    assert_eq!(tokens[1].kind, TokenKind::Bold);
    assert_eq!(tokens[1].value, Some("bold text".to_string()));
    assert_eq!(tokens[2].kind, TokenKind::String);
    assert_eq!(tokens[2].value, Some(" here".to_string()));
    assert_eq!(tokens[3].kind, TokenKind::EOF);
}

#[test]
fn test_italic_tokenization_1() {
    let source = String::from("~Some Text");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

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
    let tokens = lexer.tokenize().unwrap();
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
    let tokens = lexer.tokenize().unwrap();
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
    let tokens = lexer.tokenize().unwrap();
    assert_ne!(tokens.len(), 2);
}

#[test]
fn test_table_tokenization_1() {
    let source = String::from("---\ntable!\n\nHello World\n\n---");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::Table);
    assert_eq!(tokens[0].value, Some("Hello World\n".to_string()));
    assert_eq!(tokens[1].kind, TokenKind::EOF);
}

#[test]
fn test_table_tokenization_2() {
    let source = String::from("---      table!\n\nHello World---");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    assert_ne!(tokens.len(), 2);
    assert_ne!(tokens[0].kind, TokenKind::Table);
}

#[test]
fn test_math_regex_1() {
    let source = String::from("<math x = 1/>");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::InlineMath);
    assert_eq!(tokens[0].value, Some("x = 1".to_string()));
    assert_eq!(tokens[1].kind, TokenKind::EOF);
}

#[test]
fn test_math_regex_2() {
    let source = String::from("<math> x = 1 </math>");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::BlockMath);
    assert_eq!(tokens[0].value, Some("x = 1".to_string()));
    assert_eq!(tokens[1].kind, TokenKind::EOF);
}

#[test]
fn test_inline_regex_inline() {
    let source = String::from("Hello, here's <math x = 1/>, goodbye!");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    assert_ne!(tokens.len(), 2);
}

#[test]
fn test_horizontal_line_1() {
    let source = String::from("---");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::HorizontalLine);
    assert_eq!(tokens[1].kind, TokenKind::EOF);
}

#[test]
fn test_invalid_horizontal_line_1() {
    let source = String::from("some text \\(---)");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    assert!(!tokens.contains(&Token::new(TokenKind::HorizontalLine, None)));
}

#[test]
fn test_code_1() {
    let source = String::from("<code>:python\nprint('Hello World')</code>");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::CodeBlock);
    assert_eq!(
        tokens[0].value,
        Some(":python\nprint('Hello World')".to_string())
    );
    assert_eq!(tokens[1].kind, TokenKind::EOF);
}

#[test]
fn test_code_2() {
    let source = String::from("<code>\nprint('Hello World')</code>");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::CodeBlock);
    assert_eq!(
        tokens[0].value,
        Some(String::from("\nprint('Hello World')"))
    );
    assert_eq!(tokens[1].kind, TokenKind::EOF);
}

#[test]
fn test_code_3() {
    let source = String::from("<code>???\n<, > and /code inside\n</code>");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::CodeBlock);
    assert_eq!(
        tokens[0].value,
        Some(String::from("???\n<, > and /code inside\n"))
    );
    assert_eq!(tokens[1].kind, TokenKind::EOF);
}

#[test]
fn test_code_4() {
    let source = String::from("<code>python, typescript and rust</code>"); // info: no \n
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 2);
    assert_ne!(tokens[0].kind, TokenKind::CodeBlock);
    assert_ne!(
        tokens[0].value,
        Some(String::from("python, typescript and rust"))
    );
    assert_eq!(tokens[1].kind, TokenKind::EOF);
}

#[test]
fn test_empty_code() {
    let source = String::from("<code>\n</code>");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::CodeBlock);
    assert_eq!(tokens[0].value, Some(String::from("\n")));
    assert_eq!(tokens[1].kind, TokenKind::EOF);
}
