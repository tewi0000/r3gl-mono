use intbits::Bits;
use crate::beatmap::component::variant::VariantComponent;

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct TaikoColor(bool);

impl TaikoColor {
    pub const KAT: TaikoColor = TaikoColor(true);
    pub const DON: TaikoColor = TaikoColor(false);
}

// Variant bits usage:
// Bit 0 - Kat/Don
// Bit 1 - Small/Finisher
pub trait TaikoVariantAdapter {
    fn is_big(&self) -> bool;
    fn set_big(&mut self, value: bool);

    fn toggle_big(&mut self);

    fn is_kat(&self) -> bool;
    fn is_don(&self) -> bool;
    fn get_color(&self) -> TaikoColor;
    fn set_color(&mut self, value: TaikoColor);

    fn switch_color(&mut self);
}

impl TaikoVariantAdapter for VariantComponent {
    fn is_big(&self) -> bool { return self.0.bit(1); }
    fn set_big(&mut self, value: bool) { self.0 = self.0.with_bit(1, value); }
    fn toggle_big(&mut self) { self.0 = self.0.with_bit(1, !self.0.bit(1)); }

    fn is_kat(&self) -> bool { return self.0.bit(0); }
    fn is_don(&self) -> bool { return !self.0.bit(0); }
    fn get_color(&self) -> TaikoColor { return TaikoColor(self.0.bit(0)); }
    fn set_color(&mut self, value: TaikoColor) { self.0 = self.0.with_bit(0, value == TaikoColor::KAT); }
    fn switch_color(&mut self) { self.0 = self.0.with_bit(0, !self.0.bit(0)); }
}