use supercub::test_parse;
use std::fs;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();

    let fname = &args[1];

    //TODO: hopefully we won't need this in the future
    env_logger::init();

    let src: String = fs::read_to_string(fname)?;
    test_parse(&src);
    Ok(())
}
