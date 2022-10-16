use wcore::screen::Identifier as IdentifierTrait;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Identifier {
    #[default]
    None,
    Editor
}

impl IdentifierTrait for Identifier {}
