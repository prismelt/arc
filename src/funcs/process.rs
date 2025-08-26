use crate::funcs::structs::{FullFunction, Function as _, InlineFunction, MultiLineFunction};
use crate::utilities::constants::STD_LIB_DIRECTORY;
use crate::utilities::constants::{
    COMMENT_REGEX, FULL_FUNC_REGEX, IMPORT_REGEX, SCRIPT_REGEX, SHORT_FUNC_REGEX, MULTI_LINE_FN_REGEX,
};
use fancy_regex::{Captures, Regex};
use std::fs;
use std::path::PathBuf;

pub struct FunctionProcessor {
    full_functions: Vec<FullFunction>,
    inline_functions: Vec<InlineFunction>,
    multi_line_functions: Vec<MultiLineFunction>,
    content: String,
}

impl FunctionProcessor {
    pub fn new(content: String) -> Self {
        Self {
            full_functions: Vec::new(),
            inline_functions: Vec::new(),
            multi_line_functions: Vec::new(),
            content,
        }
    }
    pub fn process(mut self) -> Result<String, String> {
        let mut script_content = self.extract_script_content()?;
        self.full_functions = Self::extract_full_functions(&mut script_content)?;
        self.inline_functions = Self::extract_inline_functions(&mut script_content)?;
        self.multi_line_functions =
            Self::extract_multi_line_functions(&mut script_content)?;

        if !(script_content.trim().is_empty()) {
            return Err(format!(
                "Script content is not fully consumed, suggest invalid function syntax. Reminder: {}",
                script_content.trim()
            ));
        }
        // todo: avoid cascading replacement in functions iterations
        for func in self.full_functions {
            func.invoke(&mut self.content)?;
        }
        for func in self.inline_functions {
            func.invoke(&mut self.content)?;
        }
        for func in self.multi_line_functions {
            func.invoke(&mut self.content)?;
        }
        Ok(self.content)
    }

    fn extract_script_content(&mut self) -> Result<String, String> {
        let script_regex = Regex::new(SCRIPT_REGEX).expect("Hard coded regex should be valid.");
        let import_regex = Regex::new(IMPORT_REGEX).expect("Hard coded regex should be valid.");
        let comment_regex = Regex::new(COMMENT_REGEX).expect("Hard coded regex should be valid.");
        let mut fancy_output: Vec<String> = Vec::new();
        let mut matches = script_regex.captures_iter(&self.content);

        loop {
            match matches.next() {
                Some(Ok(m)) => {
                    let capture = m
                        .get(1)
                        .expect("Hard coded regex should have a capture group.")
                        .as_str();
                    fancy_output.push(Self::handle_import(
                        capture,
                        &import_regex,
                        &comment_regex,
                        &script_regex,
                    ));
                }
                None => break,
                Some(Err(e)) => return Err(format!("Regex error: {}", e)),
            }
        }
        let updated = script_regex.replace_all(&self.content, "").to_string();
        self.content = updated;
        Ok(fancy_output.join("\n"))
    }

    fn handle_import(
        content: &str,
        import_regex: &Regex,
        comment_regex: &Regex,
        script_regex: &Regex,
    ) -> String {
        import_regex
            .replace_all(content, |capture: &Captures<'_>| {
                let path = capture
                    .get(1)
                    .expect("Hard coded regex should have a capture group.")
                    .as_str();
                Self::read_import(path, comment_regex, script_regex)
            })
            .to_string()
    }

    fn read_import(path: &str, comment_regex: &Regex, script_regex: &Regex) -> String {
        let path = if path.starts_with("std/") {
            PathBuf::from(format!(
                "{}{}.txt",
                STD_LIB_DIRECTORY,
                path.strip_prefix("std/")
                    .expect("Checked path should start with std/")
            ))
        } else {
            PathBuf::from(path)
        };
        if !path.exists() {
            crate::warn!(
                "Runtime Warning: Import path does not exist: {:?}, recheck import path or try arc write for stdlib importation.",
                path
            );
            return String::new();
        }
        let string =
            fs::read_to_string(path).expect("Failed to read import file, path should exist.");

        let reminder = comment_regex.replace_all(&string, "").to_string();

        let mut fancy_output: Vec<String> = Vec::new();
        let mut matches = script_regex.captures_iter(&reminder);

        loop {
            match matches.next() {
                Some(Ok(m)) => {
                    let capture = m
                        .get(1)
                        .expect("Hard coded regex should have a capture group.")
                        .as_str();
                    fancy_output.push(String::from(capture));
                }
                None => break,
                Some(Err(e)) => {
                    crate::warn!("Runtime Warning: Regex error: {}", e);
                    break;
                }
            }
        }
        fancy_output.join("\n")
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
    
    fn extract_multi_line_functions(content: &mut String) -> Result<Vec<MultiLineFunction>, String> {
        let regex = Regex::new(MULTI_LINE_FN_REGEX).expect("Hard coded regex should be valid.");
        let mut multi_line_functions: Vec<MultiLineFunction> = Vec::new();
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
                    let function = MultiLineFunction::new(
                        String::from(name),
                        String::from(args),
                        String::from(body),
                    )?;
                    multi_line_functions.push(function);
                }
                None => break,
                Some(Err(e)) => return Err(format!("Regex error: {}", e)),
            }
        }
        *content = regex.replace_all(content, "").to_string();
        Ok(multi_line_functions)
    } 
}
