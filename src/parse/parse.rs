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
        // println!("{:#?}", source);
        // panic!("Parser: a bug occurred, infinite loop");
        Self {
            source: source.into_iter().rev().collect(),
            document: Document::init(),
        }
    }
    pub fn parse(mut self) -> Document {
        while !self.at_eof() {
            let mut line: Vec<ASTNode> = Vec::new();
            while !self.at_end_of_line() && !self.at_eof() {
                match self.this_kind() {
                    &TokenKind::MetaData => {
                        self.parse_meta();
                    }
                    &TokenKind::OrderedList => {
                        line.push(ASTNode::Indicator {
                            indicate: Indicator::StartOfOrderedList,
                        });
                        let result = self.parse_line();
                        line.push(result);
                    }
                    &TokenKind::UnorderedList => {
                        line.push(ASTNode::Indicator {
                            indicate: Indicator::StartOfUnorderedList,
                        });
                        let result = self.parse_line();
                        line.push(result);
                    }
                    &TokenKind::Table => {
                        let token = self.consume();
                        let src = token.value.expect("Parser: Table with no internal value");
                        parse_table(src, &mut self.document);
                    }
                    _ => {
                        let result = self.parse_line();
                        line.push(result);
                    }
                }
            }
            if self.at_end_of_line() {
                let _ = self.consume();
            }
            self.document.append_node(line);
        }
        self.postprocess()
    }

    fn postprocess(mut self) -> Document {
        let mut new_nodes: Vec<Vec<ASTNode>> = Vec::new();
        self.document.nodes = self.document.nodes.into_iter().rev().collect();
        'outer: while !self.document.nodes.is_empty() {
            let mut line = self.document.nodes.pop().unwrap();
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
                    let mut line = self.document.nodes.pop().unwrap();
                    if line.is_empty() {
                        new_nodes.push(line);
                        continue 'inner;
                    } else if !matches!(
                        line[0],
                        ASTNode::Indicator {
                            indicate: Indicator::StartOfOrderedList
                        }
                    ) {
                        new_nodes.push(vec![ASTNode::Indicator {
                            indicate: Indicator::EndOfOrderedList,
                        }]);
                        new_nodes.push(line);
                        break 'inner;
                    } else {
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
                    let mut line = self.document.nodes.pop().unwrap();
                    if line.is_empty() {
                        new_nodes.push(line);
                        continue 'inner;
                    } else if !matches!(
                        line[0],
                        ASTNode::Indicator {
                            indicate: Indicator::StartOfUnorderedList
                        }
                    ) {
                        new_nodes.push(vec![ASTNode::Indicator {
                            indicate: Indicator::EndOfUnorderedList,
                        }]);
                        new_nodes.push(line);
                        break 'inner;
                    } else {
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
        self.document
    }
}

// info: all the parsing method defined here
impl Parser {
    fn parse_line(&mut self) -> ASTNode {
        let is_list = if self.this_kind() == &TokenKind::OrderedList
            || self.this_kind() == &TokenKind::UnorderedList
        {
            let _ = self.consume();
            true
        } else {
            false
        };

        let syntax = self.parse_syntax();
        let content = self.perform_parse();

        if is_list {
            return ASTNode::List { syntax, content };
        }

        ASTNode::Inline { syntax, content }
    }
}

// info: all helper method defined here
impl Parser {
    fn parse_syntax(&mut self) -> Vec<StyledSyntax> {
        let mut syntax: Vec<StyledSyntax> = Vec::new();
        loop {
            match self.this_kind() {
                &TokenKind::CharacterStyle => {
                    let result = self.parse_character_style();
                    if let Some(result) = result {
                        syntax.push(result);
                    }
                }
                &TokenKind::Italic => {
                    let _ = self.consume();
                    syntax.push(StyledSyntax::Italic);
                }
                &TokenKind::Heading => {
                    let token = self.consume();
                    let level = token
                        .value
                        .expect("Parser: Heading with no internal value")
                        .len() as u8;
                    syntax.push(StyledSyntax::Heading(level));
                }
                _ => break,
            }
        }
        syntax
    }

    fn parse_character_style(&mut self) -> Option<StyledSyntax> {
        let src = self
            .consume()
            .value
            .expect("Parser: CharacterStyle with no internal value");
        let result = StyledSyntax::new_style(src);
        if let Err(err) = result {
            eprintln!("Invalid style syntax: {}", err);
            return None;
        }
        Some(result.unwrap())
    }

    fn perform_parse(&mut self) -> Vec<ASTNode> {
        let mut content_element: Vec<ASTNode> = Vec::new();
        while self.this_kind() != &TokenKind::EndOfLine && !self.at_eof() {
            let token_kind = self.this_kind();
            match token_kind {
                &TokenKind::BackSlashLeftParenthesisInline => {
                    let _ = self.consume();
                    content_element.push(self.parse_line());
                    let _ = self.expect(TokenKind::RightParenthesis);
                }
                &TokenKind::RightParenthesis => break,
                &TokenKind::String => {
                    let token = self.consume();
                    content_element.push(ASTNode::BlockedContent {
                        content: BlockedContent::PlainText(
                            token.value.expect("Invalid expected string value"),
                        ),
                    });
                }
                &TokenKind::LiteralRightParenthesis => {
                    let _ = self.consume();
                    content_element.push(ASTNode::BlockedContent {
                        content: BlockedContent::PlainText(String::from(")")),
                    });
                }
                &TokenKind::Bold => {
                    let token = self.consume();
                    let src = token.value.expect("Parser: Bold with no internal value");
                    content_element.push(ASTNode::BlockedContent {
                        content: BlockedContent::Bold(src),
                    });
                }
                &TokenKind::Definition => {
                    let token = self.consume();
                    let src = token
                        .value
                        .expect("Parser: Definition with no internal value");
                    let src: Vec<&str> = src.split("-@[]").collect();
                    content_element.push(ASTNode::BlockedContent {
                        content: BlockedContent::Definition(src[0].to_string(), src[1].to_string()),
                    });
                }
                &TokenKind::Link => {
                    let token = self.consume();
                    let src = token.value.expect("Parser: Link with no internal value");
                    let next_kind = self.this_kind();
                    let content = if next_kind == &TokenKind::String {
                        Some(
                            self.consume()
                                .value
                                .expect("String Token with no internal value"),
                        )
                    } else {
                        None
                    };
                    content_element.push(ASTNode::BlockedContent {
                        content: BlockedContent::Link(src, content),
                    });
                }
                _ => unreachable!(),
            }
        }
        content_element
    }

    fn parse_meta(&mut self) {
        let src = self
            .consume()
            .value
            .expect("Parser: MetaData with no internal value");
        if let Some(meta) = MetaProperties::new(src.clone()) {
            self.document.append_meta(meta);
        } else {
            eprintln!("Invalid <meta /> tag: {}", src);
        }
    }

    fn consume(&mut self) -> Token {
        if self.at_eof() {
            panic!("Parser: consume: at_eof");
        }
        self.source.pop().unwrap()
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

    // fn next_kind(&self) -> Option<&TokenKind> {
    //     if self.source.len() == 1 {
    //         return None;
    //     }
    //     Some(&self.source[self.source.len() - 2].kind)
    // }

    fn expect(&mut self, kind: TokenKind) -> Token {
        let token = self.consume();
        if token.kind != kind {
            panic!("Parser: expected {:?}, got {:?}", kind, token.kind);
        }
        // debug(format!("Reached expected token: {:?}", kind));
        token
    }
}
