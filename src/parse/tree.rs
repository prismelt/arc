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
        .into_string()
        .replace(r#"class="""#, "")
        .replace(r#"style="""#, "")
        .replace("</ol><br />", "</ol>")
        .replace("</ul><br />", "</ul>")
        .replace("</table><br /><br />", "</table>")
        .replace("</li><br />", "</li>")
        .replace("<ol><br />", "<ol>")
        .replace("<ul><br />", "<ul>");
        let regex = Regex::new(ANTI_META_REGEX).unwrap();
        regex.replace_all(&src, "<body>").to_string()
    }
}
