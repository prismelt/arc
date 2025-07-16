// The fmt library provides a set of functions for formatting text.
// You can import it using @include <std/fmt> in your script tag.
pub const FMT_CONTENT: &str = r#"
/// The fmt library provides a set of functions for formatting text.
/// You can import it using @include <std/fmt> in your script tag.
<script>
|*$rt| \(%[red]*$rt)
|*$rb| \(%[red]***$rb**)
|*$ri| \(~%[red]*$ri)

|*$bt| \(%[blue]*$bt)
|*$bb| \(%[blue]***$bb**)
|*$bi| \(~%[blue]*$bi)

|*$gt| \(%[green]*$gt)
|*$gb| \(%[green]***$gb**)
|*$gi| \(~%[green]*$gi)

|*$yt| \(%[yellow]*$yt)
|*$yb| \(%[yellow]***$yb**)
|*$yi| \(~%[yellow]*$yi)

|*$ot| \(%[orange]*$ot)
|*$ob| \(%[orange]***$ob**)
|*$oi| \(~%[orange]*$oi)

|*$pt| \(%[pink]*$pt)
|*$pb| \(%[pink]***$pb**)
|*$pi| \(~%[pink]*$pi)

|*$ct| \(%[cyan]*$ct)
|*$cb| \(%[cyan]***$cb**)
|*$ci| \(~%[cyan]*$ci)

|*$mt| \(%[magenta]*$mt)
|*$mb| \(%[magenta]***$mb**)
|*$mi| \(~%[magenta]*$mi)

|*$lt| \(%[lime]*$lt)
|*$lb| \(%[lime]***$lb**)
|*$li| \(~%[lime]*$li)

|*$tt| \(%[teal]*$tt)
|*$tb| \(%[teal]***$tb**)
|*$ti| \(~%[teal]*$ti)
</script>
"#;
