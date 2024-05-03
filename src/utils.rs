use std::{fs, io::Read};

use anyhow::Error;

pub fn get_reader(input: &str) -> Result<Box<dyn Read>, Error> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(fs::File::open(input)?)
    };

    Ok(reader)
}
