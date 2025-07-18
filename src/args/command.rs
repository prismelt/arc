use crate::utilities::stdout::{arg_style, command_style};
use clap::{Args as ClapArgs, ColorChoice, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    author, 
    version, 
    about, 
    long_about = None, 
    name = "Accelerated Markup Language",
    color = ColorChoice::Auto,
    disable_help_subcommand = true,
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = command_style("Compile a given file to html"))]
    Compile(CompileArgs),

    #[command(about = command_style("Render a given file directly inside the browser"))]
    Preview(PreviewArgs),

    #[command(about = command_style("Build and save the output to a PDF file"))]
    Build(BuildArgs),

    #[command(about = command_style("Print this message or the help of the given subcommand(s)"))]
    Help(HelpArgs),

    #[command(about = command_style("Write a file to the standard library"))]
    Write(WriteArgs),
}

#[derive(ClapArgs)]
pub struct CompileArgs {
    #[arg(help = arg_style("Path to the file to compile"), default_value = "new.txt")]
    pub file: PathBuf,

    #[arg(short, long, help = arg_style("Path to the output directory"))]
    pub output: Option<PathBuf>,
}

#[derive(ClapArgs)]
pub struct BuildArgs {
    #[arg(help = arg_style("Path to the file to build"), default_value = "new.txt")]
    pub file: PathBuf,

    #[arg(short, long, help = arg_style("Path to the output directory"))]
    pub output: Option<PathBuf>,

    #[arg(short='H', long="from-html", help = arg_style("Whether build from html"))]
    pub html: bool,
}

#[derive(ClapArgs)]
pub struct PreviewArgs {
    #[arg(help = arg_style("Path to the rendered file"), default_value = "new.txt")]
    pub file: PathBuf,
}

#[derive(ClapArgs)]
pub struct HelpArgs {
    #[arg(help = arg_style("The subcommand whose help message to display"))]
    pub command: Option<String>,
}

#[derive(ClapArgs)]
pub struct WriteArgs {
    #[arg(help = arg_style("Path to the file to write"))]
    pub file: Option<PathBuf>,
}
