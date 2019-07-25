use std::fs;
use std::io::Result;

pub fn read_input(filename: &str) -> Result<String> {
    let res = fs::read_to_string(filename)?;
    Ok(res)
}
