use super::node::{ASTNode, TableContent};
use super::tree::Document;
use crate::lexer::lexer::Lexer;
use crate::types::constants::{MULTIPLE_NEWLINE_REGEX, WIDTH_HEIGHT_REGEX};
use fancy_regex::Regex;

pub fn parse_table(src: String, document: &mut Document) {
    let regex = Regex::new(MULTIPLE_NEWLINE_REGEX).unwrap();
    let src = regex.replace_all(&src, "\n").to_string();
    let mut src: Vec<&str> = src.split("\n").collect();
    if src.len() == 0 {
        eprintln!("Invalid table syntax: Empty table");
        return;
    }

    let mut table_content: Vec<Vec<TableContent>> = Vec::new();

    let first_line = src[0].trim();
    let position = parse_position(first_line);

    if position != (None, None) {
        src.remove(0);
    }

    'outer: for (i, line) in src.iter().enumerate() {
        let mut line = line.trim();
        if line.is_empty() {
            continue 'outer;
        }
        let is_heading = is_heading(line);
        if is_heading {
            line = &line[1..line.len() - 1];
        }
        table_content.push(Vec::new());
        'inner: for (j, cell) in line.split(";").enumerate() {
            let (content, style) = format_style(cell);

            if content.trim() == "_" {
                table_content[i]
                    .get_mut(j - 1)
                    .expect("Row merge with no left neighbor")
                    .add_merge_col();
                continue 'inner;
            }
            if content.trim() == "^" {
                table_content
                    .get_mut(i - 1)
                    .expect("Column merge with no upper row")
                    .get_mut(j)
                    .expect("Column merge with no upper neighbor")
                    .add_merge_row();
                continue 'inner;
            }

            let lexer = Lexer::new(content);
            let tokens = lexer.tokenize();
            let parser = super::parse::Parser::new(tokens);
            let content = parser.parse().nodes.pop().expect("Empty table cell");

            table_content[i].push(TableContent::new(content, is_heading, style));
        }
    }

    document.append_node(vec![ASTNode::Table {
        position,
        content: table_content,
    }]);
}

fn parse_position(line: &str) -> (Option<f32>, Option<f32>) {
    let regex = Regex::new(WIDTH_HEIGHT_REGEX).unwrap();
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
