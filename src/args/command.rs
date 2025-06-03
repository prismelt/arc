use clap::{Args as ClapArgs, ColorChoice, Parser, Subcommand};
use std::path::PathBuf;
use crate::utilities::stdout::{command_style, arg_style};

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
}

#[derive(ClapArgs)]
pub struct CompileArgs {
    #[arg(help = arg_style("Path to the file to compile"))]
    pub file: PathBuf,

    #[arg(short, long, help = arg_style("Path to the output directory"))]
    pub output: Option<PathBuf>,
}

#[derive(ClapArgs)]
pub struct BuildArgs {
    #[arg(help = arg_style("Path to the file to build"))]
    pub file: PathBuf,

    #[arg(short, long, help = arg_style("Path to the output directory"))]
    pub output: Option<PathBuf>,

    #[arg(short='H', long="from-html", help = arg_style("Whether build from html"))]
    pub html: bool,
}

#[derive(ClapArgs)]
pub struct PreviewArgs {
    #[arg(help = arg_style("Path to the rendered file"))]
    pub file: PathBuf,
}

#[derive(ClapArgs)]
pub struct HelpArgs {
    #[arg(help = arg_style("The subcommand whose help message to display"))]
    pub command: Option<String>,
}

