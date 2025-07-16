/// The ce library provides a set of functions for formatting chemical elements.
/// You can import it using @include <std/ce> in your script tag.
pub const CE_CONTENT: &str = r#"
/// The ce library provides a set of functions for formatting chemical elements.
/// You can import it using @include <std/ce> in your script tag.
<script>
|*$ce| <math \mathrm{*$ce} />
|*$cep| (<math \mathrm{*$cep} />\)
|*$T| \mathrm{*$T}
</script>
"#;
