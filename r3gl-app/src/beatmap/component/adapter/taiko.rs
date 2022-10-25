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
pub trait TaikoCircle: VariantComponent {
    fn is_big(&self) -> bool {
        return self.get_variant().bit(1);
    }
    fn set_big(&mut self, value: bool) {
        let mut variant = self.get_variant();
        variant.set_bit(1, value);
        self.set_variant(variant);
    }
    fn toggle_big(&mut self) {
        let mut variant = self.get_variant();
        variant.set_bit(1, !variant.bit(1));
        self.set_variant(variant);
    }

    fn is_kat(&self) -> bool {
        return self.get_variant().bit(1);
    }
    fn is_don(&self) -> bool {
        return !self.get_variant().bit(1);
    }
    fn get_color(&self) -> TaikoColor {
        return TaikoColor(self.get_variant().bit(1));
    }
    fn set_color(&mut self, value: TaikoColor) {
        let mut variant = self.get_variant();
        variant.set_bit(0, value == TaikoColor::KAT);
        self.set_variant(variant);
    }
    fn switch_color(&mut self) {
        let mut variant = self.get_variant();
        variant.set_bit(1, !variant.bit(1));
        self.set_variant(variant);
    }
}

impl<T: VariantComponent> TaikoCircle for T {}