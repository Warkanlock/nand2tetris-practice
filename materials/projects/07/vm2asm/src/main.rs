use clap::Parser as ClapParser;
use code::AssemblyConfiguration;
use logs::log_command;
use std::path::Path;

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

    /// bootstrap
    #[arg(short, long)]
    bootstrap: bool,
}

pub fn main() {
    let args = Args::parse();

    // extract parameters from command line
    let app_name = "vm2asm machine translator";
    let version = env!("CARGO_PKG_VERSION");
    let input = args.input;
    let output = args.output;
    let bootstrap = args.bootstrap;

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

        // initialize the parser with class_name and content
        let mut parser = parser::Parser::new(&input_content, &input_file);

        // parse input
        parser.parse();

        // get the commands
        let commands = parser.get_fields();

        // generate assembly generator
        let mut generator = code::AssemblyGenerator::new(AssemblyConfiguration { bootstrap });

        // generate assembly instructions from commands
        generator.process_commands(&commands);

        // get the assembly instructions
        for instruction in generator.instructions.iter() {
            log_command(&format!(
                "{:?} >> {:?} from {:?}",
                parser.get_base_name(),
                instruction.instruction,
                instruction.command
            ));
        }

        // add isntructions into instructions vector
        instructions.extend(generator.instructions_to_bytes());
    }

    // save the output of all the instructions
    utils::save_file(&output, &instructions);
}
