use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::specified_object::PlaneswalkerSpecifier;
use crate::ability_tree::object::specified_object::SpecifiedPlaneswalker;

/// An object reference is a way to refer to one or more objects in the game.
///
/// Objects can be anything like cards, tokens, emblems, spells on the stack, etc.
///
/// Whenever an ability will refer to objects, they will almost always use object references.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaneswalkerKind {
    Specified(SpecifiedPlaneswalker),
}

impl PlaneswalkerKind {
    pub fn add_factor_specifier(&self, factor_specifier: PlaneswalkerSpecifier) -> Self {
        match self {
            Self::Specified(specified) => Self::Specified(specified.add_factor_specifier(factor_specifier)),
        }
    }
}

impl crate::ability_tree::AbilityTreeNode for PlaneswalkerKind {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PlaneswalkerKind.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Specified(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "planeswalker reference:")?;
        out.push_final_branch()?;
        match self {
            Self::Specified(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "planeswalker reference"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Specified(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PlaneswalkerKind {
    fn dummy_init() -> Self {
        Self::Specified(crate::utils::dummy())
    }
}
