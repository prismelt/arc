// The math library provides a set of functions for performing mathematical operations.
// You can import it using @include <std/math> in your script tag.
pub const MATH_CONTENT: &str = r#"
/// The math library provides a set of functions for performing mathematical operations.
/// You can import it using @include <std/math> in your script tag.
<script>
|*$math| \(<math *$math />)
|*$Math| \(<math> *$Math </math>)
fn $inf(): ♾️
fn $pi(): π

|*$sqrt| \(<math \sqrt{*$sqrt} />)
fn $exp(*a *b): \(<math *$a^{*$b} />)
</script>
"#;
