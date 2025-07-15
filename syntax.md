# Arc Syntax

Most of the syntax in `Arc` is similar to Markdown, with a few exceptions.

## Comments

Comments are denoted by `///`. Anything after `///` on a line is ignored.

## Meta Data

Meta data is denoted by `<meta key=value />`. It is used to define the document's meta data. The meta data is defined by a series of key-value pairs. The key is defined by the property name, and the value is defined by the property value. The property name and value are separated by an `=` sign. The meta data is terminated by a `/>` sign.

Only the first `=` sign is considered as the separator. Therefore, you can have `=` sign in the value. The `/` at the end is optional, but recommended if you want to include `<>` in the value of the meta data.

The following meta data properties are supported:

- `name`: The name of the file, if not provided, the source's file name will be used.
- `title`: The title of the document. This is used as the title of the document.
- `font-family`: The font family of the document. This is used as the font family of the document.
- `font-size`: The font size of the document. This is used as the font size of the document.
- `font-color`: The font color of the document. This is used as the font color of the document.
- `background-color`: The background color of the document. This is used as the background color of the document.
- `text-font-size`: The font size of the paragraph. This is used as the font size of the paragraph.
- `text-font-color`: The font color of the paragraph. This is used as the font color of the paragraph.
- `h1-font-size`: The font size of the h1 tag. This is used as the font size of the h1 tag.
- `h1-font-color`: The font color of the h1 tag. This is used as the font color of the h1 tag.
- `h2-font-size`: The font size of the h2 tag. This is used as the font size of the h2 tag.
- `h2-font-color`: The font color of the h2 tag. This is used as the font color of the h2 tag.
- `h3-font-size`: The font size of the h3 tag. This is used as the font size of the h3 tag.
- `h3-font-color`: The font color of the h3 tag. This is used as the font color of the h3 tag.
- `h4-font-size`: The font size of the h4 tag. This is used as the font size of the h4 tag.
- `h4-font-color`: The font color of the h4 tag. This is used as the font color of the h4 tag.

## Character Style

The character style is denoted by `%[<font-color> : <font-size> : <background-color>]`. It is used to define the style of the following text. The style syntax is defined by a series of color, font size, and background color. The style syntax is terminated by a `]` sign.

A color can be a rgb value grouped in parentheses, for example, `(255, 0, 0)` is same as `red`.

Syntax such as `%[::<background-color>]` or `%[<font-color>:<font-size>]` is also supported.

The following color literals are valid:

- `red`
- `orange`
- `yellow`
- `green`
- `blue`
- `indigo`
- `violet`
- `black`
- `white`
- `gray`
- `brown`
- `pink`
- `purple`
- `cyan`
- `magenta`
- `lime`
- `teal`
- `maroon`
- `navy`

## Styles Derived from Markdown

The following styles are derived from Markdown and work the same way:

- `**bold**`
- `~italic`
- `@[term] 'definition'`
- `# heading`
- `- unordered list`
- `1. ordered list`
- `&[link] text-to-apply`

More styles are coming soon!

## The `Backslash Left Parenthesis Inline` Syntax

The `Backslash Left Parenthesis Inline` syntax is denoted by `\(inline element)`. It is used to define a inline code block. The inline code block is terminated by a `)` sign.

To include a literal `)` in the text, use `\)` instead.

The inline element is parsed as a separate element, therefore most syntaxes are available inside the inline element. Include a `\(inline element)` will not add a `<br />` to the output html.

Example:

```arc
%[red] element and \(%[blue] element) inside the red element.

%[blue] element and \(~italic element) inside the blue element.

$[red] nesting \($[blue] nesting \($[green] nesting) inside the blue element.) is allowed!
```

Certain syntax does not make sense in the inner context, so they are disabled:

```arc
The nested \(- element) will not be rendered as a list.
The nested \(# heading) is treated as a string.
The nested \(%[red] style) can actually provide style.
```

## The whitespace decision

The whitespace in `Arc` is preserved. This means that if you have any number of consecutive spaces in your document, they will be rendered as is. However, we do want to maintain the readability of the raw document, therefore:

- The whitespace after `#` (heading), `-` (unordered list), and `1.` (ordered list) is mandatory. This means that `#heading` is not a valid heading, but will be rendered as a string `# heading`. This allows leading `#`, `-`, and `1.` to be used in the content without causing any parsing issues.

- The whitespace immediately after `~`, which denotes the start of an italic element, is optional. This means that `~italic` is the same as `~ italic`.

- The whitespace between @[term] and 'definition' is optional. This means that `@[term]'definition'` is the same as `@[term] 'definition'`.

- The whitespace inside the `%[style]` is optional (but `2 50` probably won't work). This means that `%[red]` is the same as `%[ red ]`. The whitespace between the closing `]` and actual content is also optional. This means that `%[red]content` is the same as `%[red] content`.

## Advanced Syntax

### Horizontal Line

The horizontal line is denoted by `---`. Include `---` within `\()`, `table` or other sub-elements will not be rendered as a horizontal line.

### Table

The table is denoted by `--- table!`. The table is terminated by `---`. The table syntax is defined by a series of rows, separated by a newline. Each row is defined by a series of cells, separated by a `;`. The first row is the heading row, and is denoted by `[...]`. The heading row is optional.

The following features are supported:

- Merging cells: use `_` to merge the current cell with the cell to its left, use `^` to merge the current cell with the cell above it.
- Styling: use `%[style]` to style the current cell. The style syntax is the same as the character style.
- The element within each cell is recursively parsed just like a normal inline element. All the syntax allow within `\()` is available within the table cell.
- Notice that creating multiple table can be computational expensive. For each cell, a lexer-lite instance is generated.

### Math

Use `<math ... />` to create an inline math element. Use `<math> ... </math>` to create a block math element. The content inside math element is rendered as is, without any parsing.

### Functions(macros): Meta programming

All the function must be written between the `<script>` and `</script>` tag. You can have as much of them as you want, but keeping them at the top of the document could resolve the issue of unexpected `\n` in the document.

To define a function, use the following syntax:

```arc
fn <name>(*<arg1> *<arg2> ... *<argn>): <body>
```

Then inject the `args` into the body:

```arc
fn <name>(...): <*arg1 *arg2>
```

Other character will be treated as constant:

```arc
fn <name>(...): <*args and after the argument everything is constant>
```

Call the function with:

```arc
<name>(%<arg1> %<arg2> ... %<argn>)
```

For example,

```arc
/// function definition:
<script>
fn $foo(*a *b): A string with *a and *b
</script>

/// function call:
$foo(%1 %2) /// return 'A string with 1 and 2'
$foo(%Hello %World) /// return 'A string with Hello and World'
$foo(%only one argument) /// error!
$foo(%No space between% arguments) /// error!
```

Function could also take no argument:

```arc
<script>
fn $first-program(): Hello, world!
</script>
$first-program() /// return 'Hello, world!'
```

And, of course, add function call inside the content, not just at the start of the line.

```arc
<script>
fn $foo(*a *b): A string with *a and *b
</script>
This is a string with $foo(%1 %2) and $foo(%Hello %World)
/// return 'This is a string with A string with 1 and 2 and A string with Hello and World'
```

The beauty of function is that they can reduce the amount of typing and amount of error you may encounter.

For example:

```arc
<script>
fn $title-style(*a): \(%[white:40:black] *a)
</script>
$title-style(%Hello, world!) /// return '\(%[white:40:black] Hello, world!)'
```

This could eliminate the error writing `%[white:40:black] Hello, world!` to `%white:40:black [Hello, world!]`

We suggest add a `$` prefix to the function name to avoid conflict with actual document content. However, this is not required, but a RuntimeWarning will be invoked without a `$` prefix.

Warning:

Notice naming functions similarly could be dangerous. If one function named `e` and another one named `apple` and `e` is defined before `apple`, then calling `apple()` may actually becomes `appl` + `e()`.

### Simpler Syntax use Inline Functions

In practice, because most function takes only one argument, like this:

```arc
fn $red(*a): \(%[red] *a)
```

We provide a simpler syntax to define such function:

```arc
|*$red| \(%[red] *$red) /// work almost same as the above function
```

In this case, since the function takes only one argument, we can simplify and define the name and args using the same word `$red`.

Because inline function take only one argument, we can also omit the `%` in the calling argument part:

```arc
<script>
|*$red| \(%[red] *$red)
</script>
$red(Hello, world!) /// no % before argument is needed.
/// return '\(%[red] Hello, world!)'
```

### Code Block

The code block is denoted by `<code> ... </code>`. The code block is terminated by `</code>`. The code block syntax is defined by a series of lines, separated by a newline. The first line is the language of the code block, and is denoted by `:language`. The language is optional. The following lines are the content of the code block.
```arc
<code>:langage 'command to execute'
PRINT "HELLO, WORLD!"
</code>
```
To replace part of the command with the content of the code block, use `%code` flag:
```arc
<code>:python 'python -c %code'
print("Hello, world!")
</code>
```
You are then execute it with `arx execute <file> -t <type>`. `Arx` stands for `Arc eXecution`, the sister CLI tool of `arc`. The stdout and stderr will be print to the console.


## Bad Syntax

Bad syntax could cause certain issues in Arc. Most of them will just be treated as string, however, certain error could cause undefined behavior, such as unclosed `\(` and `)` may lead to infinite loop. And invalid math tag could cause panic.

We do try to add timeout to prevent infinite loop, however, it's not actually working yet.

### More syntax coming soon!
