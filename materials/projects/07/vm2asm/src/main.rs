use clap::Parser as ClapParser;

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

    // TODO: implement the translation from the machine code
    // to assembly language
    println!("Machine code: {:?}", input_content);

    // save the output
    utils::save_file(&output, &Vec::new());
}
