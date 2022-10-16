use egui::epaint::ahash::HashMap;

use crate::screen::{Bindings, Identifier};

#[allow(type_alias_bounds)]
pub type BindingManager<S, I: Identifier> = HashMap<I, Bindings<S>>;