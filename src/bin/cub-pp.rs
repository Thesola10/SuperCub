/// Reference preprocessor for Super Cub.
/// Takes a single file as input (or stdin), outputs resulting C code to stdout.

use std::fs;
use std::env;
use std::error::Error;

use supercub::realize;

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();

    let fname = &args[1];

    //TODO: hopefully we won't need this in the future
    env_logger::init();

    let src: String = fs::read_to_string(fname)?;
    println!("{}", realize(&src, Vec::new()));
    Ok(())
}
