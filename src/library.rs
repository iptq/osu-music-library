use std::collections::HashMap;
use std::fs;
use std::panic;
use std::path::PathBuf;

use failure::Error;

use link;
use model::Metadata;
use parser::parse_osu_file;
use writer::write_metadata;

pub fn fixlibrary(libpath: PathBuf, outpath: PathBuf) -> Result<HashMap<PathBuf, Metadata>, Error> {
    let mut tmetamap = HashMap::new();
    for songdir in fs::read_dir(libpath)? {
        let mut metamap = HashMap::new();
        let songdir = songdir?.path();
        for file in fs::read_dir(&songdir)? {
            let file = file?.path();
            match file.extension() {
                Some(ext) => if ext != "osu" {
                    continue;
                },
                None => continue,
            }
            let metadata;
            match parse_osu_file(file) {
                Ok(metadata_) => metadata = metadata_,
                Err(_) => continue,
            }
            metamap.insert(metadata.audio_file.clone(), metadata);
        }
        for (_, metadata) in (&metamap).into_iter() {
            fn process(metadata: &Metadata, outpath: &PathBuf) -> Result<(), Error> {
                write_metadata(&metadata)?;
                println!("wrote to {:?}", metadata.audio_file);

                let mut linkpath = outpath.clone();
                linkpath.push(format!("{} - {}.mp3", metadata.artist, metadata.title));
                link::create_link(metadata.audio_file.clone(), linkpath)?;
                Ok(())
            }
            panic::catch_unwind(|| match process(metadata, &outpath) {
                Ok(_) | Err(_) => return,
            });
        }
        tmetamap.extend(metamap);
        println!("read directory '{:?}'", songdir);
    }
    // println!("{:?}", metamap);
    Ok(tmetamap)
}
