#[macro_export]
macro_rules! warn {
    ($fmt:expr $(, $($arg:tt)*)?) => {
        use colored::Colorize as _;
        let formatted_message = format!($fmt $(, $($arg)*)?);
        eprintln!("{}", formatted_message.bold().yellow());
    };
}
#[macro_export]
macro_rules! success {
    ($fmt:expr $(, $($arg:tt)*)?) => {
        use colored::Colorize as _;
        let formatted_message = format!($fmt $(, $($arg)*)?);
        println!("{}", formatted_message.green());
    };
}
#[macro_export]
macro_rules! error {
    ($fmt:expr $(, $($arg:tt)*)?) => {
        #[allow(unused_imports)]
        use colored::Colorize as _;
        let formatted_message = format!($fmt $(, $($arg)*)?);
        println!("{}", formatted_message.red());
    };
}
pub fn show_err<E>(res: Result<E, impl ToString>) {
    use colored::Colorize as _;
    if let Err(err) = res {
        eprintln!("{}", err.to_string().red().bold());
        std::process::exit(1);
    }
}
