#![cfg(test)]

use crate::lexer::lexer::Lexer;
use crate::lexer::traits::LexerTrait;
use crate::parse::parse::Parser;

fn dedent(text: &str) -> String {
    text.replace("    ", "").replace("\t", "")
}

#[test]
fn test_parse_table_1() {
    let src = dedent(
        r"--- table!
        [%[red]Heading 1;Heading 2;Heading 3]
        Cell 1;Cell 2;Cell 3
        Cell 4;Cell 5;Cell 6
        ---",
    );
    let lexer = Lexer::new(src);
    let tokens = lexer.tokenize();
    let parser = Parser::new(tokens);
    let document = parser.parse();

    let output = document.build();
    assert!(output.contains(r#"style="color: rgb(255, 0, 0);""#));
}

#[test]
fn test_table_with__() {
    let src = dedent(
        "--- table!
        [Heading 1;Heading 2;Heading 3]
        Cell 1;_;Cell 3
        Cell 4;Cell 5;Cell 6
        ---",
    );
    let lexer = Lexer::new(src);
    let tokens = lexer.tokenize();
    let parser = Parser::new(tokens);
    let document = parser.parse();

    let output = document.build();
    assert!(output.contains(r#"colspan="2""#));
}

#[test]
fn test_table_with_upper() {
    let src = dedent(
        "--- table!
        [Heading 1;Heading 2;Heading 3]
        Cell 1;Cell 2;Cell 3
        ^;^;Cell 6
        ---",
    );
    let lexer = Lexer::new(src);
    let tokens = lexer.tokenize();
    let parser = Parser::new(tokens);
    let document = parser.parse();

    let output = document.build();
    assert!(output.contains(r#"rowspan="2""#));
}

#[test]
#[should_panic]
fn test_invalid__() {
    let src = dedent(
        "--- table!
        [Heading 1;Heading 2;Heading 3]
        _;Cell 2;Cell 3
        Cell 4;Cell 5;Cell 6
        ---",
    );
    let lexer = Lexer::new(src);
    let tokens = lexer.tokenize();
    let parser = Parser::new(tokens);
    let document = parser.parse();

    let _ = document.build();
}

#[test]
#[should_panic]
fn test_invalid_upper() {
    let src = dedent(
        "--- table!
        [^;Heading 2;Heading 3]
        Cell 1;Cell 2;Cell 3
        Cell 4;;Cell 6
        ---",
    );
    let lexer = Lexer::new(src);
    let tokens = lexer.tokenize();
    let parser = Parser::new(tokens);
    let document = parser.parse();

    let _ = document.build();
}

#[test]
fn test_meta_styling() {
    let src = dedent(
        "--- table!
        (12.5, 22.4)
        [Heading 1;Heading 2;Heading 3]
        Cell 1;Cell 2;Cell 3
        Cell 4;Cell 5;Cell 6
        ---",
    );
    let lexer = Lexer::new(src);
    let tokens = lexer.tokenize();
    let parser = Parser::new(tokens);
    let document = parser.parse();
    let output = document.build();
    assert!(output.contains(r#"width: 125px; height: 224px;"#));
}

#[test]
fn test_consecutive_merge() {
    let src = dedent(
        "--- table!
        [Heading 1;Heading 2;Heading 3]
        Cell 1; _ ; _
        ---",
    );
    let lexer = Lexer::new(src);
    let tokens = lexer.tokenize();
    let parser = Parser::new(tokens);
    let document = parser.parse();
    let output = document.build();
    assert!(output.contains(r#"colspan="3""#));
}
