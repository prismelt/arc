pub const STYLE: &str = r#"
* {
  box-sizing: border-box;
  max-width: 100%;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
    "Helvetica Neue", Arial, sans-serif;
  user-select: none;
  -webkit-user-select: none;
  cursor: default;
}
html,
body {
  overscroll-behavior: none;
}
:root {
  --text-color: #2c3e50;
  --background-color: #ffffff;
  --link-color: #3498db;
}
ul,
ol {
  margin-top: 0;
  margin-bottom: 0;
}
body {
  color: var(--text-color);
  line-height: 1.6;
  margin-left: 4rem;
  margin-right: 4rem;
}
a {
  color: var(--link-color);
  text-decoration: none;
  cursor: pointer;
}
a:hover {
  text-decoration: underline !important;
}
span {
  user-select: none;
  margin-top: 0.1em;
  margin-bottom: 0.1em;
  max-width: 100%;
  overflow-wrap: break-word;
  word-break: normal;
  hyphens: auto;
}
.h1size {
  font-size: 2em;
}
.h2size {
  font-size: 1.5em;
}
.h3size {
  font-size: 1.25em;
}
.h4size {
  font-size: 1.125em;
}
.h1size span,
.h2size span,
.h3size span,
.h4size span {
  font-size: inherit !important;
  color: inherit !important;
  background-color: inherit !important;
}
.h1size a,
.h2size a,
.h3size a,
.h4size a {
  font-size: inherit !important;
  background-color: inherit !important;
}
table {
  border: 1px solid #ccc;
  font-family: Arial, sans-serif;
  font-size: 14px;
  border-collapse: collapse;
  margin: 1em 0;
  max-width: 100% !important;
  word-wrap: break-word;
  hyphens: auto;
}
td, th {
  word-break: break-word;
  word-wrap: break-word;
  overflow-wrap: break-word;
  hyphens: auto;
}
table td,
table th {
  border: 1px solid #ccc;
  padding: 10px;
}
table th {
  background-color: #f4f4f4;
}
table tr:nth-child(even) {
  background-color: #f9f9f9;
}
pre {
  background: #272822 !important;
  color: #f8f8f2 !important;
  padding: 1rem !important;
  border-radius: 8px !important;
  overflow-x: auto !important;
}
code {
  font-family: 'Cascadia Code', "JetBrains Mono", Menlo, Monaco, Consolas,
    monospace !important;
  font-variant-ligatures: common-ligatures !important;
}
code span {
  font-family: 'Cascadia Code', "JetBrains Mono", Menlo, Monaco, Consolas,
    monospace !important;
  font-variant-ligatures: common-ligatures !important;
}
"#;
