pub const STYLE: &str = r#"
* {
  box-sizing: border-box;
  white-space: pre;
  
}

:root {
  --text-color: #2c3e50;
  --background-color: #ffffff;
  --link-color: #3498db;
  --code-background: #f8f9fa;
  --border-color: #e9ecef;
  --blockquote-color: #6c757d;
}

ul, ol {
  margin-top: 0;
  margin-bottom: 0;
}

body {
  color: var(--text-color);
  line-height: 1.6;
}

a {
  color: var(--link-color);
  text-decoration: none;
}

a:hover {
  text-decoration: underline;
}

code {
  background-color: var(--code-background);
  padding: 0.2em 0.4em;
  border-radius: 3px;
  font-family: monospace;
}

blockquote {
  border-left: 4px solid var(--border-color);
  margin: 0;
  padding-left: 1em;
  color: var(--blockquote-color);
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

table {
  border: 1px solid #ccc;
  font-family: Arial, sans-serif;
  font-size: 14px;
  border-collapse: collapse;
  white-space: normal;
}

tbody {
  white-space: normal;
}

table td,
table th {
  border: 1px solid #ccc;
  padding: 10px;
  white-space: normal;
}

table th {
  background-color: #f4f4f4;
}

table tr:nth-child(even) {
  background-color: #f9f9f9;
}

"#;
