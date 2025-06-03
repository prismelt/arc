mod args;
mod funcs;
mod lexer;
mod parse;
mod test;
mod utilities;
use args::command::{Args, Commands::*};
use args::methods::{build, compile, help, render};
use clap::Parser as _;
use utilities::stdout::show_err;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let res = match args.command {
        Compile(compile_args) => compile(compile_args.file, compile_args.output).await,
        Preview(render_args) => render(render_args.file).await,
        Build(build_args) => build(build_args.file, build_args.output, build_args.html).await,
        Help(help_args) => help(help_args.command),
    };

    show_err(res);
}
