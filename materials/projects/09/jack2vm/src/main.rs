#![allow(dead_code,unused_imports,unused_variables)]

use clap::Parser as ClapParser;
use jack2vm::parser::JackCommand;
use std::path::Path;

mod logs;
mod parser;
mod utils;

/// specific implementation of Jack Compiler
/// to translate jack code (.jack) to virtual machine (.vm)
#[derive(ClapParser, Debug)]
#[command(author = "txxnano", version, about)]
pub struct Args {
    #[arg(short, long)]
    /// input file to use (.vm)
    input: String,

    /// output file to use (.asm)
    #[arg(short, long, default_value = "default")]
    output: String,

    /// bootstrap
    #[arg(short, long)]
    bootstrap: bool,
}

pub fn main() {
    let args = Args::parse();

    // extract parameters from command line
    let app_name = "jack2vm compiler";
    let version = env!("CARGO_PKG_VERSION");
    let input = args.input;
    let output = args.output;

    // print headers of the program
    utils::header_info(app_name, version, &input);

    // get inputs from path
    let inputs = utils::get_inputs_from_path(&input);

    // vector to store all the instrcutions across the files
    let mut instructions = Vec::new();

    for input in inputs {
        // read the contents of the input file to be translated to commands
        let input_content = utils::read_file(&input);

        // get filename instead of filepath
        let input_file = Path::new(&input)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("");

        // add isntructions into instructions vector
        instructions.extend(vec!(u8::from(0)))
    }

    // save the output of all the instructions
    utils::save_file(&output, &instructions);
}