use super::meta::MetaProperties;
use super::node::ASTNode;
use crate::utilities::constants::ANTI_META_REGEX;
use crate::utilities::style::STYLE;
use fancy_regex::Regex;
use maud::{DOCTYPE, PreEscaped, html};

#[derive(Debug)]
pub struct Document {
    pub meta: Vec<MetaProperties>,
    pub nodes: Vec<Vec<ASTNode>>,
}

impl Document {
    pub fn init() -> Self {
        Self {
            meta: Vec::new(),
            nodes: Vec::new(),
        }
    }

    pub fn append_meta(&mut self, meta: MetaProperties) {
        self.meta.push(meta);
    }

    pub fn append_node(&mut self, node: Vec<ASTNode>) {
        self.nodes.push(node);
    }

    pub fn build(&self) -> String {
        let meta = self
            .meta
            .iter()
            .map(|m| m.build())
            .collect::<Vec<String>>()
            .join("");
        let nodes = self
            .nodes
            .iter()
            .map(|n| {
                n.iter()
                    .map(|n| n.build().into_string())
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("<br />");
        let src = html!(
            (DOCTYPE)
            html lang="en" {
                head {
                    (PreEscaped(meta))
                    meta charset="UTF-8";
                    meta name="viewport" content="width=device-width, initial-scale=1.0";
                    link rel="preconnect" href="https://fonts.googleapis.com";
                    link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                    link href="https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500;700&display=swap" rel="stylesheet";
                    script src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js" {}
                    style { (PreEscaped(STYLE)) }
                }
                body {
                    (PreEscaped(nodes))
                }
            }
        )
        .into_string();
        let src = Self::replace_redundancy(src);
        let src = Self::fix_leading_br(src);
        Self::fix_whitespace(src)
    }

    fn fix_leading_br(src: String) -> String {
        let regex = Regex::new(ANTI_META_REGEX).expect("Hard coded regex should be valid.");
        regex.replace_all(&src, "<body>").to_string()
    }

    fn replace_redundancy(src: String) -> String {
        src.replace(r#"class="""#, "")
            .replace(r#"style="""#, "")
            .replace("</ol><br />", "</ol>")
            .replace("</ul><br />", "</ul>")
            .replace("</table><br /><br />", "</table>")
            .replace("</li><br />", "</li>")
            .replace("<ol><br />", "<ol>")
            .replace("<ul><br />", "<ul>")
    }

    fn fix_whitespace(html: String) -> String {
        let mut result = String::with_capacity(html.len() * 2);
        let chars: Vec<char> = html.chars().collect();
        let mut inside_tag = false;
        let mut inside_quote = false;
        let mut inside_style_or_script = false;
        let mut quote_char = '\0';
        let mut i = 0;

        while i < chars.len() {
            let ch = chars[i];
            let prev_char = if i > 0 { chars[i - 1] } else { '\0' };

            // Check for opening style or script tags
            if ch == '<' && !inside_quote && !inside_style_or_script {
                // Look ahead to see if this is a style or script tag
                let remaining: String = chars[i..].iter().collect();
                if remaining.to_lowercase().starts_with("<style")
                    || remaining.to_lowercase().starts_with("<script")
                {
                    inside_style_or_script = true;
                }
                inside_tag = true;
            }
            // Check for closing style or script tags
            else if ch == '<' && !inside_quote && inside_style_or_script {
                let remaining: String = chars[i..].iter().collect();
                if remaining.to_lowercase().starts_with("</style>")
                    || remaining.to_lowercase().starts_with("</script>")
                {
                    inside_style_or_script = false;
                }
            }
            // Regular tag handling
            else if ch == '>' && !inside_quote {
                inside_tag = false;
            }
            // Quote handling within tags
            else if (ch == '"' || ch == '\'') && inside_tag {
                if !inside_quote {
                    inside_quote = true;
                    quote_char = ch;
                } else if ch == quote_char && prev_char != '\\' {
                    inside_quote = false;
                    quote_char = '\0';
                }
            }

            // Replace space only if we're not inside tags, quotes, or style/script blocks
            if ch == ' ' && !inside_tag && !inside_quote && !inside_style_or_script {
                result.push_str("&nbsp;");
            } else {
                result.push(ch);
            }

            i += 1;
        }

        result
    }
}
