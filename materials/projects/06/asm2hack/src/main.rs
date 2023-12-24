use clap::Parser;

/// interface to assemble Hack assembly language programs into binary code
/// for execution in the Hack hardware platform
#[derive(Parser, Debug)]
#[command(author = "txxnano", version, about)]
pub struct Args {
    #[arg(short, long, default_value_t = false)]
    /// use symbolic links instead of absolute reference memory addresses (default: false)
    symbolic: bool,

    #[arg(short, long)]
    /// input file to use (.asm)
    input: String,
}


// implement modules
mod parser;
mod utils;

fn main() {
    let args = Args::parse();
    
    // extract parameters from command line
    let app_name = "asm2hack assembler";
    let version = env!("CARGO_PKG_VERSION");
    let input = args.input;
    let symbolic = args.symbolic;

    utils::header_info(app_name, version, &input, symbolic);
}

