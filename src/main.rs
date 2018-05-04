extern crate regex;
#[macro_use]
extern crate structopt;

use structopt::StructOpt;
use regex::Regex;
use std::io::{self, BufRead};

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// string to process
    #[structopt(name = "STRING")]
    string: String,
}

fn main() {
    let stdin = io::stdin();
    let re = Regex::new(&Opt::from_args().string).unwrap();

    for line_result in stdin.lock().lines() {
        let line = line_result.ok().unwrap();
        if !re.is_match(&line) {
            continue;
        }

        let mut findings: Vec<_> = re.find_iter(&line).collect();
        for (i, ch) in line.chars().enumerate() {
            for finding in &findings {
                if i == finding.start() {
                    print!("\x1B[33m");
                } else if i == finding.end() {
                    print!("\x1B[0m");
                }
            }
            print!("{}", ch);
        }
        println!("\x1B[0m");
    }
    // aaa
    // aaaaaaa
    // aa nn aa
    // aannaa
}
