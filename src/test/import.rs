#![cfg(test)]

use crate::funcs::process::FunctionProcessor;

#[test]
fn test_std_import_1() {
    let content = r#"
<script>
@include <std/fmt>
</script>
$rt(Hello, world!)
"#;
    let processor = FunctionProcessor::new(content.to_string());
    let result = processor.process();
    assert!(result.is_ok());
    assert!(result.unwrap().trim().contains(r#"\(%[red]Hello, world!)"#));
}

#[test]
fn test_std_import_2() {
    let content = r#"
<script>
@include <std/math>
@include <std/ce>
</script>
$pi()
$ce(H)
"#;
    let processor = FunctionProcessor::new(content.to_string());
    let result = processor.process();
    assert!(result.is_ok());
    let unwarped = result.unwrap();
    assert!(unwarped.trim().contains("π"));
    assert!(unwarped.trim().contains(r#"<math \mathrm{H} />"#));
}

#[test]
fn test_std_import_3() {
    let content = r#"
<script>
@include <std/fmt>
@include <std/math>
@include <std/ce>
</script>
$rt(Hello, world!)
$pi()
$ce(H)
"#;
    let processor = FunctionProcessor::new(content.to_string());
    let result = processor.process();
    assert!(result.is_ok());
    let unwarped = result.unwrap();
    assert!(unwarped.trim().contains(r#"\(%[red]Hello, world!)"#));
    assert!(unwarped.trim().contains("π"));
    assert!(unwarped.trim().contains(r#"<math \mathrm{H} />"#));
}

#[test]
fn test_custom_import_1() {
    use std::fs;
    fs::write(
        "/tmp/import_test.txt",
        "<script>\nfn $foo(): bar\n</script>",
    )
    .unwrap();
    let content = r#"
<script>
@include </tmp/import_test.txt>
</script>
$foo()
"#;
    let processor = FunctionProcessor::new(content.to_string());
    let result = processor.process();
    fs::remove_file("/tmp/import_test.txt").unwrap();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().trim(), "bar");
}

#[test]
fn test_nonexist_std_import() {
    let content = r#"
<script>
@include <std/nonexist>
</script>
"#;
    let processor = FunctionProcessor::new(content.to_string());
    let result = processor.process();
    // does not error, only throw runtime warning
    assert!(result.is_ok());
    assert_eq!(result.unwrap().trim(), "");
}

#[test]
fn test_nonexist_custom_import() {
    let content = r#"
<script>
@include </tmp/nonexist.txt>
</script>
"#;
    let processor = FunctionProcessor::new(content.to_string());
    let result = processor.process();
    // does not error, only throw runtime warning
    assert!(result.is_ok());
    assert_eq!(result.unwrap().trim(), "");
}
