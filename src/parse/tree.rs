use super::meta::MetaProperties;
use super::node::ASTNode;
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
        html!(
            (DOCTYPE)
            html {
                head {
                    (PreEscaped(meta))
                    meta charset="UTF-8";
                    // link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/water.css@2/out/water.css";
                    style { "* { box-sizing: border-box; white-space: pre; }"}
                    style { ".h1size { font-size: 2em; }"}
                    style { ".h2size { font-size: 1.5em; }"}
                    style { ".h3size { font-size: 1.25em; }"}
                    style { ".h4size { font-size: 1.125em; }"}
                }
                body {
                    (PreEscaped(nodes))
                }
            }
        )
        .into_string()
        .replace(r#"class="""#, "")
        .replace(r#"style="""#, "")
    }
}
