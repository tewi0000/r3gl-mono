use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Beatmap<HitObject: Default> {
    pub artist: String,
    pub title: String,
    pub creator: String,

    pub audio: PathBuf,
    
    pub objects: Vec<HitObject>,
}

impl<HitObject: Default> Default for Beatmap<HitObject> {
    fn default() -> Self {
        return Self {
            artist  : Default::default(),
            title   : Default::default(),
            creator : Default::default(),
            audio   : Default::default(),
            objects : Default::default(),
        };
    }
}