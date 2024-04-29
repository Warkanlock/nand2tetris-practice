use clap::Parser as ClapParser;
use logs::log_command;

// module definitions
mod code;
mod logs;
mod parser;
mod utils;

/// specific implementation of Hack Virtual Machine
/// to translate machine code (.vm) to hack assembly language (.asm)
#[derive(ClapParser, Debug)]
#[command(author = "txxnano", version, about)]
pub struct Args {
    #[arg(short, long)]
    /// input file to use (.vm)
    input: String,

    /// output file to use (.asm)
    #[arg(short, long, default_value = "default")]
    output: String,
}

pub fn main() {
    let args = Args::parse();

    // extract parameters from command line
    let app_name = "vm2asm machine translator";
    let version = env!("CARGO_PKG_VERSION");
    let input = args.input;
    let output = args.output;

    // print headers of the program
    utils::header_info(app_name, version, &input);

    // read the contents of the input file
    let input_content = utils::read_file(&input);

    // initialize the parser
    let mut parser = parser::Parser::new(&input_content);

    // parse input
    parser.parse();

    // get the commands
    let commands = parser.get_fields();

    // generate assembly generator
    let mut generator = code::AssemblyGenerator::new();

    // generate assembly instructions from commands
    generator.process_commands(&commands);

    // get the assembly instructions
    for instruction in generator.instructions.iter() {
        log_command(&format!("{:?} from {:?}", instruction.instruction, instruction.command));
    }

    // save the output
    utils::save_file(&output, &generator.instructions_to_bytes());
}
