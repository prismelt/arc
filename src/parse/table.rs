use super::node::{ASTNode, BlockedContent, TableContent};
use super::tree::Document;
use crate::lexer::lexer_lite::LexerLite;
use crate::lexer::traits::LexerTrait;
use crate::utilities::constants::{MULTIPLE_NEWLINE_REGEX, WIDTH_HEIGHT_REGEX};
use fancy_regex::Regex;

pub fn parse_table(src: String, document: &mut Document) -> Result<(), String> {
    let regex = Regex::new(MULTIPLE_NEWLINE_REGEX).expect("Hard coded regex should be valid.");
    let src = regex.replace_all(&src, "\n").to_string();
    let mut src: Vec<&str> = src.split("\n").collect();
    if src.len() == 0 {
        crate::warn!("Runtime Warning: Invalid table syntax: Empty table");
        return Ok(());
    }

    let mut table_content: Vec<Vec<TableContent>> = Vec::new();

    let first_line = src[0].trim();
    let position = parse_position(first_line);

    if position != (None, None) {
        src.remove(0);
    }

    let mut row_pos = 0;
    'outer: for line in src {
        let mut line = line.trim();
        if line.is_empty() {
            continue 'outer;
        }
        let is_heading = is_heading(line);
        if is_heading {
            line = &line[1..line.len() - 1];
        }
        table_content.push(Vec::new());

        let mut col_pos = 0;
        'inner: for cell in line.split(";") {
            let (content, style) = format_style(cell);

            if content.trim() == "_" {
                table_content[row_pos]
                    .get_mut(col_pos - 1)
                    .expect("Row merge with no left neighbor")
                    .add_merge_col();
                // row_pos += 1;
                continue 'inner;
            }
            if content.trim() == "^" {
                table_content
                    .get_mut(row_pos - 1)
                    .expect("Column merge with no upper row")
                    .get_mut(col_pos)
                    .expect("Column merge with no upper neighbor")
                    .add_merge_row();
                col_pos += 1;
                continue 'inner;
            }

            let lexer = LexerLite::new(content);
            let tokens = lexer.tokenize()?;
            let parser = super::parse::Parser::new(tokens);
            let mut nodes = parser.parse()?.nodes;
            if nodes.len() == 0 {
                // info: empty cell, composed with ;; ...
                table_content[row_pos].push(TableContent::new(
                    vec![ASTNode::BlockedContent {
                        content: BlockedContent::PlainText(String::new()),
                    }],
                    is_heading,
                    style,
                ));
                continue 'inner;
            }
            let content = nodes
                .pop()
                .expect("Pop not empty table cell should be valid.");

            table_content[row_pos].push(TableContent::new(content, is_heading, style));
            col_pos += 1;
        }
        row_pos += 1;
    }

    document.append_node(vec![ASTNode::Table {
        position,
        content: table_content,
    }]);
    Ok(())
}

fn parse_position(line: &str) -> (Option<f32>, Option<f32>) {
    let regex = Regex::new(WIDTH_HEIGHT_REGEX).expect("Hard coded regex should be valid.");
    let matched = regex.find(line);
    if let Ok(Some(_)) = matched {
        let captures = regex.captures(line).unwrap().unwrap();
        let width = captures.get(1).unwrap().as_str().parse::<f32>().unwrap();
        let height = captures.get(2).unwrap().as_str().parse::<f32>().unwrap();
        return (Some(width), Some(height));
    }
    (None, None)
}

fn is_heading(line: &str) -> bool {
    line.starts_with("[") && line.ends_with("]")
}

fn format_style(line: &str) -> (String, String) {
    let line = line.trim();
    if line.starts_with("=") && line.ends_with("=") {
        return (
            String::from(&line[1..line.len() - 1]),
            String::from("text-align: center;"),
        );
    } else if line.starts_with("=") {
        return (
            String::from(&line[1..line.len()]),
            String::from("text-align: left;"),
        );
    } else if line.ends_with("=") {
        return (
            String::from(&line[..line.len() - 1]),
            String::from("text-align: right;"),
        );
    }
    (String::from(line), String::new())
}
