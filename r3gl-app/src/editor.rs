use std::{path::Path, time::Duration, fs};

use r3gl_audio::{Audio, Player};
use wcore::timer::Timer;

use crate::{project::projects::Projects, beatmap::{beatmap::Beatmap, taiko::hitobject::HitObject, parser::osu_taiko::OsuTaikoParser}};

pub struct Editor {
    pub beatmap: Option<Beatmap<HitObject>>,
    
    // Audio/Time managment
    player: Player,
    paused: bool,
    length: u32,
    time: Timer,
}

impl Editor {
    pub fn new() -> Self {
        return Self {
            beatmap: None,

            player: Player::new().unwrap(),
            paused: true,
            length: 0,
            time: Timer::new(),
        };
    }

    // Project Management
    pub fn open_project(&mut self, path: impl AsRef<Path>, projects: &mut Projects) {
        projects.open(&path);
        
        // Parse beatmap
        let data = fs::read_to_string(&path).unwrap();
        let beatmap = OsuTaikoParser::parse(&data);
        
        // Load audio
        let mp3 = path.as_ref().parent().unwrap().join(&beatmap.audio);
        self.player.load(&Audio::from_file(mp3).unwrap()).unwrap();
        
        // Cache length
        self.length = self.player.length()
            .ok().unwrap_or(Duration::ZERO)
            .as_millis() as u32;

        // Set as current
        self.beatmap = Some(beatmap);
    }

    pub fn close_project(&mut self, projects: &mut Projects) {
        projects.current = None;

        self.player.stop();
        self.paused = true;
        self.player.set_time(Duration::ZERO);
    }

    // Time
    pub fn pause(&mut self) {
        if let Ok(length) = self.player.length() {
            let time = self.player.get_time();
            if time >= length {
                self.player.set_time(Duration::ZERO);
                self.time.reset(Duration::ZERO);
            }

            self.player.pause();
            self.paused = !self.paused;
            if !self.paused {
                self.time.reset(time);
            }

        } else {
            self.player.set_paused(true);
            self.paused = true;
        }
    }

    pub fn set_paused(&mut self, value: bool) {
        self.player.set_paused(value);
        if !value {
            self.time.reset(self.time_duration());
        }
    }

    pub fn set_time(&mut self, time: u32) {
        let time = Duration::from_millis(time as u64);
        self.player.set_time(time);
        self.time.reset(time);
    }

    pub fn time(&self) -> u32 {
        return self.time.sync(self.player.is_paused(), self.player.get_time()).as_millis() as u32;
    }

    fn time_duration(&self) -> Duration {
        return self.time.sync(self.player.is_paused(), self.player.get_time());
    }

    pub fn length(&self) -> u32 {
        return self.length;
    }

    pub fn is_paused(&self) -> bool {
        return self.paused;
    }
}