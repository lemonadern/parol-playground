extern crate parol_runtime;

mod prl_playground_grammar;
// The output is version controlled
mod prl_playground_grammar_trait;
mod prl_playground_parser;

use crate::prl_playground_grammar::PrlPlaygroundGrammar;
use crate::prl_playground_parser::parse;
use anyhow::{anyhow, Context, Result};
use parol_runtime::log::debug;
use std::env;
use std::fs;
use std::time::Instant;

// To generate:
// parol -f ./prl_playground.par -e ./prl_playground-exp.par -p ./src/prl_playground_parser.rs -a ./src/prl_playground_grammar_trait.rs -t PrlPlaygroundGrammar -m prl_playground_grammar -g

fn main() -> Result<()> {
    env_logger::init();
    debug!("env logger started");

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let file_name = args[1].clone();
        let input = fs::read_to_string(file_name.clone())
            .with_context(|| format!("Can't read file {}", file_name))?;
        let mut prl_playground_grammar = PrlPlaygroundGrammar::new();
        let now = Instant::now();
        parse(&input, &file_name, &mut prl_playground_grammar)
            .with_context(|| format!("Failed parsing file {}", file_name))?;
        let elapsed_time = now.elapsed();
        println!("Parsing took {} milliseconds.", elapsed_time.as_millis());
        if args.len() > 2 && args[2] == "-q" {
            Ok(())
        } else {
            println!("Success!\n{}", prl_playground_grammar);
            Ok(())
        }
    } else {
        Err(anyhow!("Please provide a file name as first parameter!"))
    }
}
