#![allow(unused)]
pub mod problems;

use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
pub struct Args {
    /// use example input instead of solution input
    #[arg(short, long, default_value_t = false)]
    pub example: bool,
}

pub fn run<Fun>(name: &str, solve: Fun)
where
    Fun: FnOnce(&str),
{
    let args = Args::parse();

    let input_type = if args.example { "example" } else { "solution" };
    let input_path = format!("src/problems/{}/{}.txt", name, input_type);

    let input = fs::read_to_string(&input_path)
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", input_path));

    solve(&input);
}
