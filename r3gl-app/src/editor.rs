use std::path::Path;

use instant::Duration;
use r3gl_audio::{Audio, Player};
use wcore::{clock::{SyncClock, Clock}};

use crate::{project::projects::Projects, beatmap::{beatmap::Beatmap, taiko::hitobject::HitObject, parser::osu_taiko::OsuTaikoParser}};

pub struct Editor {
    // TODO: replace this with game specific editor
    pub beatmap: Option<Beatmap<HitObject>>,
    
    // Audio/Time managment
    player: Player,
    clock: SyncClock,
}

impl Editor {
    pub fn new() -> Self {
        return Self {
            beatmap: None,

            player: Player::new().unwrap(),
            clock: SyncClock::new(),
        };
    }

    // Project Management
    pub fn open_project(&mut self, path: impl AsRef<Path>, projects: &mut Projects) {
        // Parse beatmap
        let beatmap = projects.open(&path);
        
        // Load audio
        let mp3 = path.as_ref().parent().unwrap().join(&beatmap.audio);
        self.player.load(&Audio::from_file(mp3).unwrap()).unwrap();
        
        // Update clock data
        self.clock.set_time(0);
        self.clock.set_length(self.player.length()
            .ok().unwrap_or(Duration::ZERO)
            .as_millis() as u32);

        // Set as current
        self.beatmap = Some(beatmap);
    }
    pub fn close_project(&mut self, projects: &mut Projects) {
        projects.current = None;
        self.beatmap = None;

        let time = self.player.get_time();
        self.clock.set_paused(true, time.as_millis() as u32);
        self.clock.set_length(0);
        self.player.stop();
        self.player.set_time(Duration::ZERO);
    }

    // Time
    pub fn toggle_paused(&mut self) {
          if self.player.length().is_err() {
            return;
        }

        let time = self.player.get_time();
        self.clock.toggle_paused(time.as_millis() as u32);
        self.player.pause();

        if time.as_millis() as u32 >= self.clock.get_length() {
            self.clock.set_time(0);
            self.player.set_time(Duration::ZERO);
        }
    }
    pub fn set_paused(&mut self, value: bool) {
        let time = self.player.get_time();
        self.clock.set_paused(value, time.as_millis() as u32);
        self.player.set_paused(value);
    }
    pub fn is_paused(&self) -> bool {
        return self.clock.is_paused();
    }

    pub fn set_time(&mut self, time: u32) {
        let time = Duration::from_millis(time as u64);
        self.clock.set_time(time.as_millis() as u32);
        self.player.set_time(time);
    }
    pub fn get_time(&mut self) -> u32 {
        return self.clock.get_time();
    }

    pub fn get_length(&self) -> u32 {
        return self.clock.get_length();
    }
}