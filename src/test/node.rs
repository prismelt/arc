#![cfg(test)]
use crate::parse::node::StyledSyntax;
use crate::utilities::color::{Color, ColorLiteral};

#[test]
fn test_styled_syntax_debug() {
    let syntax = StyledSyntax::Style((None, None, None));
    assert_eq!(format!("{:?}", syntax), "Style((None, None, None))");
}

#[test]
fn test_styled_syntax_new_style_1() {
    let syntax = StyledSyntax::new_style("::red".to_string());
    assert!(syntax.is_ok());
    let syntax = syntax.unwrap();
    assert!(
        matches!(syntax, StyledSyntax::Style((None, None, Some(color))) if matches!(color, Color::Literal(ColorLiteral::Red)))
    );
}

#[test]
fn test_styled_syntax_new_style_2() {
    let syntax = StyledSyntax::new_style(":16:red".to_string());
    assert!(syntax.is_ok());
    let syntax = syntax.unwrap();
    assert!(
        matches!(syntax, StyledSyntax::Style((None, Some(16), Some(color))) if matches!(color, Color::Literal(ColorLiteral::Red)))
    );
}

#[test]
fn test_styled_syntax_new_style_3() {
    let syntax = StyledSyntax::new_style("red:16".to_string());
    assert!(syntax.is_ok());
    let syntax = syntax.unwrap();
    assert!(
        matches!(syntax, StyledSyntax::Style((Some(color), Some(16), None)) if matches!(color, Color::Literal(ColorLiteral::Red)))
    );
}

#[test]
fn test_styled_syntax_new_style_4() {
    let syntax = StyledSyntax::new_style("red:16:blue".to_string());
    assert!(syntax.is_ok());
    let syntax = syntax.unwrap();
    assert!(
        matches!(syntax, StyledSyntax::Style((Some(color1), Some(16), Some(color2))) if matches!(color1, Color::Literal(ColorLiteral::Red)) && matches!(color2, Color::Literal(ColorLiteral::Blue)))
    );
}

#[test]
fn test_styled_syntax_new_style_5() {
    let syntax = StyledSyntax::new_style("red:16:blue:extra".to_string());
    assert!(syntax.is_err());
}

#[test]
fn test_styled_syntax_new_style_6() {
    let syntax = StyledSyntax::new_style("".to_string());
    assert!(syntax.is_err());
    assert_eq!(syntax.unwrap_err(), "Invalid style syntax: Empty");
}

#[test]
fn test_styled_syntax_new_style_7() {
    let syntax = StyledSyntax::new_style("::".to_string());
    assert!(syntax.is_err());
    assert_eq!(syntax.unwrap_err(), "Invalid style syntax: Empty");
}

#[test]
fn test_styled_syntax_new_style_8() {
    let syntax = StyledSyntax::new_style("red".to_string());
    assert!(syntax.is_ok());
    let syntax = syntax.unwrap();
    assert!(
        matches!(syntax, StyledSyntax::Style((Some(color), None, None)) if matches!(color, Color::Literal(ColorLiteral::Red)))
    );
}

#[test]
fn test_styled_syntax_rgb_1() {
    let syntax = StyledSyntax::new_style("::(255, 0, 0)".to_string());
    assert!(syntax.is_ok());
    let syntax = syntax.unwrap();
    assert!(
        matches!(syntax, StyledSyntax::Style((None, None, Some(color))) if matches!(color, Color::RGB(255, 0, 0)))
    );
}

#[test]
fn test_styled_syntax_rgb_2() {
    let syntax = StyledSyntax::new_style("red:16:(255, 0, 0)".to_string());
    assert!(syntax.is_ok());
    let syntax = syntax.unwrap();
    assert!(
        matches!(syntax, StyledSyntax::Style((Some(color1), Some(16), Some(color2))) if matches!(color1, Color::Literal(ColorLiteral::Red)) && matches!(color2, Color::RGB(255, 0, 0)))
    );
}

#[test]
fn test_styled_syntax_rgb_3() {
    let syntax = StyledSyntax::new_style("(255, 0, 0):16:blue".to_string());
    assert!(syntax.is_ok());
    let syntax = syntax.unwrap();
    assert!(
        matches!(syntax, StyledSyntax::Style((Some(color1), Some(16), Some(color2))) if matches!(color1, Color::RGB(255, 0, 0)) && matches!(color2, Color::Literal(ColorLiteral::Blue)))
    );
}

#[test]
fn test_styled_syntax_rgb_4() {
    let syntax = StyledSyntax::new_style("(255, 0, 0):16:(0, 0, 255)".to_string());
    assert!(syntax.is_ok());
    let syntax = syntax.unwrap();
    assert!(
        matches!(syntax, StyledSyntax::Style((Some(color1), Some(16), Some(color2))) if matches!(color1, Color::RGB(255, 0, 0)) && matches!(color2, Color::RGB(0, 0, 255)))
    );
}

#[test]
fn test_styled_syntax_rgb_5() {
    let syntax = StyledSyntax::new_style("(255, 0, 0)".to_string());
    assert!(syntax.is_ok());
    let syntax = syntax.unwrap();
    assert!(
        matches!(syntax, StyledSyntax::Style((Some(color), None, None)) if matches!(color, Color::RGB(255, 0, 0)))
    );
}

#[test]
fn test_invalid_style_1() {
    let syntax = StyledSyntax::new_style("red:16:blue:extra".to_string());
    assert!(syntax.is_err());
    assert_eq!(
        syntax.unwrap_err(),
        "Invalid style syntax: red:16:blue:extra"
    );
}

#[test]
fn test_invalid_style_2() {
    let syntax = StyledSyntax::new_style("invalid".to_string());
    assert!(syntax.is_err());
    assert_eq!(syntax.unwrap_err(), "Invalid color literal: invalid");
}

#[test]
fn test_invalid_style_3() {
    let syntax = StyledSyntax::new_style("red:invalid".to_string());
    assert!(syntax.is_err());
    assert_eq!(
        syntax.unwrap_err(),
        "Invalid value for font size: 'invalid', msg:`invalid digit found in string`"
    );
}

#[test]
fn test_invalid_style_4() {
    let syntax = StyledSyntax::new_style("red:16:invalid".to_string());
    assert!(syntax.is_err());
    assert_eq!(syntax.unwrap_err(), "Invalid color literal: invalid");
}

#[test]
fn test_invalid_style_5() {
    let syntax = StyledSyntax::new_style("red:16:(255, 0, 0, 0)".to_string());
    assert!(syntax.is_err());
    assert_eq!(
        syntax.unwrap_err(),
        "Too many values for rgb literal: (255, 0, 0, 0)"
    );
}

#[test]
fn test_invalid_style_6() {
    let syntax = StyledSyntax::new_style("red:16:(255, 0)".to_string());
    assert!(syntax.is_err());
    assert_eq!(
        syntax.unwrap_err(),
        "Insufficient values for rgb literal: (255, 0)"
    );
}
