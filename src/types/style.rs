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

span {
  margin: 0 0.15em;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
}


.h1size {
  font-size: 2em;
  white-space: normal !important;
}

.h2size {
  font-size: 1.5em;
  white-space: normal !important;
}

.h3size {
  font-size: 1.25em;
  white-space: normal !important;
}

.h4size {
  font-size: 1.125em;
  white-space: normal !important;
}

.h1size span {
  font-size: inherit !important;
}

.h2size span {
  font-size: inherit !important; 
}

.h3size span {
  font-size: inherit !important; 
}

.h4size span {
  font-size: inherit !important;
}

table {
  border: 1px solid #ccc;
  font-family: Arial, sans-serif;
  font-size: 14px;
  border-collapse: collapse;
  white-space: normal;
  margin: 1em 0;
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
