use crate::funcs::structs::{FullFunction, Function as _, InlineFunction};
use crate::utilities::constants::{FULL_FUNC_REGEX, SCRIPT_REGEX, SHORT_FUNC_REGEX};
use fancy_regex::Regex;

pub struct FunctionProcessor {
    full_functions: Vec<FullFunction>,
    inline_functions: Vec<InlineFunction>,
    content: String,
}

impl FunctionProcessor {
    pub fn new(content: String) -> Self {
        Self {
            full_functions: Vec::new(),
            inline_functions: Vec::new(),
            content,
        }
    }
    pub fn process(mut self) -> Result<String, String> {
        let mut script_content = self.extract_script_content()?;
        let full_functions = Self::extract_full_functions(&mut script_content)?;
        let inline_functions = Self::extract_inline_functions(&mut script_content)?;

        if !(script_content.trim().is_empty()) {
            return Err(format!(
                "Script content is not fully consumed, suggest invalid function syntax. Reminder: {}",
                script_content.trim()
            ));
        }
        // todo: avoid cascading replacement in functions iterations
        for func in full_functions {
            func.invoke(&mut self.content)?;
        }
        for func in inline_functions {
            func.invoke(&mut self.content)?;
        }
        Ok(self.content)
    }

    fn extract_script_content(&mut self) -> Result<String, String> {
        let regex = Regex::new(SCRIPT_REGEX).expect("Hard coded regex should be valid.");
        let mut fancy_output: Vec<String> = Vec::new();
        let mut matches = regex.captures_iter(&self.content);
        loop {
            match matches.next() {
                Some(Ok(m)) => {
                    let capture = m
                        .get(1)
                        .expect("Hard coded regex should have a capture group.")
                        .as_str();
                    fancy_output.push(capture.to_string());
                }
                None => break,
                Some(Err(e)) => return Err(format!("Regex error: {}", e)),
            }
        }
        let updated = regex.replace_all(&self.content, "").to_string();
        self.content = updated;
        Ok(fancy_output.join("\n"))
    }

    fn extract_full_functions(content: &mut String) -> Result<Vec<FullFunction>, String> {
        let regex = Regex::new(FULL_FUNC_REGEX).expect("Hard coded regex should be valid.");
        let mut full_functions: Vec<FullFunction> = Vec::new();
        let mut matches = regex.captures_iter(content);
        loop {
            match matches.next() {
                Some(Ok(m)) => {
                    let name = m
                        .get(1)
                        .expect("Hard coded regex should have a capture group.")
                        .as_str();
                    let args = m
                        .get(2)
                        .expect("Hard coded regex should have a capture group.")
                        .as_str();
                    let body = m
                        .get(3)
                        .expect("Hard coded regex should have a capture group.")
                        .as_str();
                    let function = FullFunction::new(
                        String::from(name),
                        String::from(args),
                        String::from(body),
                    )?;
                    full_functions.push(function);
                }
                None => break,
                Some(Err(e)) => return Err(format!("Regex error: {}", e)),
            }
        }
        *content = regex.replace_all(content, "").to_string();
        Ok(full_functions)
    }

    fn extract_inline_functions(content: &mut String) -> Result<Vec<InlineFunction>, String> {
        let regex = Regex::new(SHORT_FUNC_REGEX).expect("Hard coded regex should be valid.");
        let mut inline_functions: Vec<InlineFunction> = Vec::new();
        let mut matches = regex.captures_iter(content);
        loop {
            match matches.next() {
                Some(Ok(m)) => {
                    let name = m
                        .get(1)
                        .expect("Hard coded regex should have a capture group.")
                        .as_str();
                    let body = m
                        .get(2)
                        .expect("Hard coded regex should have a capture group.")
                        .as_str();
                    let function = InlineFunction::new(
                        String::from(name),
                        format!("*{}", name), // info: because of regex, add this * safely
                        String::from(body),
                    )?;
                    inline_functions.push(function);
                }
                None => break,
                Some(Err(e)) => return Err(format!("Regex error: {}", e)),
            }
        }
        *content = regex.replace_all(content, "").to_string();
        Ok(inline_functions)
    }
}
