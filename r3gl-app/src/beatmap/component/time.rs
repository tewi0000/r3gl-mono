pub trait TimeComponent {
    fn get_time(&self) -> u32;
    fn set_time(&mut self, value: u32);
}