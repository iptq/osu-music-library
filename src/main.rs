extern crate id3;
#[macro_use]
extern crate failure;
extern crate regex;

mod library;
mod link;
mod model;
mod parser;
mod writer;

use std::fs;
use std::path::PathBuf;
use std::process;

use failure::Error;

#[derive(Debug, Fail)]
pub enum OsuError {
    #[fail(display = "parse failure")]
    ParseError,
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn run() -> Result<(), Error> {
    let libpath = PathBuf::from("/home/michael/osu/Songs");
    let outpath = PathBuf::from("/home/michael/Music/osu");
    if outpath.exists() {
        fs::remove_dir_all(&outpath)?;
    }
    fs::create_dir_all(&outpath)?;
    library::fixlibrary(libpath, outpath)?;
    Ok(())
}
