#![cfg(test)]
use crate::utilities::constants::*;
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

#[test]
fn test_horizontal_line_regex_1() {
    let regex = Regex::new(HORIZONTAL_LINE_REGEX).unwrap();
    let matched = regex.find("---").unwrap().unwrap();
    assert_eq!(matched.as_str(), "---");
}

#[test]
fn test_horizontal_line_regex_2() {
    let regex = Regex::new(HORIZONTAL_LINE_REGEX).unwrap();
    let matched = regex.find("----").unwrap().unwrap();
    assert_eq!(matched.as_str(), "----");
}

#[test]
fn test_horizontal_line_regex_3() {
    let regex = Regex::new(HORIZONTAL_LINE_REGEX).unwrap();
    let matched = regex.find("-----").unwrap().unwrap();
    assert_eq!(matched.as_str(), "-----");
}

#[test]
#[should_panic]
fn test_invalid_horizontal_line_regex_1() {
    let regex = Regex::new(HORIZONTAL_LINE_REGEX).unwrap();
    let _ = regex.find("some text \\(---)").unwrap().unwrap();
}

#[test]
fn test_script_regex_1() {
    let regex = Regex::new(SCRIPT_REGEX).unwrap();
    let matched = regex.find("<script>some text</script>").unwrap().unwrap();
    assert_eq!(matched.as_str(), "<script>some text</script>");
}

#[test]
fn test_script_regex_capture_1() {
    let regex = Regex::new(SCRIPT_REGEX).unwrap();
    let matched = regex
        .find("<script>some text with </ ssscript> </script>")
        .unwrap()
        .unwrap();
    let capture = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(1)
        .unwrap();
    assert_eq!(capture.as_str(), "some text with </ ssscript> ");
}

#[test]
fn test_short_func_regex_1() {
    let regex = Regex::new(SHORT_FUNC_REGEX).unwrap();
    let matched = regex
        .find("|*some text| some text with | in it")
        .unwrap()
        .unwrap();
    assert_eq!(matched.as_str(), "|*some text| some text with | in it");
}

#[test]
fn test_short_func_regex_capture_1() {
    let regex = Regex::new(SHORT_FUNC_REGEX).unwrap();
    let matched = regex
        .find("|*some text| some text with | in it")
        .unwrap()
        .unwrap();
    let capture_1 = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(1)
        .unwrap();
    let capture_2 = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(2)
        .unwrap();
    assert_eq!(capture_1.as_str(), "some text");
    assert_eq!(capture_2.as_str(), "some text with | in it");
}

#[test]
fn test_full_func_regex_1() {
    let regex = Regex::new(FULL_FUNC_REGEX).unwrap();
    let matched = regex
        .find("fn name_here (*some text): some text with (* some text) in it")
        .unwrap()
        .unwrap();
    assert_eq!(
        matched.as_str(),
        "fn name_here (*some text): some text with (* some text) in it"
    );
}

#[test]
fn test_full_func_regex_2() {
    let regex = Regex::new(FULL_FUNC_REGEX).unwrap();
    let matched = regex
        .find("fn name_here (): some text with (* some text) in it")
        .unwrap()
        .unwrap();
    assert_eq!(
        matched.as_str(),
        "fn name_here (): some text with (* some text) in it"
    );
}

#[test]
fn test_full_func_regex_capture_1() {
    let regex = Regex::new(FULL_FUNC_REGEX).unwrap();
    let matched = regex
        .find("fn name_here (*some text): some text with (* some text) in it")
        .unwrap()
        .unwrap();
    let capture_1 = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(1)
        .unwrap();
    let capture_2 = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(2)
        .unwrap();
    let capture_3 = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(3)
        .unwrap();
    assert_eq!(capture_1.as_str(), "name_here");
    assert_eq!(capture_2.as_str(), "*some text");
    assert_eq!(capture_3.as_str(), "some text with (* some text) in it");
}

#[test]
fn test_full_func_regex_capture_2() {
    let regex = Regex::new(FULL_FUNC_REGEX).unwrap();
    let matched = regex
        .find("fn name_here (): some text with (* some text) in it")
        .unwrap()
        .unwrap();
    let capture_1 = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(1)
        .unwrap();
    let capture_2 = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(2)
        .unwrap();
    let capture_3 = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(3)
        .unwrap();
    assert_eq!(capture_1.as_str(), "name_here");
    assert_eq!(capture_2.as_str(), "");
    assert_eq!(capture_3.as_str(), "some text with (* some text) in it");
}

#[test]
fn test_full_func_regex_capture_3() {
    let regex = Regex::new(FULL_FUNC_REGEX).unwrap();
    let matched = regex
        .find("fn name_here (*some text *some other text): some text with (* some text) in it")
        .unwrap()
        .unwrap();
    let capture_1 = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(1)
        .unwrap();
    let capture_2 = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(2)
        .unwrap();
    let capture_3 = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(3)
        .unwrap();
    assert_eq!(capture_1.as_str(), "name_here");
    assert_eq!(capture_2.as_str(), "*some text *some other text");
    assert_eq!(capture_3.as_str(), "some text with (* some text) in it");
}

#[test]
fn test_empty_func() {
    let regex = Regex::new(FULL_FUNC_REGEX).unwrap();
    let matched = regex.find("fn foo():  bar").unwrap().unwrap();
    let capture_1 = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(1)
        .unwrap();
    let capture_2 = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(2)
        .unwrap();
    let capture_3 = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(3)
        .unwrap();
    assert_eq!(capture_1.as_str(), "foo");
    assert_eq!(capture_2.as_str(), "");
    assert_eq!(capture_3.as_str(), "bar");
}

#[test]
fn test_empty_func_2() {
    let regex = Regex::new(FULL_FUNC_REGEX).unwrap();
    let matched = regex.find("fn foo      ():  bar").unwrap().unwrap();
    let capture_1 = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(1)
        .unwrap();
    let capture_2 = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(2)
        .unwrap();
    let capture_3 = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(3)
        .unwrap();
    assert_eq!(capture_1.as_str(), "foo");
    assert_eq!(capture_2.as_str(), "");
    assert_eq!(capture_3.as_str(), "bar");
}

#[test]
fn test_multi_line_scripting() {
    let regex = Regex::new(SCRIPT_REGEX).unwrap();
    let source = "<script>\nHello World\n</script>".to_string();
    let matched = regex.find(&source).unwrap().unwrap();
    assert_eq!(matched.as_str(), "<script>\nHello World\n</script>");
}

#[test]
fn test_code_block_regex_1() {
    let regex = Regex::new(CODE_BLOCK_REGEX).unwrap();
    let matched = regex
        .find("<code>python\nprint('Hello World')\n</code>")
        .unwrap()
        .unwrap();
    assert_eq!(
        matched.as_str(),
        "<code>python\nprint('Hello World')\n</code>"
    );
}

#[test]
fn test_code_block_regex_2() {
    let regex = Regex::new(CODE_BLOCK_REGEX).unwrap();
    let matched = regex
        .find("<code>???\n<, > and /code inside\n</code>")
        .unwrap()
        .unwrap();
    assert_eq!(
        matched.as_str(),
        "<code>???\n<, > and /code inside\n</code>"
    );
}

#[test]
fn test_code_block_regex_3() {
    let regex = Regex::new(CODE_BLOCK_REGEX).unwrap();
    let matched = regex.find("<code>???\n<code/></code>").unwrap().unwrap();
    assert_eq!(matched.as_str(), "<code>???\n<code/></code>");
}

#[test]
fn test_code_block_with_no_language() {
    let regex = Regex::new(CODE_BLOCK_REGEX).unwrap();
    let matched = regex.find("<code>\nHello World\n</code>").unwrap().unwrap();
    assert_eq!(matched.as_str(), "<code>\nHello World\n</code>");
}

#[test]
fn test_code_block_regex_capture_1() {
    let regex = Regex::new(CODE_BLOCK_REGEX).unwrap();
    let matched = regex
        .find("<code>python\nprint('Hello World')\n</code>")
        .unwrap()
        .unwrap();
    let capture_1 = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(1)
        .unwrap();
    let capture_2 = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(2)
        .unwrap();
    assert_eq!(capture_1.as_str().trim(), "python");
    assert_eq!(capture_2.as_str().trim(), "print('Hello World')");
}

#[test]
fn test_code_block_regex_capture_2() {
    let regex = Regex::new(CODE_BLOCK_REGEX).unwrap();
    let matched = regex
        .find("<code>\n<, > and /code inside\n</code>")
        .unwrap()
        .unwrap();
    let capture_1 = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(1)
        .unwrap();
    let capture_2 = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(2)
        .unwrap();
    assert_eq!(capture_1.as_str().trim(), "");
    assert_eq!(capture_2.as_str().trim(), "<, > and /code inside");
}

#[test]
fn test_code_language_regex() {
    let regex = Regex::new(CODE_LANGUAGE_REGEX).unwrap();
    let matched = regex.find(":python").unwrap().unwrap();
    assert_eq!(matched.as_str(), ":python");
}

#[test]
fn test_code_language_regex_2() {
    let regex = Regex::new(CODE_LANGUAGE_REGEX).unwrap();
    let matched = regex.find(":python 'Hello World'").unwrap().unwrap();
    assert_eq!(matched.as_str(), ":python 'Hello World'");
}

#[test]
#[should_panic]
fn test_empty_language() {
    let regex = Regex::new(CODE_LANGUAGE_REGEX).unwrap();
    regex.find(":").unwrap().unwrap();
}

#[test]
fn test_code_language_capture() {
    let regex = Regex::new(CODE_LANGUAGE_REGEX).unwrap();
    let matched = regex.find(":python").unwrap().unwrap();
    let capture_1 = regex
        .captures(matched.as_str())
        .unwrap()
        .unwrap()
        .get(1)
        .unwrap();

    let capture_2 = regex.captures(matched.as_str()).unwrap().unwrap().get(2);

    assert_eq!(capture_1.as_str(), "python");
    assert_eq!(capture_2, None);
}
