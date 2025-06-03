use colored::Colorize as _;

#[macro_export]
macro_rules! warn {
    ($fmt:expr $(, $($arg:tt)*)?) => {
        #[allow(unused_imports)]
        use colored::Colorize as _;
        let formatted_message = format!($fmt $(, $($arg)*)?);
        println!("{}", formatted_message.bold().yellow())
    };
}
pub fn show_err<E>(res: Result<E, impl ToString>) {
    use colored::Colorize as _;
    if let Err(err) = res {
        eprintln!("{}", err.to_string().red().bold());
        std::process::exit(1);
    }
}

pub fn show_success(msg: &str) {
    println!("{}", msg.green());
}

pub fn command_style(text: &str) -> String {
    format!("{}", text.green())
}

pub fn arg_style(text: &str) -> String {
    format!("{}", text.cyan())
}
