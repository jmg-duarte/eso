use std::fs;
use std::io;
use std::io::{Result, Read};
use std::iter::Iterator;

pub fn read_input(filename: &str) -> Result<String> {
    let res = fs::read_to_string(filename)?;
    Ok(res)
}
