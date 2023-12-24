use colored::*;

/// Creates a divider line in the console output.
/// 
/// # Arguments
/// 
/// * `char` - The character to use for the divider.
/// * `count` - The number of times to repeat the character.
fn make_divider(char: char, count: usize) {
    println!("{}", char.to_string().repeat(count).cyan().bold());
}

/// Prints header information for the application.
/// 
/// # Arguments
/// 
/// * `app_name` - The name of the application.
/// * `version` - The version of the application.
/// * `args` - The command line arguments.
pub fn header_info(app_name: &str, version: &str, input: &str, symbolic: bool) {
    println!("{}", app_name.green().bold());
    println!("{}", format!("version: {}", version).white().bold());
    make_divider('=', 80);
    println!("{}: {}", "input".cyan().bold(), input);
    println!("{}: {}", "symbolic".cyan().bold(), symbolic);
    make_divider('=', 80);
    println!("{}\n", "output: ".cyan().bold());
}
