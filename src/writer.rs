use failure::Error;
use id3::{Tag, Version};

use model::Metadata;

pub fn write_metadata(metadata: &Metadata) -> Result<(), Error> {
    let path = &metadata.audio_file;
    let mut tag = Tag::read_from_path(&path)?;

    tag.set_artist(metadata.artist.clone());
    tag.set_title(metadata.title.clone());

    tag.write_to_path(path, Version::Id3v22)?;
    Ok(())
}
