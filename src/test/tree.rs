#![cfg(test)]

use crate::parse::meta::MetaProperties;
use crate::parse::node::ASTNode;
use crate::parse::node::BlockedContent;
use crate::parse::tree::Document;

#[test]
fn test_document_debug() {
    let document = Document::init();
    assert_eq!(
        format!("{:?}", document),
        "Document { meta: [], nodes: [] }"
    );
}

#[test]
fn test_document_init() {
    let document = Document::init();
    assert_eq!(document.meta.len(), 0);
    assert_eq!(document.nodes.len(), 0);
}

#[test]
fn test_document_append_meta() {
    let mut document = Document::init();
    document.append_meta(MetaProperties::Name("Test".to_string()));
    assert_eq!(document.meta.len(), 1);
    assert_eq!(document.nodes.len(), 0);
}

#[test]
fn test_document_append_node() {
    let mut document = Document::init();
    document.append_node(vec![ASTNode::BlockedContent {
        content: BlockedContent::PlainText("Test".to_string()),
    }]);
    assert_eq!(document.meta.len(), 0);
    assert_eq!(document.nodes.len(), 1);
}
