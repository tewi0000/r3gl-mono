use std::path::PathBuf;

use intbits::Bits;

use crate::beatmap::{beatmap::Beatmap, Time, component::{HitObject, time::TimeComponent, variant::VariantComponent}};

pub struct TaikoCircle {
    pub time: TimeComponent,
    pub variant: VariantComponent,
}

impl HitObject for TaikoCircle {
    #[inline(always)] fn time(&self) -> Option<&TimeComponent> { Some(&self.time) }
    #[inline(always)] fn variant(&self) -> Option<&VariantComponent> { Some(&self.variant) }

    #[inline(always)] fn time_mut(&mut self) -> Option<&mut TimeComponent> { Some(&mut self.time) }
    #[inline(always)] fn variant_mut(&mut self) -> Option<&mut VariantComponent> { Some(&mut self.variant) }
}

pub fn parse(data: &str) -> (Beatmap, Vec<Box<dyn HitObject>>) {
    let mut beatmap = Beatmap::default();
    let mut objects = Vec::<Box<dyn HitObject>>::new();

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
                    objects.push(Box::new(TaikoCircle {
                        time: TimeComponent(Time::from_ms(time)),
                        variant: VariantComponent(0
                            .with_bit(0, hit_sound.bit(1) || hit_sound.bit(3))
                            .with_bit(1, hit_sound.bit(2))
                        ),
                    }));
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

    return (beatmap, objects);
}