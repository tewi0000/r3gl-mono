pub trait VariantComponent {
    fn get_variant(&self) -> u32;
    fn set_variant(&mut self, value: u32);
}