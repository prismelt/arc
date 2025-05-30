use super::meta::MetaProperties;
use super::node::{ASTNode, BlockedContent, Indicator, StyledSyntax};
use super::table::parse_table;
use super::tree::Document;
use crate::lexer::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Parser {
    source: Vec<Token>,
    document: Document,
}

impl Parser {
    pub fn new(source: Vec<Token>) -> Self {
        Self {
            source: source.into_iter().rev().collect(),
            document: Document::init(),
        }
    }
    pub fn parse(mut self) -> Result<Document, String> {
        while !self.at_eof() {
            let mut line: Vec<ASTNode> = Vec::new();
            while !self.at_end_of_line() && !self.at_eof() {
                match self.this_kind() {
                    &TokenKind::MetaData => {
                        self.parse_meta()?;
                    }
                    &TokenKind::OrderedList => {
                        line.push(ASTNode::Indicator {
                            indicate: Indicator::StartOfOrderedList,
                        });
                        let result = self.parse_line()?;
                        line.push(result);
                    }
                    &TokenKind::UnorderedList => {
                        line.push(ASTNode::Indicator {
                            indicate: Indicator::StartOfUnorderedList,
                        });
                        let result = self.parse_line()?;
                        line.push(result);
                    }
                    &TokenKind::Table => {
                        let token = self.consume()?;
                        let src = token.value.expect("Parser: Table should contain a value");
                        parse_table(src, &mut self.document)?;
                    }
                    &TokenKind::BlockMath => {
                        let token = self.consume()?;
                        let src = token
                            .value
                            .expect("Parser: BlockMath should contain a value");
                        line.push(ASTNode::BlockedContent {
                            content: BlockedContent::BlockMath(src),
                        });
                    }
                    &TokenKind::HorizontalLine => {
                        let _ = self.consume()?;
                        line.push(ASTNode::Indicator {
                            indicate: Indicator::HorizontalLine,
                        });
                    }
                    _ => {
                        let result = self.parse_line()?;
                        line.push(result);
                    }
                }
            }
            if self.at_end_of_line() {
                let _ = self.consume()?;
            }
            self.document.append_node(line);
        }
        self.postprocess()
    }

    fn postprocess(mut self) -> Result<Document, String> {
        let mut new_nodes: Vec<Vec<ASTNode>> = Vec::new();
        self.document.nodes = self.document.nodes.into_iter().rev().collect();
        'outer: while !self.document.nodes.is_empty() {
            let mut line = self
                .document
                .nodes
                .pop()
                .expect("Parser: pop not empty vector should be valid.");
            if !line.is_empty()
                && matches!(
                    line[0],
                    ASTNode::Indicator {
                        indicate: Indicator::StartOfOrderedList
                    }
                )
            {
                let indicator = line.remove(0);

                new_nodes.push(vec![indicator]);
                new_nodes.push(line);

                'inner: loop {
                    if self.document.nodes.is_empty() {
                        new_nodes.push(vec![ASTNode::Indicator {
                            indicate: Indicator::EndOfOrderedList,
                        }]);
                        break 'outer;
                    }
                    let line_type: &Vec<ASTNode> =
                        &self.document.nodes[self.document.nodes.len() - 1];

                    if line_type.is_empty() {
                        let _ = self.document.nodes.pop();
                        continue 'inner;
                    }

                    if !matches!(
                        line_type[0],
                        ASTNode::Indicator {
                            indicate: Indicator::StartOfOrderedList
                        }
                    ) {
                        new_nodes.push(vec![ASTNode::Indicator {
                            indicate: Indicator::EndOfOrderedList,
                        }]);
                        break 'inner;
                    } else {
                        let mut line = self
                            .document
                            .nodes
                            .pop()
                            .expect("Parser: pop not empty vector should be valid.");
                        let _ = line.remove(0);
                        new_nodes.push(line);
                        continue 'inner;
                    }
                }
            } else if !line.is_empty()
                && matches!(
                    line[0],
                    ASTNode::Indicator {
                        indicate: Indicator::StartOfUnorderedList
                    }
                )
            {
                let indicator = line.remove(0);

                new_nodes.push(vec![indicator]);
                new_nodes.push(line);

                'inner: loop {
                    if self.document.nodes.is_empty() {
                        new_nodes.push(vec![ASTNode::Indicator {
                            indicate: Indicator::EndOfUnorderedList,
                        }]);
                        break 'outer;
                    }
                    let line_type: &Vec<ASTNode> =
                        &self.document.nodes[self.document.nodes.len() - 1];

                    if line_type.is_empty() {
                        self.document.nodes.pop();
                        continue 'inner;
                    }

                    if !matches!(
                        line_type[0],
                        ASTNode::Indicator {
                            indicate: Indicator::StartOfUnorderedList
                        }
                    ) {
                        new_nodes.push(vec![ASTNode::Indicator {
                            indicate: Indicator::EndOfUnorderedList,
                        }]);
                        break 'inner;
                    } else {
                        let mut line = self
                            .document
                            .nodes
                            .pop()
                            .expect("Parser: pop not empty vector should be valid.");
                        let _ = line.remove(0);
                        new_nodes.push(line);
                        continue 'inner;
                    }
                }
            } else {
                new_nodes.push(line);
            }
        }
        self.document.nodes = new_nodes;
        Ok(self.document)
    }
}

impl Parser {
    fn parse_line(&mut self) -> Result<ASTNode, String> {
        let is_list = if self.this_kind() == &TokenKind::OrderedList
            || self.this_kind() == &TokenKind::UnorderedList
        {
            let _ = self.consume()?;
            true
        } else {
            false
        };

        let syntax = self.parse_syntax()?;
        let content = self.perform_parse()?;

        if is_list {
            Ok(ASTNode::List { syntax, content })
        } else {
            Ok(ASTNode::Inline { syntax, content })
        }
    }
}

impl Parser {
    fn parse_syntax(&mut self) -> Result<Vec<StyledSyntax>, String> {
        let mut syntax: Vec<StyledSyntax> = Vec::new();
        loop {
            match self.this_kind() {
                &TokenKind::CharacterStyle => {
                    let result = self.parse_character_style()?;
                    if let Some(result) = result {
                        syntax.push(result);
                    }
                }
                &TokenKind::Italic => {
                    let _ = self.consume()?;
                    syntax.push(StyledSyntax::Italic);
                }
                &TokenKind::Heading => {
                    let token = self.consume()?;
                    let level = token
                        .value
                        .expect("Parser: Heading should contain a value")
                        .len() as u8;
                    syntax.push(StyledSyntax::Heading(level));
                }
                _ => break,
            }
        }
        Ok(syntax)
    }

    fn parse_character_style(&mut self) -> Result<Option<StyledSyntax>, String> {
        let src = self
            .consume()?
            .value
            .expect("Parser: CharacterStyle should contain a value");
        let result = StyledSyntax::new_style(src);
        if let Err(err) = result {
            crate::warn!("Runtime Warning: {}", err);
            return Ok(None);
        }
        Ok(Some(result.unwrap()))
    }

    fn perform_parse(&mut self) -> Result<Vec<ASTNode>, String> {
        let mut content_element: Vec<ASTNode> = Vec::new();
        while self.this_kind() != &TokenKind::EndOfLine && !self.at_eof() {
            let token_kind = self.this_kind();
            match token_kind {
                &TokenKind::BackSlashLeftParenthesisInline => {
                    let _ = self.consume()?;
                    content_element.push(self.parse_line()?);
                    let _ = self.expect(TokenKind::RightParenthesis)?;
                }
                &TokenKind::RightParenthesis => break,
                &TokenKind::String => {
                    let token = self.consume()?;
                    content_element.push(ASTNode::BlockedContent {
                        content: BlockedContent::PlainText(
                            token.value.expect("Parser: String should contain a value"),
                        ),
                    });
                }
                &TokenKind::LiteralRightParenthesis => {
                    let _ = self.consume()?;
                    content_element.push(ASTNode::BlockedContent {
                        content: BlockedContent::PlainText(String::from(")")),
                    });
                }
                &TokenKind::Bold => {
                    let token = self.consume()?;
                    let src = token.value.expect("Parser: Bold should contain a value");
                    content_element.push(ASTNode::BlockedContent {
                        content: BlockedContent::Bold(src),
                    });
                }
                &TokenKind::Definition => {
                    let token = self.consume()?;
                    let src = token
                        .value
                        .expect("Parser: Definition should contain a value");
                    let src: Vec<&str> = src.split("-@[]").collect();
                    content_element.push(ASTNode::BlockedContent {
                        content: BlockedContent::Definition(src[0].to_string(), src[1].to_string()),
                    });
                }
                &TokenKind::Link => {
                    let token = self.consume()?;
                    let src = token.value.expect("Parser: Link should contain a value");
                    let next_kind = self.this_kind();
                    let content = if next_kind == &TokenKind::String {
                        Some(
                            self.consume()?
                                .value
                                .expect("Parser: String with no internal value"),
                        )
                    } else {
                        None
                    };
                    content_element.push(ASTNode::BlockedContent {
                        content: BlockedContent::Link(src, content),
                    });
                }
                &TokenKind::InlineMath => {
                    let token = self.consume()?;
                    let src = token
                        .value
                        .expect("Parser: InlineMath should contain a value");
                    content_element.push(ASTNode::BlockedContent {
                        content: BlockedContent::InlineMath(src),
                    });
                }
                _ => unreachable!(),
            }
        }
        Ok(content_element)
    }

    fn parse_meta(&mut self) -> Result<(), String> {
        let src = self
            .consume()?
            .value
            .expect("Parser: MetaData should contain a value");
        if let Some(meta) = MetaProperties::new(src) {
            self.document.append_meta(meta);
        }
        Ok(())
    }

    fn consume(&mut self) -> Result<Token, String> {
        if self.at_eof() {
            return Err("Parser: consume: at_eof".to_string());
        }
        Ok(self
            .source
            .pop()
            .expect("Parser: before EOF should always have a token to consume"))
    }

    fn at_eof(&self) -> bool {
        self.source.len() <= 1 || &self.source[self.source.len() - 1].kind == &TokenKind::EOF
    }

    fn at_end_of_line(&self) -> bool {
        &self.source[self.source.len() - 1].kind == &TokenKind::EndOfLine
    }

    fn this_kind(&self) -> &TokenKind {
        &self.source[self.source.len() - 1].kind
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token, String> {
        let token = self.consume()?;
        if token.kind != kind {
            return Err(format!("Parser: expected {:?}, got {:?}", kind, token.kind));
        }
        Ok(token)
    }
}
