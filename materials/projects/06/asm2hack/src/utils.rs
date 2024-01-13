use crate::logs::{log_info, log_success};

/// Creates a divider line in the console output.
/// 
/// # Arguments
/// 
/// * `char` - The character to use for the divider.
/// * `count` - The number of times to repeat the character.
fn make_divider(char: char, count: Option<usize>) {
    let count = count.unwrap_or(20);
    log_info(char.to_string().repeat(count).as_str());
}

/// Prints header information for the application.
/// 
/// # Arguments
/// 
/// * `app_name` - The name of the application.
/// * `version` - The version of the application.
/// * `args` - The command line arguments.
pub fn header_info(app_name: &str, version: &str, input: &str, symbolic: bool) {
    log_info(format!("{} v{}", app_name, version).as_str());
    log_info(format!("version: {}", version).as_str());
    make_divider('=', None);
    log_info(format!("input: {}", input).as_str());
    log_info(format!("symbolic: {}", symbolic).as_str());
    make_divider('=', None);
    log_success(format!("output: {}", input).as_str());
}

/// Reads the contents of a file.
///
/// # Arguments
///
/// * `input` - The input file to read.
pub fn read_file(input: &str) -> String {
    log_info(format!("reading file: {}", input).as_str());
    let input_content = std::fs::read_to_string(input).expect("failed to read file");
    input_content
}
