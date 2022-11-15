use color_eyre::eyre::Result;
use dynamic_arena::DynamicArena;
use fxhash::FxHashMap;
use r3gl_app::{state::State, screen::{egui::EGuiScreen, taiko::TaikoScreen}, identifier::Identifier};
use wcore::{app::{App, AppState}, graphics::context::Context, bindings::{KeyCombination, Action}};
use winit::event::{VirtualKeyCode, ModifiersState};
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
    app.run(|graphics: &mut Context| {
        State::new(graphics)
    }, |app: &mut App<State, Identifier>, state: &mut AppState<State, Identifier>| {
        (|| -> Result<()> {
            let graphics = &state.graphics;
            app.screens.push(arena.alloc(TaikoScreen::new(graphics)?));
            app.screens.push(arena.alloc(EGuiScreen::new(graphics)?));
            state.bindings.insert(Identifier::Editor, {
                let mut binds = FxHashMap::default();
                binds.insert(KeyCombination::from((VirtualKeyCode::Space, ModifiersState::empty())), Action::new(
                    str!("Play/Pause"),
                    str!("Starts or stops the current song"),
                    |state: &mut State| {
                        state.editor.toggle_paused();
                    }
                ));

                binds
            });
            
            return Ok(());
        })().unwrap();
    });

	return Ok(());
}
