use color_eyre::eyre::Result;
use dynamic_arena::DynamicArena;
use r3gl_app::{state::State, screen::{egui::EGuiScreen, taiko::TaikoScreen}};
use wcore::{app::App, graphics::context::Context};
use str_macro::str;

fn main() -> Result<()> {
	color_eyre::install()?;
    env_logger::init();

    let app = App {
        title: str!("r3gl"),
        width: 1200,
        height: 800,
        screens: Default::default(),
    };

    let arena = DynamicArena::new();
    app.run(State::new(), |app: &mut App<State>, graphics: &mut Context| {
        (|| -> Result<()> {
            app.screens.push(arena.alloc(TaikoScreen::new(graphics)?));
            app.screens.push(arena.alloc(EGuiScreen::new(graphics)?));
            
            return Ok(());
        })().unwrap();
    });

	return Ok(());
}
