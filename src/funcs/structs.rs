use fancy_regex::{self as regex, Captures, Regex};
use uuid::Uuid;

pub trait Function {
    fn new(name: String, params: String, content: String) -> Result<Self, String>
    where
        Self: Sized;
    fn invoke(&self, content: &mut String) -> Result<(), String>;
}

pub struct FullFunction {
    name: String,
    params: Vec<String>,
    content: String,
    regex: Regex,
}

pub struct InlineFunction {
    name: String,
    params: String,
    content: String,
    regex: Regex,
}

impl Function for FullFunction {
    fn new(name: String, params: String, content: String) -> Result<Self, String> {
        let params = Self::parse_args(params)?;
        if name.trim().is_empty() {
            return Err(format!("Invalid function name: {}, cannot be empty", name));
        }
        if !name.trim().starts_with("$") {
            crate::warn!(
                "Runtime Warning: function name should start with '$', got {}",
                name
            );
        }
        Ok(FullFunction {
            params,
            content,
            regex: Regex::new(&format!(r"{}\((.*?)\)", regex::escape(&name)))
                .expect("Generated regex should be valid."),
            name: String::from(name.trim()),
        })
    }
    fn invoke(&self, content: &mut String) -> Result<(), String> {
        let mut validation_error: Option<String> = None;
        let result = self.regex.replace_all(content, |capture: &Captures<'_>| {
            let args = capture
                .get(1)
                .expect("Generated regex should have a capture group.")
                .as_str();

            let validate_result = Self::validate(args.to_owned(), &self.params);

            if let Err(e) = validate_result {
                validation_error = Some(e);
                return String::new();
            }

            let input_params = validate_result.unwrap();

            let uuids = input_params
                .iter()
                .map(|_| Uuid::new_v4().to_string())
                .collect::<Vec<String>>();

            let pair = self
                .params
                .iter()
                .zip(input_params.into_iter())
                .zip(uuids)
                .map(|((param, input), id)| (param.clone(), input, format!("__UUID_{}_UUID__", id)))
                .collect::<Vec<(String, String, String)>>();

            let mut content_new = self.content.clone();

            for (param, _, id) in pair.iter() {
                content_new = content_new.replace(&format!("*{}", param), id);
            }
            for (_, input, id) in pair.iter() {
                content_new = content_new.replace(id, input);
            }
            content_new
        });

        if let Some(e) = validation_error {
            return Err(e);
        }
        *content = result.to_string();
        Ok(())
    }
}

impl Function for InlineFunction {
    fn new(name: String, params: String, content: String) -> Result<Self, String> {
        let params = params.trim();
        if params[1..].trim().is_empty() {
            return Err(format!(
                "Invalid function argument / name: {}, cannot be empty",
                params
            ));
        }
        if name.trim().is_empty() {
            return Err(format!(
                "Invalid function argument / name: {}, cannot be empty",
                name
            ));
        }
        let name = if !name.trim().starts_with("$") {
            crate::warn!(
                "Runtime Warning: function name should start with `$`, got {}",
                name
            );
            crate::warn!("Runtime Warning: `$` is automatically added for inline function");
            format!("${}", name.trim())
        } else {
            String::from(name.trim())
        };
        Ok(InlineFunction {
            params: params.replace("*", ""),
            content,
            regex: Regex::new(&format!(r"{}\((.*?)\)", regex::escape(&name)))
                .expect("Generated regex should be valid."),
            name,
        })
    }
    fn invoke(&self, content: &mut String) -> Result<(), String> {
        let mut validation_error: Option<String> = None;
        let result = self.regex.replace_all(content, |capture: &Captures<'_>| {
            let args = capture
                .get(1)
                .expect("Generated regex should have a capture group.")
                .as_str();

            if let Err(e) = Self::validate(args.to_owned()) {
                validation_error = Some(e);
                return String::new();
            }

            let replacement = self.content.replace(&format!("*{}", self.params), args);
            replacement
        });

        if let Some(e) = validation_error {
            return Err(e);
        }
        *content = result.to_string();
        Ok(())
    }
}

impl InlineFunction {
    fn validate(input_params: String) -> Result<(), String> {
        if input_params.replace("%", "").trim().is_empty() {
            return Err(format!(
                "Invalid inline function arguments: {}",
                input_params
            ));
        }
        Ok(())
    }
}

impl FullFunction {
    fn parse_args(args: String) -> Result<Vec<String>, String> {
        let args = args.trim();
        if args.is_empty() {
            return Ok(Vec::new());
        }
        if !(args.starts_with("*")) {
            return Err(format!("Invalid function arguments: {}", args));
        }
        let params: Vec<String> = args[1..]
            .split(" *")
            .map(|s| String::from(s.trim()))
            .collect();

        Ok(params)
    }
    fn validate(input_params: String, params: &Vec<String>) -> Result<Vec<String>, String> {
        let params = params
            .into_iter()
            .filter(|p| !p.is_empty())
            .collect::<Vec<&String>>();

        if input_params.trim().is_empty() && params.len() == 0 {
            return Ok(Vec::new());
        }

        if params.len() == 0 && !input_params.trim().is_empty() {
            return Err(format!(
                "Invalid function arguments: {}, expected no arguments",
                input_params
            ));
        }
        if !input_params.starts_with("%") && params.len() > 0 {
            return Err(format!(
                "Invalid function arguments: expect {},but got {}",
                params.len(),
                input_params
                    .split("%")
                    .filter(|s| !s.trim().is_empty())
                    .count()
            ));
        }

        let input_parameters = input_params[1..]
            .split(" %")
            .map(|s| String::from(s))
            .collect::<Vec<String>>();

        if input_parameters.len() != params.len() {
            return Err(format!(
                "Invalid function arguments: {}, expected {} arguments, got {}",
                input_params,
                params.len(),
                input_parameters.len()
            ));
        }
        Ok(input_parameters)
    }
}
