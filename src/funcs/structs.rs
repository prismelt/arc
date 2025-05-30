use fancy_regex::Regex;

pub struct Function {
    name: String,
    param: Option<String>,
    content: Vec<String>,
    regex: Regex,
}

impl Function {
    pub fn new(name: String, parameter: String, content: String) -> Self {
        if name.is_empty() {
            panic!("Function name cannot be empty");
        }
        if parameter.is_empty() {
            Function {
                param: None,
                content: vec![content],
                regex: Self::create_regex(&name),
                name,
            }
        } else {
            Function {
                content: content
                    .split(&("*".to_owned() + &parameter))
                    .map(|s| s.to_string())
                    .collect(),
                param: Some(parameter),
                regex: Self::create_regex(&name),
                name,
            }
        }
    }

    fn create_regex(name: &String) -> Regex {
        let escaped_value = fancy_regex::escape(name);
        let pattern = format!(r"{}\s*\(([^)]*)\)", escaped_value);
        Regex::new(&pattern).unwrap()
    }

    pub fn invoke(&self, context: &mut String) {
        let regex = &self.regex;
        let param = &self.param;
        let replaced = regex
            .replace_all(&context, |capture: &fancy_regex::Captures<'_>| {
                let input_param = capture.get(1).unwrap().as_str();
                if let Some(_) = param {
                    let replaced_string = self.content.join(input_param);
                    replaced_string
                } else {
                    if !(self.content.is_empty()) {
                        panic!("Function {} need to be called with no parameter", self.name);
                    }
                    self.content[0].clone()
                }
            })
            .to_string();
        *context = replaced;
    }
}
