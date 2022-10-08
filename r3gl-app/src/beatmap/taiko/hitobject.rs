use crate::beatmap::Time;

#[derive(Default, Debug, Clone)]
pub struct HitObject {
    pub time : Time,
    pub kat  : bool,
    pub big  : bool,
}