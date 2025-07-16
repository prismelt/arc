#![cfg(test)]

use crate::lexer::lexer::Lexer;
use crate::lexer::traits::LexerTrait;
use crate::parse::parse::Parser;

fn dedent(text: &str) -> String {
    text.replace("    ", "").replace("\t", "")
}

#[test]
fn test_html_parsing_1() {
    let source = dedent(
        r"---html!
        <div>
            <p>Hello World</p>
        </div>
        ---",
    );
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let result = parser.parse().unwrap();
    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0].len(), 1);
    assert_eq!(
        format!("{:?}", result.nodes[0][0]),
        "BlockedContent { content: HTMLContainer(\"<div>\\n<p>Hello World</p>\\n</div>\") }"
    );
}

#[test]
fn test_html_parsing_2() {
    let source = dedent(
        r"---html!
        <div>
            <p>Hello World</p>
            <p>This is next line</p>
            <p>This is next line</p>
        </div>
        ---

        This is next line",
    );
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let result = parser.parse().unwrap();
    assert_eq!(result.nodes.len(), 2);
    assert_eq!(result.nodes[0].len(), 1);
    assert_eq!(result.nodes[1].len(), 1);
    assert_eq!(
        format!("{:?}", result.nodes[0][0]),
        "BlockedContent { content: HTMLContainer(\"<div>\\n<p>Hello World</p>\\n<p>This is next line</p>\\n<p>This is next line</p>\\n</div>\") }"
    );
    assert_eq!(
        format!("{:?}", result.nodes[1][0]),
        "Inline { syntax: [], content: [BlockedContent { content: PlainText(\"This is next line\") }] }"
    );
}

#[test]
fn test_html_final_render() {
    let source = dedent(
        r"---html!
        <div>
            <p>Hello World</p>
            <p>This is next line</p>
            <p>This is next line</p>
        </div>
        ---",
    );
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let result = parser.parse().unwrap();
    let html = format!("{:?}", result.build());
    assert!(html.contains("<div>"));
    assert!(html.contains("<p>"));
    assert!(html.contains("</p>"));
    assert!(html.contains("</div>"));
}
