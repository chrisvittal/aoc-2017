
use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;

#[inline]
fn to_buf_reader<P: AsRef<Path>>(p: P) -> io::BufReader<File> {
    let f = File::open(p).expect("could not open file");
    io::BufReader::new(f)
}

#[inline]
/// Simple function that collects a file into a `Lines` iterator, panics on
/// any error.
fn to_lines<P: AsRef<Path>>(p: P) -> io::Lines<io::BufReader<File>> {
    to_buf_reader(p).lines()
}

/// Gets the first line of the file minus any trailing newline or CRLF;
pub fn first_line<P: AsRef<Path>>(p: P) -> String {
    let mut s = String::new();
    to_buf_reader(p).read_line(&mut s).expect("could not read line");
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
    s
}

/// Simple function that collects a file into a `Vec<String>`, panics on
/// any error.
pub fn to_strings<P: AsRef<Path>>(p: P) -> Vec<String> {
    to_lines(p).map(|r| r.expect("error in read")).collect()
}

pub fn to_single_parsed<F, P>(p: P) -> Vec<F>
where
    P: AsRef<Path>,
    F: FromStr
{
    to_lines(p).map(|v| {
        let v = v.expect("error in read");
        match v.parse() {
            Err(_) => panic!("parse error for input: {}", v),
            Ok(vp) => vp
        }
    }).collect()
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
