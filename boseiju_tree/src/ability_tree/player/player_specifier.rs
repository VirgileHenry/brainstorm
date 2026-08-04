mod ally_specifier;
mod opponent_specifier;
mod other_player_specifier;

pub use ally_specifier::AllySpecifier;
pub use opponent_specifier::OpponentSpecifier;
pub use other_player_specifier::OtherPlayerSpecifier;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;
use crate::ability_tree::object::specified_object::Specifier;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerSpecifier {
    Ally(AllySpecifier),
    Opponent(OpponentSpecifier),
    Other(OtherPlayerSpecifier),
}

impl Specifier for PlayerSpecifier {}

impl Node for PlayerSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::PlayerSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Ally(child) => children.push(child as &dyn Node),
            Self::Opponent(child) => children.push(child as &dyn Node),
            Self::Other(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "player specifier:")?;
        out.push_final_branch()?;
        match self {
            Self::Ally(child) => child.display(out)?,
            Self::Opponent(child) => child.display(out)?,
            Self::Other(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "player specifier"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PlayerSpecifier {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Ally(child) => child.span(),
            Self::Opponent(child) => child.span(),
            Self::Other(child) => child.span(),
        }
    }
}

impl idris::Idris for PlayerSpecifier {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "PlayerSpecifier"
    }
}

impl Default for PlayerSpecifier {
    fn default() -> Self {
        Self::Other(Default::default())
    }
}
