use std::{path::Path, time::Duration, fs};

use instant::Instant;
use r3gl_audio::{Audio, Player};

use crate::{project::projects::Projects, beatmap::{beatmap::Beatmap, taiko::hitobject::HitObject, parser::osu_taiko::OsuTaikoParser}};

pub struct Editor {
    pub beatmap: Option<Beatmap<HitObject>>,
    pub last_pause: Instant,
    pub last_time: u128,
}

impl Editor {
    pub fn new() -> Self {
        return Self {
            beatmap: None,
            last_pause: Instant::now(),
            last_time: 0,
        };
    }

    // Project Management
    pub fn open_project(&mut self, path: impl AsRef<Path>, projects: &mut Projects, player: &Player) {
        projects.open(&path);
        
        let data = fs::read_to_string(&path).unwrap();
        let beatmap = OsuTaikoParser::parse(&data);
        
        // TODO: parse audio name from beatmap
        let mp3 = path.as_ref().parent().unwrap().join(&beatmap.audio);
        player.load(&Audio::from_file(mp3).unwrap()).unwrap();

        let time = player.get_time();
        self.last_time = time.as_millis();
        self.last_pause = Instant::now();

        self.beatmap = Some(beatmap);
    }

    pub fn close_project(&mut self, projects: &mut Projects, player: &mut Player) {
        projects.current = None;

        player.stop();
        player.set_time(Duration::ZERO);
        
        let time = player.get_time();
        self.last_time = time.as_millis();
        self.last_pause = Instant::now();
    }

    // Time
    pub fn pause(&mut self, player: &mut Player) {
        if let Ok(length) = player.length() {
            let time = player.get_time();
            if time >= length {
                player.set_time(Duration::ZERO);
            }

            self.last_pause = Instant::now();
            self.last_time = time.as_millis();
            player.pause();
        } else {
            player.set_paused(true);
        }
    }

}