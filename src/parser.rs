use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

use failure::Error;
use regex::Regex;

use model::Metadata;
use OsuError;

pub fn parse_osu_file(path: PathBuf) -> Result<Metadata, Error> {
    let rgx_section = Regex::new(r"^\[(.+)\]")?;
    let rgx_kvpair = Regex::new(r"^([^:]+)\s*:\s*([^:]+)\s*$")?;

    let f = File::open(&path)?;
    let f = BufReader::new(f);
    let parent = path.parent().unwrap();

    let mut _section = String::new();
    let mut artist_opt: Option<String> = None;
    let mut title_opt: Option<String> = None;
    let mut audio_opt: Option<PathBuf> = None;
    for line in f.lines() {
        let line = line?;
        match rgx_section.captures(&line) {
            Some(c) => {
                _section = String::from(&c[1]);
                continue;
            }
            None => (),
        }
        match rgx_kvpair.captures(&line) {
            Some(c) => match &c[1] {
                "ArtistUnicode" => artist_opt = Some(String::from(&c[2])),
                "TitleUnicode" => title_opt = Some(String::from(&c[2])),
                "AudioFilename" => audio_opt = Some(parent.join(String::from(&c[2])).to_path_buf()),
                _ => (),
            },
            None => (),
        }
    }
    let metadata = match (artist_opt, title_opt, audio_opt) {
        (Some(artist), Some(title), Some(audio_file)) => Some(Metadata {
            artist,
            title,
            audio_file,
        }),
        _ => bail!("failed to identify metadata"),
    };
    match metadata {
        Some(metadata) => return Ok(metadata),
        None => bail!(OsuError::ParseError),
    };
}
