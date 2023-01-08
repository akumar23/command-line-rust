use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;


#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
    .version("0.1.0")
    .author("Aryan Kumar")
    .about("Rust head")
    .arg(
        Arg::with_name("files")
            .value_name("FILE")
            .help("Input file(s)")
            .multiple(true)
            .default_value("-"),
    )
    .arg(
        Arg::with_name("lines")
            .short("n")
            .long("lines")
            .value_name("LINES")
            .help("Number lines")
            .default_value("10"),
    )
    .arg(
        Arg::with_name("bytes")
            .short("c")
            .long("bytes")
            .value_name("BYTES")
            .takes_value(true)
            .conflicts_with("lines")
            .help("Number of bytes"),
    )
    .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}



