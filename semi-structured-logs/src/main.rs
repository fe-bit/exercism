#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

fn log(log_level: LogLevel, string: &str) {
    match log_level {
        LogLevel::Info => println!("[Info]: {}", string),
        LogLevel::Warning => println!("[Warning]: {}", string),
        LogLevel::Error => println!("[Error]: {}", string),
    }
}

fn info(string: &str) {
    log(LogLevel::Info, string);
}

fn main() {
    log(LogLevel::Error, "Stack overflow");
    info("Timezone changed");
}
