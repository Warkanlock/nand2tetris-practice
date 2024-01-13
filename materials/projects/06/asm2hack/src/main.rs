use clap::Parser as ClapParser;
use logs::log_info;

// module definitions
mod parser;
mod utils;
mod logs;

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
}

pub fn main() {
    let args = Args::parse();

    // extract parameters from command line
    let app_name = "asm2hack assembler";
    let version = env!("CARGO_PKG_VERSION");
    let input = args.input;
    let symbolic = args.symbolic;

    utils::header_info(app_name, version, &input, symbolic);

    let input_content = utils::read_file(&input);

    // create a new parser
    let mut parser = parser::Parser::new(&input_content, symbolic);

    // run the parser against the content
    parser.parse();

    // get the fields
    let fields = parser.get_fields();

    for field in fields.iter() {
        log_info(format!("{:?}", field).as_str());
    }
}

