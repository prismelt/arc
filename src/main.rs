mod args;
mod lexer;
mod parse;
mod test;
mod types;

use args::{
    Args,
    Commands::{Compile, Preview},
};
use clap::Parser as _;
use colored::*;
use lexer::lexer::Lexer;
use lexer::traits::LexerTrait;
use parse::meta::MetaProperties;
use parse::parse::Parser;
use std::path::{Path, PathBuf};

fn transpile(source: PathBuf) -> Result<String, String> {
    use std::fs;
    let src = fs::read_to_string(&source).map_err(|e| format!("Failed to read file: {}", e))?;
    let lexer = Lexer::new(src.clone());
    let tokens = lexer.tokenize();

    let parser = Parser::new(tokens);
    let document = parser.parse();
    let html = document.build();

    Ok(html)
}

fn show_err(res: Result<(), String>) {
    if let Err(err) = res {
        eprintln!("{}", err.bold().red());
    }
}

fn compile(source: PathBuf, output_path: Option<PathBuf>) -> Result<(), String> {
    use std::fs;
    let src = fs::read_to_string(&source).map_err(|e| format!("Failed to read file: {}", e))?;
    let lexer = Lexer::new(src.clone());
    let tokens = lexer.tokenize();

    let parser = Parser::new(tokens);
    let document = parser.parse();
    let html = document.build();

    if let Some(output_path) = output_path {
        fs::write(output_path, html).map_err(|e| format!("Failed to write to file: {}", e))?;
        return Ok(());
    }

    let parent = source.parent().unwrap_or(Path::new(""));
    let source_name = if let Some(MetaProperties::Name(name)) = document
        .meta
        .iter()
        .find(|m| matches!(m, MetaProperties::Name(_)))
    {
        &name
    } else {
        source
            .file_stem()
            .ok_or("Source path has no file name component")?
            .to_str()
            .ok_or("Source filename is not valid UTF-8")?
    };

    let output_path = parent.join(format!("{}.html", source_name));
    fs::write(output_path, format!("<!-- {} -->\n {}", src, html))
        .map_err(|e| format!("Failed to write to file: {}", e))?;

    Ok(())
}

fn render(source: PathBuf) -> Result<(), String> {
    use std::fs;
    use std::process::Command;

    let path = PathBuf::from("/Users/main/arc-preview.html");
    let html = transpile(source)?;
    fs::write(&path, html).map_err(|e| format!("Failed to write to file: {}", e))?;

    Command::new("open")
        .arg(&path)
        .spawn()
        .map_err(|e| format!("Failed to open browser: {}", e))?
        .wait()
        .map_err(|e| format!("Failed to wait for browser: {}", e))?;

    let mut input = String::new();
    println!("\nPress Enter to clean up the temporary file ...");
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Failed to read input: {}", e))?;

    fs::remove_file(&path).map_err(|e| format!("Failed to cleanup file: {}", e))?;

    Ok(())
}

fn main() {
    println!("Arc: Accelerated Markup Language");
    let args = Args::parse();

    let res = match args.command {
        Compile(compile_args) => compile(compile_args.file, compile_args.output),
        Preview(render_args) => render(render_args.file),
    };

    show_err(res);
}
