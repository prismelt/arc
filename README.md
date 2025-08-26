# 🚀 Arc: Accelerated Markup Language ✨

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Markup Language](https://img.shields.io/badge/Language-Markup-blue.svg?style=for-the-badge)
![Status](https://img.shields.io/badge/status-beta-orange.svg?style=for-the-badge)
![MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge)

Welcome to the `Arc` project! 👋

`Arc`, short for **A**ccelerated Ma**r**kup Language, is a new, high-performance markup language designed for speed and
simplicity, as a modern alternative to Markdown. Built with the power and safety of **Rust** 🦀, Arc aims to provide a
robust and efficient way to structure human-oriented content.

Markdown is a great tool, but it has its limitations. Including html-resembled content is dangerous, and advance styling
such as font size and color is difficult to achieve. `Arc` addresses these issues by providing a safe and extensible
markup language.

## 🌟 Features

- ⚡ **Accelerated Parsing:** Leveraging Rust's performance capabilities for fast parsing.
- 🎯 **Simple Syntax:** Easy to learn and write.
- 🛡️ **Memory Safe:** Built on Rust's ownership system.
- ✅ **Extensible:** Designed with future expansion in mind.
- 📄 **Human-Readable:** Clear and concise syntax.
- 🔧 **Function Support:** Write reusable functions and that eradicate boilerplate.

## 📖 Syntax

Compare with Markdown, here's how you write the a simple sentence in `Arc`:

```markdown
Hello World.
```

```arc
Hello World and I can add a <p> tag safely here.
```

Notice that the requirement of two spaces at the end of a line to create a new line is gone. `Arc` automatically track
for all new line characters and insert `<br />` element accordingly. If you do need to write long strings in multiple
lines, use the `\` syntax:

```arc
This is a very long \
string \
so I have to \
break it into multiple lines.
```

Most of your favorite markdown syntax still works in `Arc`, with a lot more features added.

```arc
/// use /// to create comments
%[red] this is how to make the entire line red
%[(255, 0, 0):16:blue] This line now has red text, a font size of 16 and a blue background.
%[red] The line is red but \(%[blue] this is blue) and this is back to red.
/// in fact, you can nesting as much layer as you want
%[red] The line is red but \(%[blue] this is blue and \(%[green] this is green) and this is back to blue) and this is back to red.
```

For a comprehensive list of syntax, check out the [syntax](./SYNTAX.md) page.

## 📦 Installation

(Coming soon - detailed installation instructions will be provided once packages are available.)

For now, you can build from source:

1. Clone the repository:
   ```zsh
   git clone [https://github.com/prismelt/arc.git](https://github.com/prismelt/arc.git)
   ```
2. Navigate to the project directory:
   ```zsh
   cd arc
   ```
3. Build the project:
   ```zsh
   cargo build --release
   ```

## 💡 CLI tool usage

```zsh
arc compile <file> -o <output directory> # compile to html with an optional output path
arc preview <file> # render the file directly in the browser
arc build <file> -o <output directory> # build to pdf with an optional output path
arc write <file> # write the file into standard library directory, if no file is provided, it will updated the formatting library of the stdlib
arc help <command> # print help for a specific command
```

## 📜 License

This project is licensed under the MIT License.

## 👍 Latest Updates

### Update 0.4.2

- Fix table overflow
- Default value for ```arc preview```, ```arc compile``` and ```arc build``` as **new.txt**

### Update 0.4.3 [Aug.25]

- Fix Latex (mjax) font rendering issues
- Default font to Cosmic Neue

### Update 0.4.4 (big improvements) [Aug.25]

- Update for ```arc preview``` to enable reloading
- Add default warning when overwriting existing file
- Add syntax for multi-line functions (macros)
- Remove Redundant whitespaces in the ```arc compile``` html

### Update 0.4.5 (Planning)

- Improved error message
- Add support for ce package of Latex (if possible)
- Minimize ```arc compile``` file size by remove redundant css.
- Increase the usability of the ```<meta>``` tags.
- Add --live tag for ```arc preview``` to enable live reloading