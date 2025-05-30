#![cfg(test)]

use crate::parse::node::ASTNode;
use crate::parse::node::BlockedContent;

#[test]
fn test_html_building_1() {
    let node = ASTNode::BlockedContent {
        content: BlockedContent::PlainText("Hello World".to_string()),
    };
    let html = node.build();
    assert_eq!(html.into_string(), "<span>Hello World</span>");
}

#[test]
fn test_html_building_2() {
    let node = ASTNode::BlockedContent {
        content: BlockedContent::Bold("Hello World".to_string()),
    };
    let html = node.build();
    assert_eq!(html.into_string(), "<strong>Hello World</strong>");
}

#[test]
fn test_html_building_3() {
    let node = ASTNode::BlockedContent {
        content: BlockedContent::Link(
            "https://www.google.com".to_string(),
            Some("Google".to_string()),
        ),
    };
    let html = node.build();
    assert_eq!(
        html.into_string(),
        "<a href=\"https://www.google.com\">Google</a>"
    );
}

#[test]
fn test_html_building_4() {
    let node = ASTNode::BlockedContent {
        content: BlockedContent::Link("https://www.google.com".to_string(), None),
    };
    let html = node.build();
    assert_eq!(
        html.into_string(),
        "<a href=\"https://www.google.com\">https://www.google.com</a>"
    );
}

#[test]
fn test_html_building_5() {
    let node = ASTNode::BlockedContent {
        content: BlockedContent::Definition("term".to_string(), "definition".to_string()),
    };
    let html = node.build();
    assert_eq!(
        html.into_string(),
        "<span><span style=\"color: red;text-decoration: underline;\">term</span>: <span>definition</span></span>"
    );
}
