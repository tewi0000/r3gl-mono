use std::path::PathBuf;

use intbits::Bits;

use crate::beatmap::{beatmap::Beatmap, taiko::hitobject::HitObject};

pub struct OsuTaikoParser {}

// This is horrible, please help
impl OsuTaikoParser {
    pub fn parse(data: &str) -> Beatmap<HitObject> {
        let mut beatmap = Beatmap::default();
        
        let mut objects_section = false;
        for line in data.lines() {
            if objects_section {
                let mut parts = line.split(",");
                parts.next();
                parts.next();
                if let Some(time) = parts.next() {
                    let time = time.parse().unwrap();
                    parts.next();
                    if let Some(hit_sound) = parts.next() {
                        let hit_sound: u32 = hit_sound.parse().unwrap();
                        beatmap.objects.push(HitObject {
                            time,
                            kat: hit_sound.bit(1) || hit_sound.bit(3),
                            big: hit_sound.bit(2),
                        });
                    }
                }
            } else if line.starts_with("[HitObjects]") {
                objects_section = true;
            } else {
                let mut parts = line.split(":");
                if let Some(key) = parts.next() {
                    if let Some(value) = parts.next() {
                        let key = key.trim();
                        let value = value.trim();
    
                        match key {
                            "Title" => beatmap.title = value.to_owned(),
                            "Artist" => beatmap.artist = value.to_owned(),
                            "AudioFilename" => beatmap.audio = PathBuf::from(value.to_owned()),
    
                            _ => {}
                        }
                    }
                }
            }
        }

        return beatmap;
    }
}

