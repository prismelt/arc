use super::helper::confirm_overwrite;
use super::helper::find_name_from_txt;
use super::helper::handle_request;
use super::helper::remove_style_for_pdf;
use super::helper::timeout;
use crate::args::command::Args;
use crate::lexer::lexer::Lexer;
use crate::lexer::traits::LexerTrait as _;
use crate::parse::meta::MetaProperties;
use crate::parse::parse::Parser;
use crate::show_err;
use crate::utilities::constants::STD_LIB_DIRECTORY;
use crate::utilities::lib::ce::CE_CONTENT;
use crate::utilities::lib::fmt::FMT_CONTENT;
use crate::utilities::lib::math::MATH_CONTENT;
use crate::utilities::stdout::show_success;
use clap::CommandFactory as _;
use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::net::Shutdown;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc;
use std::time::Duration;
use std::{fs, thread};
use tokio::fs as async_fs;

pub async fn compile(source: PathBuf, output_path: Option<PathBuf>) -> Result<(), String> {
    let src = async_fs::read_to_string(&source)
        .await
        .map_err(|e| format!("Failed to read file {:?}: {}", source, e))?;
    let src_clone = src.clone();
    let tokens = timeout(|| Lexer::new(src).tokenize(), 5000).await?;
    let document = timeout(|| Parser::new(tokens).parse(), 5000).await?;
    let html = document.build();

    if let Some(output_path) = output_path {
        confirm_overwrite(&output_path)?;
        async_fs::write(&output_path, html)
            .await
            .map_err(|e| format!("Failed to write to file: {}", e))?;
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
    confirm_overwrite(&output_path)?;

    async_fs::write(
        &output_path,
        format!("<!-- {} -->\n{}", src_clone, html.replace("\n", "")),
    )
        .await
        .map_err(|e| format!("Failed to write to file: {}", e))?;

    show_success(&format!(
        "Success! HTML saved to {}",
        &output_path.display()
    ));

    Ok(())
}

pub async fn render(source: PathBuf) -> Result<(), String> {
    let listener =
        TcpListener::bind("127.0.0.1:0").map_err(|e| format!("Failed to bind to port: {}", e))?;

    let port = listener
        .local_addr()
        .map_err(|e| format!("Failed to get port: {}", e))?
        .port();

    let url = format!("http://127.0.0.1:{}", port);

    let (shutdown_tx, shutdown_rx) = mpsc::channel();

    let rt_handle = tokio::runtime::Handle::current();
    let source_clone = source.clone();

    let server_handle = thread::spawn(move || {
        listener
            .set_nonblocking(true)
            .expect("Cannot set non-blocking");

        loop {
            if shutdown_rx.try_recv().is_ok() {
                break;
            }

            match listener.accept() {
                Ok((mut stream, _)) => {
                    let res = rt_handle.block_on(handle_request(&mut stream, &source_clone));
                    show_err(res);

                    let _ = stream.shutdown(Shutdown::Both);
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(100));
                    continue;
                }
                Err(e) => {
                    show_err(Err::<(), String>(format!(
                        "Failed to accept connection: {}",
                        e
                    )));
                    break;
                }
            }
        }
    });

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

pub async fn build(
    source: PathBuf,
    output_path: Option<PathBuf>,
    from_html: bool,
) -> Result<(), String> {
    let src = async_fs::read_to_string(&source)
        .await
        .map_err(|e| format!("Failed to read file {:?}: {}", source, e))?;

    let (html, document) = if from_html {
        (src, None)
    } else {
        let tokens = timeout(|| Lexer::new(src).tokenize(), 5000).await?;
        let document = timeout(|| Parser::new(tokens).parse(), 5000).await?;
        (document.build(), Some(document))
    };

    let html: String = remove_style_for_pdf(html);

    let pdf_output_path = if let Some(path) = output_path {
        if path.extension().is_some() && path.extension().unwrap() == "pdf" {
            path
        } else {
            path.with_extension("pdf")
        }
    } else {
        if let Some(document) = document {
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
        } else {
            if let Some(name) = find_name_from_txt(&html)? {
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
        }
    };
    confirm_overwrite(&pdf_output_path)?;

    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .headless(true)
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

    let javascript = format!(
        r#"
        document.open();
        document.write('{}');
        document.close();
    "#,
        html
    );

    tab.evaluate(&javascript, false)
        .map_err(|e| format!("Failed to evaluate JavaScript: {}", e))?;

    std::thread::sleep(std::time::Duration::from_millis(5000));

    let pdf_data = tab
        .print_to_pdf(None)
        .map_err(|e| format!("Failed to print to PDF: {}", e))?;

    async_fs::write(&pdf_output_path, &pdf_data)
        .await
        .map_err(|e| {
            format!(
                "Failed to write PDF to file {} : {}",
                pdf_output_path.display(),
                e
            )
        })?;

    show_success(&format!("PDF saved to {}", pdf_output_path.display()));

    Ok(())
}

pub fn write(command: Option<PathBuf>) -> Result<(), String> {
    if let Some(command) = command {
        let ending = command
            .file_stem()
            .ok_or("Failed to get file stem")?
            .to_str()
            .ok_or("Failed to convert file stem to string")?;
        fs::write(
            format!("{}/{}.txt", STD_LIB_DIRECTORY, ending),
            fs::read_to_string(&command)
                .map_err(|e| format!("Failed to read file {:?}: {}", command, e))?,
        )
            .map_err(|e| format!("Failed to write to file {:?}: {}", command, e))?;

        show_success(&format!("File written to {:?}", command));
        Ok(())
    } else {
        fs::write(format!("{}/fmt.txt", STD_LIB_DIRECTORY), FMT_CONTENT)
            .map_err(|e| format!("Failed to write to file {}: {}", STD_LIB_DIRECTORY, e))?;
        fs::write(format!("{}/math.txt", STD_LIB_DIRECTORY), MATH_CONTENT)
            .map_err(|e| format!("Failed to write to file {}: {}", STD_LIB_DIRECTORY, e))?;
        fs::write(format!("{}/ce.txt", STD_LIB_DIRECTORY), CE_CONTENT)
            .map_err(|e| format!("Failed to write to file {}: {}", STD_LIB_DIRECTORY, e))?;
        show_success("Standard library updated!");
        Ok(())
    }
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
        Some("write") => {
            Args::command()
                .find_subcommand_mut("write")
                .ok_or_else(|| "Failed to find subcommand `write`")?
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
