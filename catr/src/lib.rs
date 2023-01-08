use std::error::Error;
use clap::{App, Arg};
type MyResult<T> = Result<T, Box<dyn Error>>;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("cat-clone")
    .version("0.1.0")
    .author("Aryan Kumar")
    .about("Rust cat")
    .arg(
        Arg::with_name("files")
        .value_name("FILE")
        .help("Input file(s)")
        .multiple(true)
        .default_value("-"),
    )
    .arg(
        Arg::with_name("number")
            .short("n")
            .long("number")
            .help("Number lines")
            .takes_value(false)
            .conflicts_with("number_nonblank"),
    )
    .arg(
        Arg::with_name("number_nonblank")
            .short("b")
            .long("number-nonblank")
            .help("Number non-blank lines")
            .takes_value(false)
    )
    .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                for line_result in file.lines() {
                    let line = line_result?;
                    println!("{}", line);
                }
            }
        }
    }
    Ok(())
}