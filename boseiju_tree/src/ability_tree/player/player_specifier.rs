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
pub enum PlayerSpecifier<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    Ally(AllySpecifier),
    Opponent(OpponentSpecifier),
    Other(OtherPlayerSpecifier<Q>),
}

impl<Q> Specifier for PlayerSpecifier<Q> where Q: crate::ability_tree::quantifier::Quantifier {}

impl<Q> Node for PlayerSpecifier<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::PlayerSpecifier
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
impl<Q> boseiju_span::Spanned for PlayerSpecifier<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Ally(child) => child.span(),
            Self::Opponent(child) => child.span(),
            Self::Other(child) => child.span(),
        }
    }
}

impl<Q> idris::Idris for PlayerSpecifier<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl<Q> Default for PlayerSpecifier<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn default() -> Self {
        Self::Other(Default::default())
    }
}
