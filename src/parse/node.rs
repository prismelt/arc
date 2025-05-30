use crate::utilities::color::Color;
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
    Table {
        position: (Option<f32>, Option<f32>),
        content: Vec<Vec<TableContent>>,
    },
}

#[derive(Debug)]
pub struct TableContent {
    content: Vec<ASTNode>,
    is_heading: bool,
    style: String,
    rowspan: u16,
    colspan: u16,
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
    BlockMath(String),
    InlineMath(String),
}

#[derive(Debug)]
pub enum Indicator {
    StartOfOrderedList,
    StartOfUnorderedList,
    EndOfOrderedList,
    EndOfUnorderedList,
    HorizontalLine,
}

struct CSSAttrs {
    class: Option<String>,
    style: String,
}

impl Default for CSSAttrs {
    fn default() -> Self {
        Self {
            class: None,
            style: String::new(),
        }
    }
}

impl TableContent {
    pub fn new(content: Vec<ASTNode>, is_heading: bool, style: String) -> Self {
        Self {
            content,
            is_heading,
            style,
            rowspan: 1,
            colspan: 1,
        }
    }

    pub fn add_merge_row(&mut self) {
        self.rowspan += 1;
    }
    pub fn add_merge_col(&mut self) {
        self.colspan += 1;
    }

    fn build(&self) -> Markup {
        if self.is_heading {
            html! { th colspan=(self.colspan) rowspan=(self.rowspan) style=(self.style) { (PreEscaped(self.content.iter().map(|c| c.build().into_string()).collect::<Vec<String>>().join(""))) } }
        } else {
            html! { td colspan=(self.colspan) rowspan=(self.rowspan) style=(self.style) { (PreEscaped(self.content.iter().map(|c| c.build().into_string()).collect::<Vec<String>>().join(""))) } }
        }
    }
}

impl ASTNode {
    pub fn build(&self) -> Markup {
        match self {
            ASTNode::BlockedContent { content } => Self::build_block_content(content),
            ASTNode::Inline { syntax, content } => Self::build_inline(syntax, content),
            ASTNode::List { syntax, content } => Self::build_list(syntax, content),
            ASTNode::Indicator { indicate } => Self::match_indicator(indicate),
            ASTNode::Table { position, content } => Self::build_table(position, content),
        }
    }

    fn resolve_syntax(syntax: &Vec<StyledSyntax>) -> (String, String) {
        let syntax = syntax
            .iter()
            .map(|s| s.build())
            .fold(CSSAttrs::default(), |mut a, n| {
                a.class = a.class.or(n.class);
                a.style = format!("{}{}", a.style, n.style);
                a
            });

        (syntax.class.unwrap_or(String::new()), syntax.style)
    }

    fn iter_build_content(content: &Vec<ASTNode>) -> Markup {
        PreEscaped(
            content
                .iter()
                .map(|c| c.build().into_string())
                .collect::<Vec<String>>()
                .join(""),
        )
    }

    fn match_indicator(indicator: &Indicator) -> Markup {
        match indicator {
            Indicator::StartOfOrderedList => html! { (PreEscaped("<ol>")) },
            Indicator::StartOfUnorderedList => html! { (PreEscaped("<ul>")) },
            Indicator::EndOfOrderedList => html! { (PreEscaped("</ol>")) },
            Indicator::EndOfUnorderedList => html! { (PreEscaped("</ul>")) },
            Indicator::HorizontalLine => html! { (PreEscaped("<hr />")) },
        }
    }

    fn handle_table_position(position: &(Option<f32>, Option<f32>)) -> String {
        if let (Some(width), Some(height)) = position {
            let width = width * 10.0;
            let height = height * 10.0;
            format!("width: {}px; height: {}px;", width, height)
        } else {
            String::from("width: auto; height: auto;")
        }
    }

    fn build_table(
        position: &(Option<f32>, Option<f32>),
        content: &Vec<Vec<TableContent>>,
    ) -> Markup {
        html! {
            table style=(Self::handle_table_position(position)) {
                tbody {
                    @for row in content {
                        tr {
                            @for cell in row {
                                (cell.build())
                            }
                        }
                    }
                }
            }
        }
    }

    fn build_list(syntax: &Vec<StyledSyntax>, content: &Vec<ASTNode>) -> Markup {
        let (class, style) = Self::resolve_syntax(syntax);
        let content = Self::iter_build_content(content);
        html! { li class=(class) style=(style) { (content) } }
    }

    fn build_inline(syntax: &Vec<StyledSyntax>, content: &Vec<ASTNode>) -> Markup {
        let (class, style) = Self::resolve_syntax(syntax);
        let content = Self::iter_build_content(content);
        html! { span class=(class) style=(style) { (content) } }
    }

    fn build_link(src: &str, content: &Option<String>) -> Markup {
        match content {
            Some(content) => html! { a href=(src) { (content) } },
            None => html! { a href=(src) { (src) } },
        }
    }

    fn build_definition(term: &str, definition: &str) -> Markup {
        html! {
            span {
                span style="color: red;text-decoration: underline;" { (term) }
                ": "
                span { (definition) }
            }
        }
    }

    fn build_block_content(content: &BlockedContent) -> Markup {
        match content {
            BlockedContent::Bold(src) => html! { strong { (src) } },
            BlockedContent::Link(src, content) => Self::build_link(src, content),
            BlockedContent::PlainText(src) => html! { span { (src) } },
            BlockedContent::Definition(term, definition) => {
                Self::build_definition(term, definition)
            }
            BlockedContent::InlineMath(src) => {
                html! { span { (PreEscaped(format!(r"\({}\)", src))) } }
            }
            BlockedContent::BlockMath(src) => {
                html! { span { (PreEscaped(format!(r"$${}$$", src))) } }
            }
        }
    }
}

impl StyledSyntax {
    fn build(&self) -> CSSAttrs {
        match self {
            StyledSyntax::Style((color, size, background)) => {
                let mut style = String::new();
                if let Some(color) = color {
                    style.push_str(&format!("color: {} !important;", color.build()));
                }
                if let Some(size) = size {
                    style.push_str(&format!("font-size: {}px !important;", size));
                }
                if let Some(background) = background {
                    style.push_str(&format!(
                        "background-color: {} !important;",
                        background.build()
                    ));
                }
                CSSAttrs {
                    class: None,
                    style: style,
                }
            }
            StyledSyntax::Heading(level) => CSSAttrs {
                class: Some(format!("h{}size", level)),
                style: String::new(),
            },
            StyledSyntax::Italic => CSSAttrs {
                class: None,
                style: String::from("font-style: italic !important;"),
            },
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
