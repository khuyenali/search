use std::{env, io, process};

use search::indexer::{Config, Indexer};
use search::webserver;

const DIR_PATH: &str = "docs/";
// const DIR_PATH: &str = "example";
const FILE_PATH: &str = "inverted_index.json";

fn main() -> io::Result<()> {
    let mut args = env::args();
    args.next();

    match args.next() {
        Some(arg) => match arg.as_str() {
            "serve" => serve_web(),
            "index" => cmd(args),

            _ => eprintln!("Missing arguments"),
        },
        None => (),
    };

    Ok(())
}

fn serve_web() {
    webserver::start().unwrap()
}

fn cmd(args: impl Iterator<Item = String>) {
    let config = parse_config(args).unwrap_or_else(|err| {
        eprintln!("Fail parse config: {}", err);
        process::exit(1);
    });
    let indexer = Indexer::build(config).unwrap();

    indexer.run().unwrap();
}

fn parse_config(mut args: impl Iterator<Item = String>) -> Result<Config<'static>, &'static str> {
    match args.next() {
        Some(arg) => {
            if arg == "build" {
                Ok(Config::Build(DIR_PATH))
            } else if arg == "load" {
                Ok(Config::Load(FILE_PATH))
            } else {
                Err("Missing arguments (build or load)")
            }
        }
        None => Err("Missing arguments (build or load)"),
    }
}
