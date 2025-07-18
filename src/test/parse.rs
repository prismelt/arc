#![cfg(test)]
use crate::lexer::lexer::Lexer;
use crate::lexer::traits::LexerTrait;
use crate::parse::meta::MetaProperties;
use crate::parse::node::ASTNode;
use crate::parse::node::Indicator;
use crate::parse::parse::Parser;

#[test]
fn test_parser_debug() {
    let parser = Parser::new(Vec::new());
    assert_eq!(
        format!("{:?}", parser),
        "Parser { source: [], document: Document { meta: [], nodes: [] } }"
    );
}

#[test]
fn test_basic_parsing_1() {
    let source = "Hello World".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    assert_eq!(document.meta.len(), 0);
    assert_eq!(document.nodes.len(), 1);
    assert_eq!(document.nodes[0].len(), 1);
    assert_eq!(
        format!("{:?}", document.nodes[0][0]),
        "Inline { syntax: [], content: [BlockedContent { content: PlainText(\"Hello World\") }] }"
    );
}

#[test]
fn test_basic_parsing_2() {
    let source = "Hello World\nHello World".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    assert_eq!(document.meta.len(), 0);
    assert_eq!(document.nodes.len(), 2);
    assert_eq!(document.nodes[0].len(), 1);
    assert_eq!(document.nodes[1].len(), 1);
    assert_eq!(
        format!("{:?}", document.nodes[0][0]),
        "Inline { syntax: [], content: [BlockedContent { content: PlainText(\"Hello World\") }] }"
    );
    assert_eq!(
        format!("{:?}", document.nodes[1][0]),
        "Inline { syntax: [], content: [BlockedContent { content: PlainText(\"Hello World\") }] }"
    );
}

#[test]
fn test_basic_parsing_3() {
    let source = "Hello World\nHello World\n\nHello World".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    assert_eq!(document.meta.len(), 0);
    assert_eq!(document.nodes.len(), 4);
    assert_eq!(document.nodes[0].len(), 1);
    assert_eq!(document.nodes[1].len(), 1);
    assert_eq!(document.nodes[2].len(), 0);
    assert_eq!(document.nodes[3].len(), 1);
    assert_eq!(
        format!("{:?}", document.nodes[0][0]),
        "Inline { syntax: [], content: [BlockedContent { content: PlainText(\"Hello World\") }] }"
    );
    assert_eq!(
        format!("{:?}", document.nodes[1][0]),
        "Inline { syntax: [], content: [BlockedContent { content: PlainText(\"Hello World\") }] }"
    );
    assert_eq!(format!("{:?}", document.nodes[2]), "[]");
    assert_eq!(
        format!("{:?}", document.nodes[3][0]),
        "Inline { syntax: [], content: [BlockedContent { content: PlainText(\"Hello World\") }] }"
    );
}

#[test]
fn test_meta_parsing_1() {
    let source = "<meta name=My Document key=value />\nHello World".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    assert_eq!(document.meta.len(), 1);
    assert_eq!(
        document.meta[0],
        MetaProperties::Name("My Document key=value".to_string())
    );
    assert_eq!(document.nodes.len(), 2);
    assert_eq!(document.nodes[1].len(), 1);
    assert_eq!(
        format!("{:?}", document.nodes[1][0]),
        "Inline { syntax: [], content: [BlockedContent { content: PlainText(\"Hello World\") }] }"
    );
}

#[test]
fn test_meta_parsing_2() {
    let source =
        "<meta name=My Document>\n<meta title=TEST />\nHello World\nThis is test".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    assert_eq!(document.meta.len(), 2);
    assert_eq!(
        document.meta[0],
        MetaProperties::Name("My Document".to_string())
    );
    assert_eq!(document.meta[1], MetaProperties::Title("TEST".to_string()));
    assert_eq!(document.nodes.len(), 4);
    assert_eq!(document.nodes[2].len(), 1);
    assert_eq!(
        format!("{:?}", document.nodes[2][0]),
        "Inline { syntax: [], content: [BlockedContent { content: PlainText(\"Hello World\") }] }"
    );
    assert_eq!(document.nodes[3].len(), 1);
    assert_eq!(
        format!("{:?}", document.nodes[3][0]),
        "Inline { syntax: [], content: [BlockedContent { content: PlainText(\"This is test\") }] }"
    );
}

#[test]
fn test_complex_expression_1() {
    let source = String::from("%[red:16:(255, 0, 0)] some text next");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    assert_eq!(document.meta.len(), 0);
    assert_eq!(document.nodes.len(), 1);
    assert_eq!(document.nodes[0].len(), 1);
    assert_eq!(
        format!("{:?}", document.nodes[0][0]),
        "Inline { syntax: [Style((Some(Literal(Red)), Some(16), Some(RGB(255, 0, 0))))], content: [BlockedContent { content: PlainText(\"some text next\") }] }"
    );
}

#[test]
fn test_complex_expression_2() {
    let source = String::from("~ %[red:16:(255, 0, 0)] some text next");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    assert_eq!(document.meta.len(), 0);
    assert_eq!(document.nodes.len(), 1);
    assert_eq!(document.nodes[0].len(), 1);
    assert_eq!(
        format!("{:?}", document.nodes[0][0]),
        "Inline { syntax: [Italic, Style((Some(Literal(Red)), Some(16), Some(RGB(255, 0, 0))))], content: [BlockedContent { content: PlainText(\"some text next\") }] }"
    );
}

#[test]
fn test_complex_expression_3() {
    let source = String::from("~ %[red] some char \\(&[www.google.com/path/to/page] some char)");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    assert_eq!(document.meta.len(), 0);
    assert_eq!(document.nodes.len(), 1);
    assert_eq!(document.nodes[0].len(), 1);
    assert_eq!(
        format!("{:?}", document.nodes[0][0]),
        "Inline { syntax: [Italic, Style((Some(Literal(Red)), None, None))], content: [BlockedContent { content: PlainText(\"some char \") }, Inline { syntax: [], content: [BlockedContent { content: Link(\"www.google.com/path/to/page\", Some(\"some char\")) }] }] }"
    );
}

#[test]
fn test_complex_expression_4() {
    let source = "~ %[red:10:blue] type string\nthis is next line with \\(inner **bold**) content"
        .to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    assert_eq!(document.nodes.len(), 2);
    assert_eq!(document.meta.len(), 0);
    assert_eq!(document.nodes[0].len(), 1);
    assert_eq!(document.nodes[1].len(), 1);
}

#[test]
fn test_definition_1() {
    let source = "@[term] 'definition'".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    assert_eq!(document.nodes.len(), 1);
    assert_eq!(document.meta.len(), 0);
    assert_eq!(document.nodes[0].len(), 1);
    assert_eq!(
        format!("{:?}", document.nodes[0][0]),
        "Inline { syntax: [], content: [BlockedContent { content: Definition(\"term\", \"definition\") }] }"
    );
}

#[test]
fn test_definition_2() {
    let source = "@[term, term2, term3] 'definition, definition2, definition3'".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    assert_eq!(document.nodes.len(), 1);
    assert_eq!(document.meta.len(), 0);
    assert_eq!(document.nodes[0].len(), 1);
    assert_eq!(
        format!("{:?}", document.nodes[0][0]),
        "Inline { syntax: [], content: [BlockedContent { content: Definition(\"term, term2, term3\", \"definition, definition2, definition3\") }] }"
    );
}

#[test]
fn test_heading_1() {
    let source = "# Hello World".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);

    let document = parser.parse().unwrap();

    assert_eq!(document.nodes.len(), 1);
    assert_eq!(document.meta.len(), 0);
    assert_eq!(document.nodes[0].len(), 1);
    assert_eq!(
        format!("{:?}", document.nodes[0][0]),
        "Inline { syntax: [Heading(1)], content: [BlockedContent { content: PlainText(\"Hello World\") }] }"
    );
}

#[test]
fn test_multiple_headings() {
    let source = "# Hello World\n## This is next line".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    assert_eq!(document.nodes.len(), 2);
    assert_eq!(document.meta.len(), 0);
    assert_eq!(document.nodes[0].len(), 1);
    assert_eq!(document.nodes[1].len(), 1);
    assert_eq!(
        format!("{:?}", document.nodes[0][0]),
        "Inline { syntax: [Heading(1)], content: [BlockedContent { content: PlainText(\"Hello World\") }] }"
    );
    assert_eq!(
        format!("{:?}", document.nodes[1][0]),
        "Inline { syntax: [Heading(2)], content: [BlockedContent { content: PlainText(\"This is next line\") }] }"
    );
}

#[test]
fn test_deep_nesting() {
    let source = String::from("Hello, \\(inner 1 \\( inner 2 \\(inner 3) inner 2) inner 1) world.");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    assert_eq!(document.nodes.len(), 1);
    assert_eq!(document.nodes[0].len(), 1);

    // Check the structure matches what we see in the output
    if let ASTNode::Inline { syntax: _, content } = &document.nodes[0][0] {
        assert_eq!(content.len(), 3); // "Hello, ", nested content, and "world."

        // Check the nested content (second element)
        if let ASTNode::Inline {
            syntax: _,
            content: inner_content,
        } = &content[1]
        {
            assert_eq!(inner_content.len(), 3); // "inner 1 ", nested content, and "inner 1"

            // Check the deeper nested content
            if let ASTNode::Inline {
                syntax: _,
                content: deeper_content,
            } = &inner_content[1]
            {
                assert_eq!(deeper_content.len(), 3); // "inner 2 ", nested content, and "inner 2"
                return;
            }
        }
    }
    panic!("Failed to parse deep nesting");
}

#[test]
fn test_ordered_list_1() {
    let source = String::from("1. Hello World\n2. This is next line\n3. This is next line");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    assert_eq!(document.nodes.len(), 5); // 3 list, 2 list indicators
}

#[test]
fn test_ordered_list_2() {
    let source = String::from(
        "1. Hello World\n2. This is next line\n3. This is next line\n\n4. This is next line",
    );
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    assert_eq!(document.nodes.len(), 6);
}

#[test]
fn test_ordered_list_with_styling() {
    let source =
        String::from("1. \\(Hello World)\n2. This is \\(next line)\n3. %[red]This is next line");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    assert_eq!(document.nodes.len(), 5); // 3 list, 2 list indicators
}

#[test]
fn test_unordered_list_1() {
    let source = String::from("- Hello World\n- This is next line\n- This is next line");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    assert_eq!(document.nodes.len(), 5); // 3 list, 2 list indicators
}

#[test]
fn test_nesting_right_parenthesis() {
    let source = String::from(r"\(%[yellow](second time parsing\))");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    assert_eq!(
        format!("{:?}", document.nodes[0][0]),
        r#"Inline { syntax: [], content: [Inline { syntax: [Style((Some(Literal(Yellow)), None, None))], content: [BlockedContent { content: PlainText("(second time parsing") }, BlockedContent { content: PlainText(")") }] }] }"#
    );
}

#[test]
fn test_triple_tide() {
    let source = String::from("~~~Hello World~~~");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    assert_eq!(document.nodes.len(), 1);
    assert_eq!(document.nodes[0].len(), 1);
    assert_eq!(
        format!("{:?}", document.nodes[0][0]),
        "Inline { syntax: [], content: [BlockedContent { content: PlainText(\"~~~Hello World~~~\") }] }"
    );
}

#[test]
fn test_parsing_math_1() {
    let source = String::from("<math> x = 1 </math>");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    let html = document.build();
    assert!(
        html.replace("&nbsp;", " ")
            .contains("<span>$$x = 1$$</span>")
    );
}

#[test]
fn test_parsing_math_2() {
    let source = String::from("<math x = 1/>");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    let html = document.build();
    assert!(
        html.replace("&nbsp;", " ")
            .contains("<span>\\(x = 1\\)</span>")
    );
}

#[test]
fn test_parsing_math_3() {
    let source = String::from("Hello World <math x = 1/>");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    let html = document.build();
    assert!(
        html.replace("&nbsp;", " ")
            .contains("<span>\\(x = 1\\)</span>")
    );
}

#[test]
fn test_invalid_math() {
    let source = String::from("math <math> x = 1 </math> cannot exist inLine");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 2);
}

#[test]
fn test_inline_math_inline() {
    let source = String::from("Hello World <math x = 1/> This is next line");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    let html = document.build();
    assert!(
        html.replace("&nbsp;", " ")
            .contains("<span>\\(x = 1\\)</span>")
    );
}

#[test]
fn test_ordered_with_unordered_list() {
    let source = String::from(
        "1. Hello World\n2. This is next line\n3. This is next line\n- This is next line\n- This is next line\n- This is next line",
    );
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    assert_eq!(document.nodes.len(), 10);
}

#[test]
fn test_horizontal_line_1() {
    let source = String::from("---");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    assert_eq!(document.nodes.len(), 1);
    assert_eq!(document.nodes[0].len(), 1);
    assert_eq!(
        format!("{:?}", document.nodes[0][0]),
        "Indicator { indicate: HorizontalLine }"
    );
}

#[test]
fn test_horizontal_line_2() {
    let source = String::from("Hello World\n---\nThis is next line");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    assert_eq!(document.nodes.len(), 3);
    assert_eq!(document.nodes[1].len(), 1);
    assert_eq!(
        format!("{:?}", document.nodes[1][0]),
        "Indicator { indicate: HorizontalLine }"
    );
}

#[test]
fn test_invalid_horizontal_line_1() {
    let source = String::from("some text \\(---)");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    let contains_horizontal = document.nodes.iter().any(|node| match node.as_slice() {
        [
            ASTNode::Indicator {
                indicate: Indicator::HorizontalLine,
            },
        ] => true,
        _ => false,
    });
    assert!(!contains_horizontal);
}

#[test]
fn test_code_1() {
    let source = String::from("<code>:python\nprint('Hello World')</code>");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    assert_eq!(document.nodes.len(), 1);
    assert_eq!(document.nodes[0].len(), 1);
    assert_eq!(
        format!("{:?}", document.nodes[0][0]),
        "BlockedContent { content: CodeBlock(\"python\", \"print('Hello World')\") }"
    );
}
#[test]
fn test_code_2() {
    let source = String::from("<code>\nprint('Hello World')</code>");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    assert_eq!(document.nodes.len(), 1);
    assert_eq!(document.nodes[0].len(), 1);
    assert_eq!(
        format!("{:?}", document.nodes[0][0]),
        "BlockedContent { content: CodeBlock(\"\", \"print('Hello World')\") }"
    );
}
#[test]
fn test_code_3() {
    let source = String::from("<code>???\n<, > and /code inside\n</code>"); // no colon used
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    assert_eq!(document.nodes.len(), 1);
    assert_eq!(document.nodes[0].len(), 1);
    assert_eq!(
        format!("{:?}", document.nodes[0][0]),
        r#"BlockedContent { content: CodeBlock("", "<, > and /code inside") }"#
    );
}
#[test]
fn test_code_4() {
    let source = String::from("<code>python, typescript and rust</code>"); // info: no \n
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let document = parser.parse().unwrap();

    assert_ne!(
        format!("{:?}", document.nodes[0][0]),
        "BlockedContent { content: CodeBlock(\"python, typescript and rust\", \"\") }"
    );
}
