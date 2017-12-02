
use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;

/// Simple function that collects a file into a `Lines` iterator, panics on
/// any error.
pub fn to_lines<P: AsRef<Path>>(p: P) -> io::Lines<io::BufReader<File>> {
    let f = File::open(p).expect("could not open file");
    let r = io::BufReader::new(f);
    r.lines()
}

/// Simple function that collects a file into a `Vec<String>`, panics on
/// any error.
pub fn to_strings<P: AsRef<Path>>(p: P) -> Vec<String> {
    to_lines(p).map(|r| r.expect("error in read")).collect()
}

/// Converts input into parsed vector
pub fn to_split_parsed<F, P>(p: P) -> Vec<Vec<F>>
where
    P: AsRef<Path>,
    F: FromStr
{
    to_lines(p).map(|r| r.expect("error in read"))
        .map(|s| s.split_whitespace()
             .map(|v| match v.parse() {
                 Err(_) => panic!("parse error for input: {}", v),
                 Ok(vp) => vp
             })
             .collect())
        .collect()
}
