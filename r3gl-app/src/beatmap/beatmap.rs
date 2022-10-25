use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Beatmap {
    pub artist: String,
    pub title: String,
    pub creator: String,

    pub audio: PathBuf,
}

impl Default for Beatmap {
    fn default() -> Self {
        return Self {
            artist  : Default::default(),
            title   : Default::default(),
            creator : Default::default(),
            audio   : Default::default(),
        };
    }
}