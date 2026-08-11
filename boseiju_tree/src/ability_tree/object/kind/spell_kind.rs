use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;
use crate::ability_tree::object::OneAmong;
use crate::ability_tree::object::specified_object::SpecifiedPermanent;

/// An object reference is a way to refer to one or more objects in the game.
///
/// Objects can be anything like cards, tokens, emblems, spells on the stack, etc.
///
/// Whenever an ability will refer to objects, they will almost always use object references.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpellKind {
    OneAmong(OneAmong<Self>),
    Permanent(SpecifiedPermanent),
    Spell {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

impl crate::Node for SpellKind {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::SpellKind
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {


        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::OneAmong(child) => children.push(child as &dyn Node),
            Self::Permanent(child) => children.push(child as &dyn Node),
            Self::Spell { .. } => {
                let node_id = crate::NodeKind::SpellBasicKind;
                let child = crate::dummy_terminal::TreeNodeDummyTerminal::new(node_id);
                children.push(child as &dyn Node)
            }
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "spell kind:")?;
        out.push_final_branch()?;
        match self {
            Self::OneAmong(child) => child.display(out)?,
            Self::Permanent(child) => child.display(out)?,
            Self::Spell { .. } => write!(out, "spell")?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "spell reference"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for SpellKind {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::OneAmong(child) => child.span(),
            Self::Permanent(child) => child.span(),
            Self::Spell { span } => *span,
        }
    }
}

impl Default for SpellKind {
    fn default() -> Self {
        Self::OneAmong(Default::default())
    }
}
