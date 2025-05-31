#![cfg(test)]

use crate::funcs::process::FunctionProcessor;

#[test]
fn test_invalid_script_content() {
    let content = r#"
    <script>
        function foo() {
            return "bar";
        }
        function bar() {
            return "bar";
        }
    </script>
"#;
    let processor = FunctionProcessor::new(content.to_string());
    let result = processor.process();
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        format!(
            "Script content is not fully consumed, suggest invalid function syntax. Reminder: {}",
            content
                .replace("</script>", "")
                .replace("<script>", "")
                .trim()
        )
    );
}

#[test]
fn test_document_with_no_script() {
    let content = r#"
    Hello, world!
    Something else.
"#;
    let processor = FunctionProcessor::new(content.to_string());
    let result = processor.process();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), content);
}

#[test]
fn test_basic_full_function_1() {
    let content = r#"
<script>
fn foo(): bar
</script>
Hello, world!
Something else.
foo()
"#;
    let processor = FunctionProcessor::new(content.to_string());
    let result = processor.process();

    assert_eq!(
        result.unwrap().trim(),
        "Hello, world!\nSomething else.\nbar"
    );
}

#[test]
fn test_basic_full_function_2() {
    let content = r#"
<script>
fn foo(): bar
</script>
Hello, world!
Something else.
foo() and foo() again!
"#;
    let processor = FunctionProcessor::new(content.to_string());
    let result = processor.process();

    assert_eq!(
        result.unwrap().trim(),
        "Hello, world!\nSomething else.\nbar and bar again!"
    );
}

#[test]
fn test_basic_full_function_3() {
    let content = r#"
<script>
fn $foo(*var *var2): bar + *var1 + *var2
</script>
$foo() with no argument should error
"#;
    let processor = FunctionProcessor::new(content.to_string());
    let result = processor.process();
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Invalid function arguments: expect 2,but got 0".to_string()
    );
}

#[test]
fn test_basic_full_function_4() {
    let content = r#"
<script>
fn $foo(*var1 *var2): bar + *var1 + *var2
</script>
$foo(%100 %200)
"#;
    let processor = FunctionProcessor::new(content.to_string());
    let result = processor.process();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().trim(), "bar + 100 + 200");
}

#[test]
fn test_multiple_functions() {
    let content = r#"
<script>
fn $foo(*var1 *var2): bar + *var1 + *var2
fn $bar(*var1 *var2): foo + *var1 + *var2
fn $const(): THIS IS A CONSTANT RETURN VALUE
</script>
$foo(%100 %200)
$bar(%300 %400)
$const()
"#;
    let processor = FunctionProcessor::new(content.to_string());
    let result = processor.process();
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap().trim(),
        "bar + 100 + 200\nfoo + 300 + 400\nTHIS IS A CONSTANT RETURN VALUE"
    );
}

#[test]
#[should_panic]
fn test_edge_case_1() {
    let content = r#"
<script>
fn$foo(*var1 *var2): bar + *var1 + *var2
</script>
$foo(%100 %200)
"#; // info: space between function name and arguments is required
    let processor = FunctionProcessor::new(content.to_string());
    let _ = processor.process().unwrap();
}

#[test]
#[should_panic]
fn test_edge_case_2() {
    let content = r#"
<script>
fn $foo(*var1*var2): bar + *var1 + *var2
</script>
$foo(%100 %200) and $foo(%300 %400)
"#; // info: space between function arguments is required
    let processor = FunctionProcessor::new(content.to_string());
    let _ = processor.process().unwrap();
}

#[test]
#[should_panic]
fn test_edge_case_3() {
    let content = r#"
<script>
fn $foo(*var1 *var2): bar+*var1+*var2
</script>
$foo(%100%200) 
"#; // info: space between arguments is required
    let processor = FunctionProcessor::new(content.to_string());
    let _ = processor.process().unwrap();
}

#[test]
fn test_inline_function_1() {
    let content = r#"
<script>
|*foo| *foo and *foo!
</script>
$foo(100) and $foo(%Hello, %world!)
"#;
    let processor = FunctionProcessor::new(content.to_string());
    let result = processor.process();
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap().trim(),
        "100 and 100! and %Hello, %world! and %Hello, %world!!"
    );
}

#[test]
fn test_inline_function_2() {
    let content = r#"
<script>
|*$foo| *$foo and *$foo!
</script>
$foo(100) and $foo(%Hello, %world!)
"#;
    let processor = FunctionProcessor::new(content.to_string());
    let result = processor.process();
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap().trim(),
        "100 and 100! and %Hello, %world! and %Hello, %world!!"
    );
}
