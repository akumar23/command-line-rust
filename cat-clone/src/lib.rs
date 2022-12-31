use std::error::Error;
use clap::{App, Arg};
type MyResult<T> = Result<T, Box<dyn Error>>;

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
        Arg::with_files
    )
    .get_matches();

    Ok(Config {
        files: ..=,
        number_lines: ..=,
        number_nonblank_lines: ..=,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}