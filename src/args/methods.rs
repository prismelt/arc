use crate::args::command::Args;
use crate::lexer::lexer::Lexer;
use crate::lexer::traits::LexerTrait;
use crate::parse::meta::MetaProperties;
use crate::parse::parse::Parser;
use crate::utilities::stdout::show_success;
use clap::CommandFactory as _;
use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::fs;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn compile(source: PathBuf, output_path: Option<PathBuf>) -> Result<(), String> {
    let src = fs::read_to_string(&source).map_err(|e| format!("Failed to read file: {}", e))?;
    let lexer = Lexer::new(src.clone());
    let tokens = lexer.tokenize()?;

    let parser = Parser::new(tokens);
    let document = parser.parse()?;
    let html = document.build();

    if let Some(output_path) = output_path {
        fs::write(&output_path, html).map_err(|e| format!("Failed to write to file: {}", e))?;
        show_success(&format!(
            "Success! HTML saved to {}",
            &output_path.display()
        ));
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
    fs::write(&output_path, format!("<!-- {} -->\n {}", src, html))
        .map_err(|e| format!("Failed to write to file: {}", e))?;

    show_success(&format!(
        "Success! HTML saved to {}",
        &output_path.display()
    ));

    Ok(())
}

pub fn render(source: PathBuf) -> Result<(), String> {
    let src = fs::read_to_string(&source).map_err(|e| format!("Failed to read file: {}", e))?;
    let lexer = Lexer::new(src.clone());
    let tokens = lexer.tokenize()?;

    let parser = Parser::new(tokens);
    let document = parser.parse()?;
    let html = document.build();

    let listener =
        TcpListener::bind("127.0.0.1:0").map_err(|e| format!("Failed to bind to port: {}", e))?;

    let port = listener
        .local_addr()
        .map_err(|e| format!("Failed to get port: {}", e))?
        .port();

    let url = format!("http://127.0.0.1:{}", port);

    let (shutdown_tx, shutdown_rx) = mpsc::channel();

    let server_handle = thread::spawn(move || {
        listener
            .set_nonblocking(true)
            .expect("Cannot set non-blocking");

        loop {
            if shutdown_rx.try_recv().is_ok() {
                break;
            }

            match listener.accept() {
                Ok((stream, _)) => {
                    handle_request(stream, &html);
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(300));
                    continue;
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                    break;
                }
            }
        }
    });

    thread::sleep(Duration::from_millis(100));

    Command::new("open")
        .arg(&url)
        .spawn()
        .map_err(|e| format!("Failed to open browser: {}", e))?;

    show_success(&format!("Preview server running at {}", url));
    show_success("Press Enter to stop the server and exit...");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Failed to read input: {}", e))?;

    let _ = shutdown_tx.send(());

    let _ = server_handle.join();

    show_success("Preview server stopped!");
    Ok(())
}

fn handle_request(mut stream: TcpStream, html: &str) {
    let response = format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Type: text/html; charset=utf-8\r\n\
         Content-Length: {}\r\n\
         Cache-Control: no-cache\r\n\
         Connection: close\r\n\
         \r\n\
         {}",
        html.len(),
        html
    );

    let _ = stream.write_all(response.as_bytes());
    let _ = stream.flush();
}

pub async fn build(source: PathBuf, output_path: Option<PathBuf>) -> Result<(), String> {
    let src = fs::read_to_string(&source).map_err(|e| format!("Failed to read file: {}", e))?;
    let lexer = Lexer::new(src.clone());
    let tokens = lexer.tokenize()?;

    let parser = Parser::new(tokens);
    let document = parser.parse()?;
    let html = document.build();

    let pdf_output_path = if let Some(path) = output_path {
        if path.extension().is_some() && path.extension().unwrap() == "pdf" {
            path
        } else {
            path.with_extension("pdf")
        }
    } else {
        if let Some(MetaProperties::Name(name)) = document
            .meta
            .iter()
            .find(|m| matches!(m, MetaProperties::Name(_)))
        {
            PathBuf::from(name).with_extension("pdf")
        } else {
            let parent = source.parent().unwrap_or(Path::new(""));
            let source_file_stem = source
                .file_stem()
                .ok_or("Source path has no file name component")?
                .to_str()
                .ok_or("Source filename is not valid UTF-8")?;

            parent.join(format!("{}.pdf", source_file_stem))
        }
    };

    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .headless(true) // Set to false for visual debugging
            .build()
            .map_err(|e| format!("Failed to build launch options: {}", e))?,
    )
    .map_err(|e| format!("Failed to launch browser: {}", e))?;

    let tab = browser
        .new_tab()
        .map_err(|e| format!("Failed to create new tab: {}", e))?;

    tab.navigate_to("about:blank")
        .map_err(|e| format!("Failed to navigate to URL: {}", e))?;

    tab.wait_until_navigated()
        .map_err(|e| format!("Failed to wait for navigation: {}", e))?;

    let escaped_html = html
        .replace(r"\", r"\\")
        .replace("'", r"\'")
        .replace(r#"""#, r#"\""#)
        .replace("\n", r"\n")
        .replace("\t", r"\t")
        .replace("\t", r"\t");

    let javascript = format!(
        r#"
        document.open();
        document.write('{}');
        document.close();
    "#,
        escaped_html
    );

    tab.evaluate(&javascript, false)
        .map_err(|e| format!("Failed to evaluate JavaScript: {}", e))?;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    let pdf_data = tab
        .print_to_pdf(None)
        .map_err(|e| format!("Failed to print to PDF: {}", e))?;

    fs::write(&pdf_output_path, &pdf_data).map_err(|e| {
        format!(
            "Failed to write PDF to file {} : {}",
            pdf_output_path.display(),
            e
        )
    })?;

    show_success(&format!("PDF saved to {}", pdf_output_path.display()));

    Ok(())
}

pub fn help(command: Option<String>) -> Result<(), String> {
    match command.as_deref() {
        Some("compile") => {
            Args::command()
                .find_subcommand_mut("compile")
                .ok_or_else(|| "Failed to find subcommand `compile`")?
                .print_help()
                .map_err(|e| format!("Failed to print help: {}", e))?;
            Ok(())
        }
        Some("preview") => {
            Args::command()
                .find_subcommand_mut("preview")
                .ok_or_else(|| "Failed to find subcommand `preview`")?
                .print_help()
                .map_err(|e| format!("Failed to print help: {}", e))?;
            Ok(())
        }
        Some("build") => {
            Args::command()
                .find_subcommand_mut("build")
                .ok_or_else(|| "Failed to find subcommand `build`")?
                .print_help()
                .map_err(|e| format!("Failed to print help: {}", e))?;
            Ok(())
        }
        _ => {
            Args::command()
                .print_help()
                .map_err(|e| format!("Failed to print help: {}", e))?;
            Ok(())
        }
    }
}
