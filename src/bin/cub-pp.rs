//! Reference preprocessor for Super Cub.
//! Takes a single file as input (or stdin), outputs resulting C code to stdout.

use std::fs;
use std::io;
use std::env;
use std::error::Error;

use supercub::realize;

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();


    //TODO: hopefully we won't need this in the future
    env_logger::init();

    let src: String;
    if args.len() >= 2 {
        let fname = &args[1];
        src = fs::read_to_string(fname)?;
    } else {
        src = io::read_to_string(io::stdin())?;
    }
    println!("{}", realize(&src, Vec::new()));
    Ok(())
}
