use std::fmt::Display;

use wcore::screen::Identifier as IdentifierTrait;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Identifier {
    #[default]
    None,
    Editor
}

impl IdentifierTrait for Identifier {
    
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Identifier::None   => write!(f, ""),
            Identifier::Editor => write!(f, "Editor"),
        }
    }
}