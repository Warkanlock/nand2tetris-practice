use colored::*;

// remove warnings for unused code
#[allow(dead_code)]
enum MessageTypes { 
    Info,
    Warning,
    Error,
    Success,
    Command,
}

fn _message(msg: &str, msg_type: MessageTypes) {
    // based color depending on message type
    let msg_type = match msg_type {
        MessageTypes::Info => "INFO".cyan().bold(),
        MessageTypes::Warning => "WARNING".yellow().bold(),
        MessageTypes::Error => "ERROR".red().bold(),
        MessageTypes::Success => "SUCCESS".green().bold(),
        MessageTypes::Command => "COMMAND".blue().bold(),
    };

    println!("[{}]: {}", msg_type, msg);
}

#[allow(unused)]
pub fn log_info(msg: &str) {
    _message(msg, MessageTypes::Info);
}

#[allow(unused)]
pub fn log_warn(msg: &str) {
    _message(msg, MessageTypes::Warning);
}

#[allow(unused)]
pub fn log_error(msg: &str) {
    _message(msg, MessageTypes::Error);
}

#[allow(unused)]
pub fn log_success(msg: &str) {
    _message(msg, MessageTypes::Success);
}

#[allow(unused)]
pub fn log_command(msg: &str) {
    _message(msg, MessageTypes::Command);
}
