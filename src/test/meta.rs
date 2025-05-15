#![cfg(test)]

use crate::parse::meta::MetaProperties;

#[test]
fn test_from_string_1() {
    let input = "name=My Document";
    let result = MetaProperties::new(String::from(input));
    assert!(matches!(result, Some(MetaProperties::Name(ref s)) if s == "My Document"));

    let input = "font-size=16";
    let result = MetaProperties::new(String::from(input));
    assert!(matches!(result, Some(MetaProperties::FontSize(16))));

    let input = "font-color=(255, 0, 0)";
    let result = MetaProperties::new(String::from(input));
    assert!(matches!(result, Some(MetaProperties::FontColor(_))));

    let input = "allow-html=true";
    let result = MetaProperties::new(String::from(input));
    assert!(matches!(result, Some(MetaProperties::AllowHtml(true))));
}

#[test]
fn test_meta_properties_new_2() {
    let input = "name =My Document";
    let result = MetaProperties::new(String::from(input));
    assert!(matches!(result, Some(MetaProperties::Name(ref s)) if s == "My Document"));

    let input = "font-size= 16 ";
    let result = MetaProperties::new(String::from(input));
    assert!(matches!(result, Some(MetaProperties::FontSize(16))));

    let input = "  font-color= (255, 0, 0)";
    let result = MetaProperties::new(String::from(input));
    assert!(matches!(result, Some(MetaProperties::FontColor(_))));

    let input = "allow-html  =     true";
    let result = MetaProperties::new(String::from(input));
    assert!(matches!(result, Some(MetaProperties::AllowHtml(true))));
}

#[test]
fn test_color_building_process() {
    let input = "font-color=(255, 0, 0)";
    let result = MetaProperties::new(String::from(input));

    if let Some(MetaProperties::FontColor(color)) = result {
        assert_eq!(color.build(), "rgb(255, 0, 0)");
    } else {
        panic!("Expected font-color");
    }
}

#[test]
#[should_panic]
fn test_invalid_meta_property() {
    let input = "invalid=property";
    let _ = MetaProperties::new(String::from(input));
}

#[test]
fn test_invalid_color() {
    let input = "font-color=invalid";
    let result = MetaProperties::new(String::from(input));
    assert!(result.is_none());
}

#[test]
#[should_panic]
fn test_case_sensitivity() {
    let input = "NAME=Test";
    let _ = MetaProperties::new(String::from(input));
}

#[test]
fn test_whitespace_handling() {
    let input = " name = My Document with Spaces ";
    let result = MetaProperties::new(String::from(input));
    assert!(matches!(result, Some(MetaProperties::Name(ref s)) if s == "My Document with Spaces"));

    let input = "font-size =  20  ";
    let result = MetaProperties::new(String::from(input));
    assert!(matches!(result, Some(MetaProperties::FontSize(20))));

    let input = "font-color  =  (100,  150,  200) ";
    let result = MetaProperties::new(String::from(input));
    assert!(matches!(result, Some(MetaProperties::FontColor(_))));

    let input = " allow-html = true ";
    let result = MetaProperties::new(String::from(input));
    assert!(matches!(result, Some(MetaProperties::AllowHtml(true))));
}

#[test]
fn test_empty_values() {
    let input = "name=";
    let result = MetaProperties::new(String::from(input));
    assert!(result.is_none());
}

#[test]
#[should_panic]
fn test_missing_equals_sign() {
    let input = "nameMyDocument";
    let _ = MetaProperties::new(String::from(input));
}

#[test]
fn test_multiple_equals_signs() {
    let input = "name=My=Document";
    let result = MetaProperties::new(String::from(input));
    assert!(matches!(result, Some(MetaProperties::Name(ref s)) if s == "My=Document"));
}

#[test]
#[should_panic]
fn test_invalid_integer_value() {
    let input = "font-size=invalid";
    let _ = MetaProperties::new(String::from(input));
}

#[test]
#[should_panic]
fn test_invalid_boolean_value() {
    let input = "allow-html=invalid";
    let _ = MetaProperties::new(String::from(input));
}

#[test]
fn test_boolean_variations() {
    let input = "allow-html=true";
    let result = MetaProperties::new(String::from(input));
    assert!(matches!(result, Some(MetaProperties::AllowHtml(true))));

    let input = "allow-html=TRUE";
    let result = MetaProperties::new(String::from(input));
    assert!(matches!(result, Some(MetaProperties::AllowHtml(true))));

    let input = "allow-html=false";
    let result = MetaProperties::new(String::from(input));
    assert!(matches!(result, Some(MetaProperties::AllowHtml(false))));

    let input = "allow-html=FALSE";
    let result = MetaProperties::new(String::from(input));
    assert!(matches!(result, Some(MetaProperties::AllowHtml(false))));
}
