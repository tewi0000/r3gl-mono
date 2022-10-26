use super::{beatmap::Beatmap, component::HitObject};

pub mod osu_taiko;

pub fn parse<'a: 'b, 'b>(data: &str) -> (Beatmap, Vec<Box<dyn HitObject>>) {
    return osu_taiko::parse(data);
}