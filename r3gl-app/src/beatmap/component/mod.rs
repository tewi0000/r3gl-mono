use self::{time::TimeComponent, variant::VariantComponent};

pub mod adapter;
pub mod variant;
pub mod time;

pub trait HitObject {
    fn time(&self) -> Option<&TimeComponent>;
    fn variant(&self) -> Option<&VariantComponent>;

    fn time_mut(&mut self) -> Option<&mut TimeComponent>;
    fn variant_mut(&mut self) -> Option<&mut VariantComponent>;
}