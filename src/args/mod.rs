use clap::{Args as ClapArgs, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None, name = "arc")]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Compile a given file to html")]
    Compile(CompileArgs),

    #[command(about = "Render a given file directly inside the browser")]
    Preview(PreviewArgs),
}

#[derive(ClapArgs)]
pub struct CompileArgs {
    #[arg(help = "Path to the file to compile")]
    pub file: PathBuf,

    #[arg(short, long, help = "Path to the output directory")]
    pub output: Option<PathBuf>,
}

#[derive(ClapArgs)]
pub struct PreviewArgs {
    #[arg(help = "Path to the rendered file")]
    pub file: PathBuf,
}
