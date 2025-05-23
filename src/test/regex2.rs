#![cfg(test)]
use crate::types::constants::*;
use fancy_regex::Regex;

#[test]
fn test_table_container_regex_1() {
    let regex = Regex::new(TABLE_CONTAINER_REGEX).unwrap();
    let matched = regex
        .find("---      table!\n\nHello World\n\n---")
        .unwrap()
        .unwrap();
    assert_eq!(matched.as_str(), "---      table!\n\nHello World\n\n---");
}

#[test]
#[should_panic]
fn test_table_container_regex_2() {
    let regex = Regex::new(TABLE_CONTAINER_REGEX).unwrap();
    let _ = regex
        .find("---      table!\n\nHello World\n\n     ---")
        .unwrap()
        .unwrap();
}

#[test]
#[should_panic]
fn test_table_container_regex_3() {
    let regex = Regex::new(TABLE_CONTAINER_REGEX).unwrap();
    let _ = regex.find("---table! Hello World\n\n---").unwrap().unwrap();
}

#[test]
#[should_panic]
fn test_table_container_regex_4() {
    let regex = Regex::new(TABLE_CONTAINER_REGEX).unwrap();
    let _ = regex
        .find("---      table!\n\nHello World---")
        .unwrap()
        .unwrap();
}

#[test]
fn test_table_container_capture_1() {
    let regex = Regex::new(TABLE_CONTAINER_REGEX).unwrap();
    let matched = regex
        .find("---      table!\nHello, world!\nHello World\n\n---")
        .unwrap()
        .unwrap();
    let capture = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(1)
        .unwrap();
    assert_eq!(capture.as_str(), "Hello, world!\nHello World\n");
}

#[test]
fn test_table_container_capture_2() {
    let regex = Regex::new(TABLE_CONTAINER_REGEX).unwrap();
    let matched = regex
        .find("---      table!\nHello, world!\n\nHello World\n---")
        .unwrap()
        .unwrap();
    let capture = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(1)
        .unwrap();
    assert_eq!(capture.as_str(), "Hello, world!\n\nHello World");
}

#[test]
fn test_multiple_newline_regex_1() {
    let regex = Regex::new(MULTIPLE_NEWLINE_REGEX).unwrap();
    let matched = regex.find("\n\n\n\n").unwrap().unwrap();
    assert_eq!(matched.as_str(), "\n\n\n\n");
}

#[test]
#[should_panic]
fn test_multiple_newline_regex_2() {
    let regex = Regex::new(MULTIPLE_NEWLINE_REGEX).unwrap();
    let _ = regex.find("\n").unwrap().unwrap();
}

#[test]
#[should_panic]
fn test_math_1() {
    let regex = Regex::new(INLINE_MATH_REGEX).unwrap();
    let matched = regex.find("<math x = 1>").unwrap().unwrap();
    assert_eq!(matched.as_str(), "<math x = 1>");
}

#[test]
fn test_math_2() {
    let regex = Regex::new(INLINE_MATH_REGEX).unwrap();
    let matched = regex.find("<math x = 1 />").unwrap().unwrap();
    assert_eq!(matched.as_str(), "<math x = 1 />");
}

#[test]
#[should_panic]
fn test_math_regex_capture_1() {
    let regex = Regex::new(INLINE_MATH_REGEX).unwrap();
    let matched = regex.find("<math x = 1>").unwrap().unwrap();
    let capture = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(1)
        .unwrap();
    assert_eq!(capture.as_str(), "x = 1");
}

#[test]
fn test_math_regex_capture_2() {
    let regex = Regex::new(INLINE_MATH_REGEX).unwrap();
    let matched = regex.find("<math x = 1 />").unwrap().unwrap();
    let capture = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(1)
        .unwrap();
    assert_eq!(capture.as_str(), "x = 1");
}

#[test]
fn test_block_math_1() {
    let regex = Regex::new(BLOCK_MATH_REGEX).unwrap();
    let matched = regex.find("<math>\nx = 1\n</math>").unwrap().unwrap();
    assert_eq!(matched.as_str(), "<math>\nx = 1\n</math>");
}

#[test]
fn test_block_math_2() {
    let regex = Regex::new(BLOCK_MATH_REGEX).unwrap();
    let matched = regex.find("<math>\nx = 1\n</math>").unwrap().unwrap();
    assert_eq!(matched.as_str(), "<math>\nx = 1\n</math>");
}

#[test]
fn test_block_math_regex_capture_1() {
    let regex = Regex::new(BLOCK_MATH_REGEX).unwrap();
    let matched = regex.find("<math>\nx = 1\n</math>").unwrap().unwrap();
    let capture = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(1)
        .unwrap();
    assert_eq!(capture.as_str(), "x = 1");
}

#[test]
fn test_anti_meta_regex_1() {
    let regex = Regex::new(ANTI_META_REGEX).unwrap();
    let matched = regex.find("<body><br /><br /><br />").unwrap().unwrap();
    assert_eq!(matched.as_str(), "<body><br /><br /><br />");
}

#[test]
#[should_panic]
fn test_anti_meta_regex_2() {
    let regex = Regex::new(ANTI_META_REGEX).unwrap();
    let _ = regex.find("<body> <br /><br /><br />").unwrap().unwrap();
}
