// mod helper;
mod args;
mod lexer;
mod parse;
mod test;
mod types;

use args::Args;
use clap::Parser as _;
use lexer::lexer::Lexer;
use parse::meta::MetaProperties;
use parse::parse::Parser;

fn main() {
    let args = Args::parse();

    let source_dirs = match args.command {
        args::Commands::Compile { file } => file,
    };

    let source_parent = source_dirs.parent().unwrap_or(std::path::Path::new(""));
    let source_name = source_dirs.file_name().unwrap().to_str().unwrap();

    let source = std::fs::read_to_string(&source_dirs).expect("Failed to read file");
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    for token in &tokens {
        println!("{:#?}", token);
    }

    let parser = Parser::new(tokens);
    let document = parser.parse();
    let html = document.build();

    let source_name = if let Some(MetaProperties::Name(name)) = document
        .meta
        .iter()
        .find(|m| matches!(m, MetaProperties::Name(_)))
    {
        name
    } else {
        source_name
    };
    let output_path = source_parent.join(format!("{}.html", source_name));

    std::fs::write(output_path, html).expect("Failed to write to file");
}
