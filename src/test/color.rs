#![cfg(test)]

use crate::types::color::Color;

#[test]
fn test_color_from_string_literal_1() {
    let color = Color::from_string("red".to_string()).unwrap();
    assert_eq!(color.build(), "rgb(255, 0, 0)");
}

#[test]
fn test_color_from_string_literal_2() {
    let color = Color::from_string("RED".to_string()).unwrap();
    assert_eq!(color.build(), "rgb(255, 0, 0)");
}

#[test]
fn test_color_from_string_literal_3() {
    let color = Color::from_string("invalid".to_string());
    assert_eq!(color.unwrap_err(), "Invalid color literal: invalid");
}

#[test]
fn test_color_from_string_rgb_1() {
    let color = Color::from_string("(255, 0, 0)".to_string());
    assert_eq!(color.unwrap().build(), "rgb(255, 0, 0)");
}

#[test]
fn test_color_from_string_rgb_2() {
    let color = Color::from_string("(255, 0)".to_string());
    assert_eq!(
        color.unwrap_err(),
        "Insufficient values for rgb literal: (255, 0)"
    );
}

#[test]
fn test_color_from_string_rgb_3() {
    let color = Color::from_string("(255, 0, 0, 0)".to_string());
    assert_eq!(
        color.unwrap_err(),
        "Too many values for rgb literal: (255, 0, 0, 0)"
    );
}

#[test]
fn test_color_from_string_rgb_4() {
    let color = Color::from_string("(256, 0, 0)".to_string());
    assert_eq!(color.unwrap_err(), "Invalid value for rgb literal: 256");
}

#[test]
fn test_color_from_string_rgb_5() {
    let color = Color::from_string("(-1, 0, 0)".to_string());
    assert_eq!(color.unwrap_err(), "Invalid value for rgb literal: -1");
}

#[test]
fn test_color_from_string_rgb_6() {
    let color = Color::from_string("(0, 0, 0)".to_string());
    assert_eq!(color.unwrap().build(), "rgb(0, 0, 0)");
}

#[test]
fn test_color_from_string_rgb_7() {
    let color = Color::from_string("(255, 255, 255)".to_string());
    assert_eq!(color.unwrap().build(), "rgb(255, 255, 255)");
}

#[test]
fn test_color_from_string_rgb_8() {
    let color = Color::from_string("(a, b, c)".to_string());
    assert_eq!(color.unwrap_err(), "Invalid value for rgb literal: a");
}

#[test]
fn test_color_from_string_rgb_9() {
    let color = Color::from_string("()".to_string());
    assert_eq!(
        color.unwrap_err(),
        "Insufficient values for rgb literal: ()"
    );
}

#[test]
fn test_color_from_string_rgb_10() {
    let color = Color::from_string("(,,,)".to_string());
    assert_eq!(color.unwrap_err(), "Too many values for rgb literal: (,,,)");
}

#[test]
fn test_color_from_string_rgb_11() {
    let color = Color::from_string("(255, 0, 0,)".to_string());
    assert_eq!(
        color.unwrap_err(),
        "Too many values for rgb literal: (255, 0, 0,)"
    );
}

#[test]
fn test_color_from_string_rgb_12() {
    let color = Color::from_string("(,,)".to_string());
    assert_eq!(color.unwrap_err(), "Invalid value for rgb literal: ");
}

#[test]
fn test_color_from_string_literal_4() {
    let color = Color::from_string("rEd".to_string()).unwrap();
    assert_eq!(color.build(), "rgb(255, 0, 0)");
}

#[test]
fn test_color_from_string_literal_5() {
    let color = Color::from_string(" red ".to_string()).unwrap();
    assert_eq!(color.build(), "rgb(255, 0, 0)");
}

#[test]
fn test_color_from_string_literal_6() {
    let color = Color::from_string("".to_string());
    assert_eq!(color.unwrap_err(), "Invalid color literal: ");
}
