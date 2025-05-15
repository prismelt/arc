use crate::types::color::Color;
use maud::{Markup, PreEscaped, html};

#[derive(Debug)]
pub enum ASTNode {
    Inline {
        syntax: Vec<StyledSyntax>,
        content: Vec<ASTNode>,
    },
    BlockedContent {
        content: BlockedContent,
    },
    List {
        syntax: Vec<StyledSyntax>,
        content: Vec<ASTNode>,
    },
    Indicator {
        indicate: Indicator,
    },
}

#[derive(Debug)]
pub enum StyledSyntax {
    Style((Option<Color>, Option<u8>, Option<Color>)),
    Heading(u8),
    Italic,
}

#[derive(Debug)]
pub enum BlockedContent {
    Bold(String),
    Definition(String, String),
    Link(String, Option<String>),
    PlainText(String),
}

#[derive(Debug)]
pub enum Indicator {
    StartOfOrderedList,
    StartOfUnorderedList,
    EndOfOrderedList,
    EndOfUnorderedList,
}

impl ASTNode {
    pub fn build(&self) -> Markup {
        match self {
            ASTNode::BlockedContent { content } => match content {
                BlockedContent::Bold(src) => html! { strong { (src) } },
                BlockedContent::Link(src, content) => {
                    let Some(content) = content else {
                        return html! { a href=(src) { (src) } };
                    };
                    html! { a href=(src) { (content) } }
                }
                BlockedContent::PlainText(src) => html! { span { (src) } },
                BlockedContent::Definition(term, definition) => {
                    html! {
                        span {
                            span style="color: red;text-decoration: underline;" { (term) }
                            ": "
                            span { (definition) }
                        }
                    }
                }
            },
            ASTNode::Inline { syntax, content } => {
                let syntax = syntax
                    .iter()
                    .map(|s| s.build())
                    .collect::<Vec<String>>()
                    .join("");
                let content = PreEscaped(
                    content
                        .iter()
                        .map(|c| c.build().into_string())
                        .collect::<Vec<String>>()
                        .join(""),
                );
                html! { span style=(syntax) { (content) } }
            }
            ASTNode::List { syntax, content } => {
                let syntax = syntax
                    .iter()
                    .map(|s| s.build())
                    .collect::<Vec<String>>()
                    .join("");
                let content = PreEscaped(
                    content
                        .iter()
                        .map(|c| c.build().into_string())
                        .collect::<Vec<String>>()
                        .join(""),
                );
                html! { li style=(syntax) { (content) } }
            }
            ASTNode::Indicator { indicate } => match indicate {
                Indicator::StartOfOrderedList => html! { (PreEscaped("<ol>")) },
                Indicator::StartOfUnorderedList => html! { (PreEscaped("<ul>")) },
                Indicator::EndOfOrderedList => html! { (PreEscaped("</ol>")) },
                Indicator::EndOfUnorderedList => html! { (PreEscaped("</ul>")) },
            },
        }
    }
}

impl StyledSyntax {
    fn build(&self) -> String {
        match self {
            StyledSyntax::Style((color, size, background)) => {
                let mut style = String::new();
                if let Some(color) = color {
                    style.push_str(&format!("color: {};", color.build()));
                }
                if let Some(size) = size {
                    style.push_str(&format!("font-size: {}px;", size));
                }
                if let Some(background) = background {
                    style.push_str(&format!("background-color: {};", background.build()));
                }
                style
            }
            StyledSyntax::Heading(level) => format!("class=\"h{}size\"", level),
            StyledSyntax::Italic => String::from("font-style: italic;"),
        }
    }

    pub fn new_style(src: String) -> Result<Self, String> {
        if src.replace(":", "").is_empty() {
            return Err(String::from("Invalid style syntax: Empty"));
        }

        let src: Vec<&str> = src.split(":").collect();
        let result = Self::parse_src(src);
        if let Err(err) = result {
            return Err(err);
        }
        let tuple = result.unwrap();
        Ok(Self::Style(tuple))
    }

    fn parse_src(src: Vec<&str>) -> Result<(Option<Color>, Option<u8>, Option<Color>), String> {
        match src.len() {
            1 => {
                let color = Self::parse_color(src[0])?;
                Ok((color, None, None))
            }
            2 => {
                let color = Self::parse_color(src[0])?;
                let size = Self::parse_u8(src[1])?;
                Ok((color, size, None))
            }
            3 => {
                let color = Self::parse_color(src[0])?;
                let size = Self::parse_u8(src[1])?;
                let background = Self::parse_color(src[2])?;
                Ok((color, size, background))
            }
            _ => Err(format!("Invalid style syntax: {}", src.join(":"))),
        }
    }

    fn parse_u8(value: &str) -> Result<Option<u8>, String> {
        if value.trim().is_empty() {
            return Ok(None);
        }
        let result = value.parse::<u8>();
        if let Ok(value) = result {
            return Ok(Some(value));
        }
        Err(format!(
            "Invalid value for font size: '{}', msg:`{}`",
            value,
            result.unwrap_err()
        ))
    }

    fn parse_color(value: &str) -> Result<Option<Color>, String> {
        if value.trim().is_empty() {
            return Ok(None);
        }
        let result = Color::from_string(value.to_string());
        if let Ok(color) = result {
            return Ok(Some(color));
        }
        Err(result.unwrap_err())
    }
}
