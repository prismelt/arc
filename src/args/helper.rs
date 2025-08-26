use crate::lexer::lexer::Lexer;
use crate::lexer::traits::LexerTrait;
use crate::parse::parse::Parser;
use crate::utilities::constants::NAME_REGEX;
use fancy_regex::Regex;
use inquire;
use std::io::Write;
use std::marker::Send;
use std::net::TcpStream;
use std::path::PathBuf;
use std::time::Duration;
use tokio::fs as async_fs;

pub fn confirm_overwrite(path: &PathBuf) -> Result<(), String> {
    if path.exists() {
        let confirm = inquire::Confirm::new(&format!(
            "File {} already exists, overwrite?",
            path.display()
        ))
            .with_default(false)
            .prompt()
            .map_err(|e| format!("Failed to confirm overwrite: {}", e))?;
        if !confirm {
            return Err(format!(
                "File {} already exists, aborting operation.",
                path.display()
            ));
        }
    }
    Ok(())
}

// pub async fn handle_ws(stream: TcpStream, mut reload_rx: tokio::sync::broadcast::Receiver<()>) {
//     let ws_stream = accept_async(stream).await.unwrap();
//     let (mut ws_sender, _) = ws_stream.split();
//
//     tokio::spawn(async move {
//         while reload_rx.recv().await.is_ok() {
//             // send "reload" to client
//             let _ = ws_sender
//                 .send(tokio_tungstenite::tungstenite::Message::Text(
//                     "reload".into(),
//                 ))
//                 .await;
//         }
//     });
// }
//
// pub fn watch_source(path: &PathBuf, reload_tx: Sender<()>) -> Result<(), String> {
//     let tx_clone = reload_tx.clone();
//
//     let mut debouncer =
//         new_debouncer(
//             Duration::from_secs(2),
//             move |res: DebounceEventResult| match res {
//                 Ok(events) => {
//                     for e in events {
//                         let _ = tx_clone.send(());
//                     }
//                 }
//                 Err(e) => eprintln!("watch error: {:?}", e),
//             },
//         )
//             .map_err(|e| format!("Failed to create debouncer: {}", e))?;
//
//     debouncer
//         .watcher()
//         .watch(path, RecursiveMode::Recursive)
//         .map_err(|e| format!("Failed to watch path {:?}: {}", path, e))?;
//
//     Ok(())
// }
pub async fn timeout<T: Send + 'static>(
    task: impl FnOnce() -> Result<T, String> + Send + 'static,
    duration: u64,
) -> Result<T, String> {
    let timeout_duration = Duration::from_millis(duration);
    let actual_task = tokio::task::spawn_blocking(move || task());
    let result = tokio::time::timeout(timeout_duration, actual_task)
        .await
        .map_err(|_| {
            format!(
                "Task timed out after {:.2} seconds",
                timeout_duration.as_secs_f64()
            )
        })?
        .map_err(|e| format!("Task panicked: {}", e))??;
    Ok(result)
}
async fn serve_html(source: &PathBuf) -> Result<String, String> {
    let src = async_fs::read_to_string(&source)
        .await
        .map_err(|e| format!("Failed to read file {:?}: {}", source, e))?;
    let tokens = timeout(|| Lexer::new(src).tokenize(), 5000).await?;
    let document = timeout(|| Parser::new(tokens).parse(), 5000).await?;
    let html = document.build();
    Ok(html)
}

pub async fn handle_request(stream: &mut TcpStream, source: &PathBuf) -> Result<(), String> {
    let html = serve_html(source).await?;
    let body = html.as_bytes();
    let headers = format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Type: text/html; charset=utf-8\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n",
        body.len()
    );

    stream
        .write_all(headers.as_bytes())
        .map_err(|e| format!("Failed to write headers: {}", e))?;
    stream
        .write_all(body)
        .map_err(|e| format!("Failed to write body: {}", e))?;
    stream.flush().ok();
    Ok(())
}

pub fn remove_style_for_pdf(html: String) -> String {
    html.replace("&nbsp;", " ")
        .replace(r"\", r"\\")
        .replace("'", r"\'")
        .replace(r#"""#, r#"\""#)
        .replace("\n", r"\n")
        .replace("\t", r"\t")
        .replace("\t", r"\t")
}

pub fn find_name_from_txt(html: &str) -> Result<Option<String>, String> {
    let regex = Regex::new(NAME_REGEX).expect("Hard coded regex should be valid.");
    let captures = regex
        .captures(html)
        .map_err(|e| format!("Failed to capture: {}", e))?;

    if let Some(capture) = captures {
        let name = capture
            .get(1)
            .expect("Hard coded regex should have a capture group.")
            .as_str();
        Ok(Some(name.to_owned()))
    } else {
        Ok(None)
    }
}
