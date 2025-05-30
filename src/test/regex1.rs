#![cfg(test)]
use crate::utilities::constants::*;
use fancy_regex::Regex;

#[test]
fn test_newline_regex() {
    let regex = Regex::new(NEWLINE_REGEX).unwrap();
    assert_eq!(regex.find("\n").unwrap().unwrap().as_str(), "\n");
}

#[test]
fn test_consecutive_newline_regex_() {
    let regex = Regex::new(NEWLINE_REGEX).unwrap();
    assert_eq!(regex.find("\n\n\n\n").unwrap().unwrap().as_str(), "\n");
}

#[test]
fn test_meta_regex() {
    let regex = Regex::new(META_DATA_REGEX_LONG).unwrap();
    assert_eq!(
        regex
            .find("<meta name=\"test\"/>")
            .unwrap()
            .unwrap()
            .as_str(),
        "<meta name=\"test\"/>"
    );
}

#[test]
fn test_meta_regex_2() {
    let regex = Regex::new(META_DATA_REGEX_LONG).unwrap();
    assert_eq!(
        regex
            .find("<meta name=\"test\" key=\"value\"/>")
            .unwrap()
            .unwrap()
            .as_str(),
        "<meta name=\"test\" key=\"value\"/>"
    );
}

#[test]
fn test_meta_regex_capture() {
    let regex = Regex::new(META_DATA_REGEX_LONG).unwrap();
    let matched = regex
        .find("<meta name=\"test\" key=\"value\"/>")
        .unwrap()
        .unwrap();
    let capture = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(1)
        .unwrap();
    assert_eq!(capture.as_str(), "name=\"test\" key=\"value\"");
}

#[test]
fn test_meta_regex_capture_2() {
    let regex = Regex::new(META_DATA_REGEX_LONG).unwrap();
    let matched = regex
        .find("<meta name=test but with/> inside/> \n <meta name=\"test\" key=\"value\"/>")
        .unwrap()
        .unwrap();
    let capture = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(1)
        .unwrap();
    assert_eq!(capture.as_str(), "name=test but with/> inside");
}

#[test]
fn test_tilde_regex() {
    let regex = Regex::new(ITALIC_REGEX).unwrap();
    assert_eq!(regex.find("~").unwrap().unwrap().as_str(), "~");
}

#[test]
#[should_panic]
fn test_tilde_regex_2() {
    let regex = Regex::new(ITALIC_REGEX).unwrap();
    assert_eq!(regex.find("~~").unwrap().unwrap().as_str(), "~");
}

#[test]
fn test_right_parenthesis_regex() {
    let regex = Regex::new(RIGHT_PARENTHESIS_REGEX).unwrap();
    assert_eq!(regex.find(")").unwrap().unwrap().as_str(), ")");
}

#[test]
fn test_right_parenthesis_regex_2() {
    let regex = Regex::new(RIGHT_PARENTHESIS_REGEX).unwrap();
    assert_eq!(regex.find("))").unwrap().unwrap().as_str(), ")");
}

#[test]
fn test_literal_right_parenthesis_regex() {
    let regex = Regex::new(LITERAL_RIGHT_PARENTHESIS_REGEX).unwrap();
    assert_eq!(regex.find(r"\)").unwrap().unwrap().as_str(), r"\)");
}

#[test]
fn test_literal_right_parenthesis_regex_2() {
    let regex = Regex::new(LITERAL_RIGHT_PARENTHESIS_REGEX).unwrap();
    assert_eq!(regex.find(r"\\\)").unwrap().unwrap().as_str(), r"\)");
}

#[test]
fn test_backslash_left_parenthesis_regex() {
    let regex = Regex::new(BACKSLASH_LEFT_PARENTHESIS_INLINE_REGEX).unwrap();
    assert_eq!(regex.find(r"\(").unwrap().unwrap().as_str(), r"\(");
}

#[test]
fn test_backslash_left_parenthesis_regex_2() {
    let regex = Regex::new(BACKSLASH_LEFT_PARENTHESIS_INLINE_REGEX).unwrap();
    assert_eq!(regex.find(r"\\(").unwrap().unwrap().as_str(), r"\(");
}

#[test]
fn test_character_style_regex() {
    let regex = Regex::new(CHARACTER_STYLE_REGEX).unwrap();
    assert_eq!(
        regex.find(r"%[test]").unwrap().unwrap().as_str(),
        r"%[test]"
    );
}

#[test]
fn test_character_style_regex_2() {
    let regex = Regex::new(CHARACTER_STYLE_REGEX).unwrap();
    assert_eq!(
        regex
            .find(r"%[test] but with ] inside")
            .unwrap()
            .unwrap()
            .as_str(),
        r"%[test] "
    );
}

#[test]
fn test_non_closing_meta_regex() {
    let regex = Regex::new(META_DATA_REGEX_SHORT).unwrap();
    assert_eq!(
        regex
            .find("<meta name=\"test\" key=\"value\">")
            .unwrap()
            .unwrap()
            .as_str(),
        "<meta name=\"test\" key=\"value\">"
    );
}

#[test]
fn test_two_type_of_meta_regex() {
    let regex_1 = Regex::new(META_DATA_REGEX_LONG).unwrap();
    let regex_2 = Regex::new(META_DATA_REGEX_SHORT).unwrap();

    let input_1 = r#"<meta name="test" key="value"/>"#;
    let input_2 = r#"<meta name="test" key="value">"#;

    let result_1 = regex_1.find(input_1).unwrap().unwrap().as_str();
    let result_2 = regex_2.find(input_2).unwrap().unwrap().as_str();

    assert_eq!(result_1, input_1);
    assert_eq!(result_2, input_2);

    let capture_1 = regex_1.captures(result_1).unwrap().unwrap().get(1).unwrap();
    let capture_2 = regex_2.captures(result_2).unwrap().unwrap().get(1).unwrap();

    assert_eq!(capture_1.as_str(), capture_2.as_str());
}

#[test]
fn test_complex_meta_regex_with_only_slash() {
    let regex_1 = Regex::new(META_DATA_REGEX_LONG).unwrap();
    let regex_2 = Regex::new(META_DATA_REGEX_SHORT).unwrap();

    let input_1 = r#"<meta name="test / value" key="complex / data"/>"#;
    let input_2 = r#"<meta name="test / value" key="complex / data">"#;

    let result_1 = regex_1.find(input_1).unwrap().unwrap().as_str();
    let result_2 = regex_2.find(input_2).unwrap().unwrap().as_str();

    assert_eq!(result_1, input_1);
    assert_eq!(result_2, input_2);

    let capture_1 = regex_1.captures(result_1).unwrap().unwrap().get(1).unwrap();
    let capture_2 = regex_2.captures(result_2).unwrap().unwrap().get(1).unwrap();

    assert_eq!(capture_1.as_str(), capture_2.as_str());
}

#[test]
fn test_complex_meta_regex_with_forward_slash() {
    let regex_1 = Regex::new(META_DATA_REGEX_LONG).unwrap();
    let regex_2 = Regex::new(META_DATA_REGEX_SHORT).unwrap();

    let input_1 = r#"<meta name="test/value" key="complex/data"/>"#;
    let input_2 = r#"<meta name="test/value" key="complex/data">"#;

    let result_1 = regex_1.find(input_1).unwrap().unwrap().as_str();
    let result_2 = regex_2.find(input_2).unwrap().unwrap().as_str();

    assert_eq!(result_1, input_1);
    assert_eq!(result_2, input_2);

    let capture_1 = regex_1.captures(result_1).unwrap().unwrap().get(1).unwrap();
    let capture_2 = regex_2.captures(result_2).unwrap().unwrap().get(1).unwrap();

    assert_eq!(capture_1.as_str(), capture_2.as_str());
}

#[test]
fn test_backslash_end_of_line() {
    let regex = Regex::new(CRLF_REGEX).unwrap();
    assert_eq!(regex.find("\\\n").unwrap().unwrap().as_str(), "\\\n");
}

#[test]
fn test_backslash_end_of_line_2() {
    let regex = Regex::new(CRLF_REGEX).unwrap();
    assert_eq!(
        regex.find("\\    \n").unwrap().unwrap().as_str(),
        "\\    \n"
    );
}

#[test]
fn test_backslash_end_of_line_3() {
    let regex = Regex::new(CRLF_REGEX).unwrap();
    assert_eq!(regex.find("\\ \n").unwrap().unwrap().as_str(), "\\ \n");
}

#[test]
fn test_ordered_list() {
    let regex = Regex::new(ORDERED_LIST_REGEX).unwrap();
    assert_eq!(regex.find("1. Item").unwrap().unwrap().as_str(), "1. ");
    assert_eq!(regex.find("10. Item").unwrap().unwrap().as_str(), "10. ");
    assert_eq!(regex.find("10.Item").unwrap(), None);
}

#[test]
fn test_ordered_list_2() {
    let regex = Regex::new(ORDERED_LIST_REGEX).unwrap();
    assert_eq!(regex.find("1.     Item").unwrap().unwrap().as_str(), "1. ");
    assert_eq!(
        regex.find("10.         Item").unwrap().unwrap().as_str(),
        "10. "
    );
}

#[test]
fn test_unordered_list() {
    let regex = Regex::new(UNORDERED_LIST_REGEX).unwrap();
    assert_eq!(regex.find("- Item").unwrap().unwrap().as_str(), "- ");
    assert_eq!(regex.find("-Item").unwrap(), None);
}

#[test]
fn test_unordered_list_2() {
    let regex = Regex::new(UNORDERED_LIST_REGEX).unwrap();
    assert_eq!(regex.find("-     Item").unwrap().unwrap().as_str(), "- ");
    assert_eq!(
        regex.find("-         Item").unwrap().unwrap().as_str(),
        "- "
    );
}

#[test]
fn test_heading() {
    let regex = Regex::new(HEADING_REGEX).unwrap();
    assert_eq!(regex.find("# Item").unwrap().unwrap().as_str(), "# ");
    assert_eq!(regex.find("## Item").unwrap().unwrap().as_str(), "## ");
    assert_eq!(regex.find("### Item").unwrap().unwrap().as_str(), "### ");
    assert_eq!(regex.find("#### Item").unwrap().unwrap().as_str(), "#### ");
}

#[test]
fn test_heading_2() {
    let regex = Regex::new(HEADING_REGEX).unwrap();
    assert_eq!(regex.find("#     Item").unwrap().unwrap().as_str(), "# ");
    assert_eq!(
        regex.find("##         Item").unwrap().unwrap().as_str(),
        "## "
    );
    assert_eq!(
        regex
            .find("###             Item")
            .unwrap()
            .unwrap()
            .as_str(),
        "### "
    );
    assert_eq!(
        regex
            .find("####                 Item")
            .unwrap()
            .unwrap()
            .as_str(),
        "#### "
    );
}

#[test]
fn test_backslash_with_end_of_line() {
    let regex = Regex::new(CRLF_REGEX).unwrap();
    assert_eq!(regex.find("\\\n").unwrap().unwrap().as_str(), "\\\n");
    assert_eq!(
        regex.find("\\    \n").unwrap().unwrap().as_str(),
        "\\    \n"
    );
    assert_eq!(regex.find("\\ \n").unwrap().unwrap().as_str(), "\\ \n");
}

#[test]
fn test_link() {
    let regex = Regex::new(LINK_REGEX).unwrap();
    let matched = regex
        .find("&[https://www.google.com/path/to/page]")
        .unwrap()
        .unwrap();
    let capture = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(1)
        .unwrap();
    assert_eq!(capture.as_str(), "https://www.google.com/path/to/page");
}

#[test]
fn test_link_2() {
    let regex = Regex::new(LINK_REGEX).unwrap();
    let matched = regex
        .find("&[https://www.google.com/path/to/page?query=string]")
        .unwrap()
        .unwrap();
    let capture = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(1)
        .unwrap();
    assert_eq!(
        capture.as_str(),
        "https://www.google.com/path/to/page?query=string"
    );
}

#[test]
fn test_link_3() {
    let regex = Regex::new(LINK_REGEX).unwrap();
    let matched = regex
        .find("&[https://www.google.com/path/to/page?query=string#anchor]")
        .unwrap()
        .unwrap();
    let capture = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(1)
        .unwrap();
    assert_eq!(
        capture.as_str(),
        "https://www.google.com/path/to/page?query=string#anchor"
    );
}

#[test]
fn test_human_link() {
    let regex = Regex::new(LINK_REGEX).unwrap();
    let matched = regex.find("&[google.com]").unwrap().unwrap();
    let capture = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(1)
        .unwrap();
    assert_eq!(capture.as_str(), "google.com");
}

#[test]
fn test_human_link_2() {
    let regex = Regex::new(LINK_REGEX).unwrap();
    let matched = regex.find("&[google.com/path/to/page]").unwrap().unwrap();
    let capture = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(1)
        .unwrap();
    assert_eq!(capture.as_str(), "google.com/path/to/page");
}

#[test]
fn test_human_link_3() {
    let regex = Regex::new(LINK_REGEX).unwrap();
    let matched = regex
        .find("&[google.com/path/to/page?query=string]")
        .unwrap()
        .unwrap();
    let capture = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(1)
        .unwrap();
    assert_eq!(capture.as_str(), "google.com/path/to/page?query=string");
}

#[test]
fn test_string_regex_1() {
    let regex = Regex::new(STRING_REGEX).unwrap();
    let matched = regex.find("Hello World").unwrap().unwrap();
    assert_eq!(matched.as_str(), "Hello World");
}

#[test]
fn test_string_regex_2() {
    let regex = Regex::new(STRING_REGEX).unwrap();
    let matched = regex.find("Hello World*").unwrap().unwrap();
    assert_eq!(matched.as_str(), "Hello World*");
}

#[test]
fn test_string_regex_3() {
    let regex = Regex::new(STRING_REGEX).unwrap();
    let matched = regex.find("Hello World**").unwrap().unwrap();
    assert_eq!(matched.as_str(), "Hello World");
}

#[test]
fn test_string_regex_4() {
    let regex = Regex::new(STRING_REGEX).unwrap();
    let matched = regex.find("Hello World***").unwrap().unwrap();
    assert_eq!(matched.as_str(), "Hello World");
}

#[test]
fn test_string_regex_5() {
    let regex = Regex::new(STRING_REGEX).unwrap();
    let matched = regex.find("Hello World****").unwrap().unwrap();
    assert_eq!(matched.as_str(), "Hello World");
}

#[test]
fn test_string_regex_6() {
    let regex = Regex::new(STRING_REGEX).unwrap();
    let matched = regex.find(r#"Hello\ World\"#).unwrap().unwrap();
    assert_eq!(matched.as_str(), r#"Hello\ World\"#);
}

#[test]
fn test_string_regex_7() {
    let regex = Regex::new(STRING_REGEX).unwrap();
    let matched = regex.find(r#"Hello) World\*"#).unwrap().unwrap();
    assert_eq!(matched.as_str(), "Hello");
}

#[test]
fn test_string_regex_8() {
    let regex = Regex::new(STRING_REGEX).unwrap();
    let matched = regex.find(r#"Hello ( World"#).unwrap().unwrap();
    assert_eq!(matched.as_str(), "Hello ( World");
}

#[test]
fn test_string_regex_9() {
    let regex = Regex::new(STRING_REGEX).unwrap();
    let matched = regex.find("Hello \n, world!").unwrap().unwrap();
    assert_eq!(matched.as_str(), "Hello ");
}

#[test]
fn test_definition_regex_1() {
    let regex = Regex::new(DEFINITION_REGEX).unwrap();
    let matched = regex.find(r#"@[test] 'test'"#).unwrap().unwrap();
    assert_eq!(matched.as_str(), "@[test] 'test'");
}

#[test]
fn test_definition_regex_2() {
    let regex = Regex::new(DEFINITION_REGEX).unwrap();
    let matched = regex
        .find(r#"@[test, test2, test3] 'test, test2, test3'"#)
        .unwrap()
        .unwrap();
    let captures = regex.captures(matched.as_str()).unwrap().unwrap();
    assert_eq!(captures.get(1).unwrap().as_str(), "test, test2, test3");
    assert_eq!(captures.get(2).unwrap().as_str(), "test, test2, test3");
}

#[test]
fn test_newline_regex_2() {
    let regex = Regex::new(NEWLINE_REGEX).unwrap();
    let matched = regex.find("Hello\nWorld").unwrap().unwrap();
    assert_eq!(matched.as_str(), "\n");
}

#[test]
fn test_bold_text() {
    let regex = Regex::new(BOLD_REGEX).unwrap();
    let matched = regex.find("**bold text**").unwrap().unwrap();
    assert_eq!(matched.as_str(), "**bold text**");
}

#[test]
fn test_bold_text_capture() {
    let regex = Regex::new(BOLD_REGEX).unwrap();
    let matched = regex.find("**bold text**").unwrap().unwrap();
    let capture = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(1)
        .unwrap();
    assert_eq!(capture.as_str(), "bold text");
}

#[test]
fn test_color_regex_capture_1() {
    let regex = Regex::new(CHARACTER_STYLE_REGEX).unwrap();
    let matched = regex.find("%[red:16:(255, 0, 0)]").unwrap().unwrap();
    let capture = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(1)
        .unwrap();
    assert_eq!(capture.as_str(), "red:16:(255, 0, 0)");
}
