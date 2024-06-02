use std::fs::read_dir;
use std::io::Write;

use crate::logs::{log_error, log_info, log_success};

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
pub fn header_info(app_name: &str, version: &str, input: &str) {
    log_info(format!("{} v{}", app_name, version).as_str());
    log_info(format!("version: {}", version).as_str());
    make_divider('=', None);
    log_info(format!("input: {}", input).as_str());
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

/// Saves content to a file.
///
/// # Arguments
///
/// * `input` - The input file to read.
///
pub fn save_file(output: &str, content: &Vec<u8>) {
    let mut file = std::fs::File::create(output).expect("failed to create file");

    for byte in content.iter() {
        file.write_all(&[*byte]).expect("failed to write to file");
    }

    log_success(format!("file saved: {}", output).as_str());
}

/// Gets the inputs from a path.
///
/// # Arguments
///
/// * `path` - The path to get inputs from.
///
/// # Returns
///
/// * A vector of strings containing the inputs.
pub fn get_inputs_from_path(path: &str) -> Vec<String> {
    let mut inputs: Vec<String> = Vec::new();

    // read input and assess if it's a file or a folder
    if path.ends_with(".vm") {
        inputs.push(path.to_string());
    } else {
        // read content of the folder under input and get all .vm files
        let files = read_dir(&path);

        // match and assess if the directory is readable
        match files {
            Ok(files) => {
                // add all .vm files to the inputs
                for entry in files {
                    match entry {
                        Ok(file) => {
                            let file_name = file.path().display().to_string();

                            if file_name.ends_with(".vm") {
                                inputs.push(file_name);
                            }
                        }
                        Err(e) => {
                            log_error(format!("failed to read file: {:?}", e).as_str());
                        }
                    }
                }
            }
            Err(e) => {
                log_error(format!("failed to read directory: {:?}", e).as_str());
            }
        }
    }

    inputs
}
