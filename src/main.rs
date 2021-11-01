use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;
use std::prelude::rust_2021::FromIterator;
use std::str::SplitWhitespace;
use anyhow::Context;
use regex::Regex;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt, Debug)]
struct Cli {

    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: PathBuf,

    /// Output file. Optional. If not provided, stdout will be used
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let regex = Regex::new(r"[^A-Za-z0-9\s]").unwrap();

    let args: Cli = Cli::from_args();

    let file: File = File::open(&args.path).with_context(|| format!("Could not read file `{}`", args.path.to_str().get_or_insert("") ))?;

    let reader: BufReader<Box<dyn Read>>  = BufReader::new(Box::new(file));

    let mut word_counter: HashMap<String, i32> = HashMap::new();

    for line in reader.lines() {
        let line_string = match line {
            Ok(line_string) => { line_string },
            Err(error) => { return Err(error.into()) }
        };

        let line_string = regex.replace_all(&line_string, " ");

        let mut iter:Vec<&str> = line_string.split_whitespace().collect();

        for word in iter {
            if let Some(val) = word_counter.get_mut(word) {
                *val = *val + 1;
            } else {
                word_counter.insert(word.parse::<String>().with_context(|| "Could not parse to from &str to String!")?, 1 );
            }
        }

    }

    pretty_output(word_counter.iter().collect(), args.output);

    Ok(())
}

/// If an output is provided, write the results of wcr to that file. Else write the
/// output to stdout instead
fn pretty_output(counter_structure: Vec<(&String, &i32)>, output: Option<PathBuf>) {
    let mut result: String = String::new();

    for (key, value) in counter_structure {
        result.push_str(&*format!("{} {} \n", key, value));
    }

    if let Some(path) = output {
        fs::write(path, result);
    } else {
        print!("{}", result);
    }

}