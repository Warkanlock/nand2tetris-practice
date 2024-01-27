use clap::Parser as ClapParser;
use code::{binary_instructions_to_bytes, process_fields};
use logs::log_command;

// module definitions
mod code;
mod logs;
mod parser;
mod utils;

/// interface to assemble Hack assembly language programs into binary code
/// for execution in the Hack hardware platform
#[derive(ClapParser, Debug)]
#[command(author = "txxnano", version, about)]
pub struct Args {
    #[arg(short, long, default_value_t = false)]
    /// use symbolic links instead of absolute reference memory addresses (default: false)
    symbolic: bool,

    #[arg(short, long)]
    /// input file to use (.asm)
    input: String,

    /// output file to use (.hack)
    #[arg(short, long, default_value = "default")]
    output: String,
}

pub fn main() {
    let args = Args::parse();

    // extract parameters from command line
    let app_name = "asm2hack assembler";
    let version = env!("CARGO_PKG_VERSION");
    let input = args.input;
    let symbolic = args.symbolic;
    let output = args.output;

    // print headers of the program
    utils::header_info(app_name, version, &input, symbolic);

    // read the contents of the input file
    let input_content = utils::read_file(&input);

    // create a new parser
    let mut parser = parser::Parser::new(&input_content, symbolic);

    // run the parser against the content
    parser.parse();

    // get the fields as binary instructions
    let fields = parser.get_fields();
    let binary_instructions = process_fields(fields);

    // print the binary instructions
    for binary_instruction in binary_instructions.iter() {
        log_command(&binary_instruction.binary);
    }

    // save binary instructions as a file
    utils::save_file(&output, &binary_instructions_to_bytes(&binary_instructions));
}
