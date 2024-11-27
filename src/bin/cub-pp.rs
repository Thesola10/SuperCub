use supercub::test_parse;
use std::fs;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();

    let fname = &args[1];

    let src: String = fs::read_to_string(fname)?;
    test_parse(&src);
    Ok(())
}
