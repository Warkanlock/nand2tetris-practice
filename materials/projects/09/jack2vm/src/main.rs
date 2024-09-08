#![allow(dead_code, unused_imports, unused_variables)]

use clap::Parser as ClapParser;
use jack2vm::logs::{log_info, log_success, log_warn};
use tokenizer::JackTokenizer;
use parser::JackParser;
use std::path::Path;

mod logs;
mod tokenizer;
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
    #[arg(short, long)]
    output: Option<String>,

    /// expect to extract not only the output but the tree structure
    /// result of the syntax analysis step
    #[arg(short, long, default_value_t = false)]
    tree: bool,
}

pub fn main() {
    let args = Args::parse();

    // extract parameters from command line
    let app_name = "jack2vm compiler";
    let version = env!("CARGO_PKG_VERSION");
    let input = args.input;
    let output = args.output;
    let syntax_tree = args.tree;

    // print headers of the program
    utils::header_info(app_name, version, &input);

    // get inputs from path
    let inputs = utils::get_inputs_from_path(&input, "jack");

    // vector to store all the instrcutions across the files
    let mut instructions = Vec::new();

    for input in inputs {
        // read the contents of the input file to be translated to commands
        let content = utils::read_file(&input);

        log_success(&format!("{}: read successfully", input));

        // get filename instead of filepath
        let filename = Path::new(&input)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("");

        // initialize a tokenizer based on the content
        let mut tokenizer: JackTokenizer = JackTokenizer::new(&content, false);

        // obtain across tokens
        tokenizer.tokenize();

        log_success(&format!("{}: parsed successfully", filename));

        if syntax_tree {
            tokenizer.prepare_tree();
        }

        // parse from a list of tokens the correct AST
        let mut parser : JackParser = JackParser::new();
        parser.parse(tokenizer.tokens);

        // add isntructions into instructions vector
        instructions.extend(vec![u8::from(0)])
    }

    if let Some(output) = output {
        // save the output of all the instructions
        utils::save_file(&output, &instructions);

        // exit the program
        std::process::exit(0);
    }

    log_info("no output file specified, using default output file");
    std::process::exit(0);
}
