use std::path::Path;

use instant::Duration;
use r3gl_audio::{Audio, AudioData};
use wcore::clock::{SyncClock, Clock};

use crate::{beatmap::{Time, beatmap::Beatmap, parser::osu_taiko::TaikoCircle}, project::project_manager::ProjectManager};

pub struct Editor {
    // TODO: replace this with game specific editor
    pub objects: Vec<TaikoCircle>,
    pub beatmap: Option<Beatmap>,
    
    // Audio/Time managment
    audio: Audio,
    clock: SyncClock,
}

impl Editor {
    pub fn new() -> Self {
        return Self {
            objects: vec![],
            beatmap: None,

            audio: Audio::new().unwrap(),
            clock: SyncClock::new(),
        };
    }

    // Project Management
    pub fn open_project(&mut self, path: impl AsRef<Path>, projects: &mut ProjectManager) {
        // Parse beatmap
        let (beatmap, objetcs) = projects.open(&path);
        
        // Load audio
        let mp3 = path.as_ref().parent().unwrap().join(&beatmap.audio);
        self.audio.load(&AudioData::from_file(mp3).unwrap()).unwrap();
        
        // Update clock data
        self.clock.set_time(0);
        self.clock.set_length(self.audio.length()
            .ok().unwrap_or(Duration::ZERO)
            .as_millis() as u32);

        // Set as current
        self.objects = objetcs;
        self.beatmap = Some(beatmap);
    }
    pub fn close_project(&mut self, projects: &mut ProjectManager) {
        projects.current = None;
        self.beatmap = None;
        self.objects.clear();

        let time = self.audio.get_time();
        self.clock.set_paused(true, time.as_millis() as u32);
        self.clock.set_length(0);
        self.audio.stop();
        self.audio.set_time(Duration::ZERO);
    }

    // Time
    pub fn toggle_paused(&mut self) {
          if self.audio.length().is_err() {
            return;
        }

        let time = self.audio.get_time();
        self.clock.toggle_paused(time.as_millis() as u32);
        self.audio.pause();

        if time.as_millis() as u32 >= self.clock.get_length() {
            self.clock.set_time(0);
            self.audio.set_time(Duration::ZERO);
        }
    }
    pub fn set_paused(&mut self, value: bool) {
        let time = self.audio.get_time();
        self.clock.set_paused(value, time.as_millis() as u32);
        self.audio.set_paused(value);
    }
    pub fn is_paused(&self) -> bool {
        return self.clock.is_paused();
    }

    pub fn set_time(&mut self, time: u32) {
        let time = Duration::from_millis(time as u64);
        self.clock.set_time(time.as_millis() as u32);
        self.audio.set_time(time);
    }
    pub fn get_time(&mut self) -> Time {
        return Time::from_ms(self.clock.get_time());
    }

    pub fn get_length(&self) -> u32 {
        return self.clock.get_length();
    }
}