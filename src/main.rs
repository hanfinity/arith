use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
mod parse;

/*
Michael Han
CS 510 - Advanced Functional Programming
Project - Untyped Arithmetic Interpreter

This is a version of the interpreter implemented in OCAML for Types and Programming Languages (TAPL)
The interpreter parses and evaluates untyped arithmetic expressions as described in TAPL ch. 3
The original OCAML code can be found here:
https://www.cis.upenn.edu/~bcpierce/tapl/checkers/arith.tar.gz
 */

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]);
    match file {
        Ok(f) => {
            let br = BufReader::new(f);
            let lines = br.lines();
            let r = Regex::new(r"/\*.*\*/").unwrap();
            for l in lines {
                match l {
                    Ok(s) => {
                        let s0 = &s.replace(";", "");
                        let s1 = r.replace_all(s0, "");
                        if s1.len() != 0 {println!("{}", parse::eval(parse::parse(&s1.to_string())))}
                    }
                    _ => ()
                }
            }
        }
        _ => println!("{} not found", &args[1])
    }
}

#[test]
fn regex_test() {
    let r = Regex::new(r"/\*.*\*/").unwrap();
    let s = r.replace_all("/*test comment*/", "fixed");
    assert_eq!(s, "fixed")
}