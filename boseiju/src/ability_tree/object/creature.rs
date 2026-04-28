use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::AttachedObject;
use crate::ability_tree::object::OneAmong;
use crate::ability_tree::object::PreviouslyMentionned;
use crate::ability_tree::object::SelfReferencing;
use crate::ability_tree::object::reference::CreatureReference;

/// An object reference is a way to refer to one or more objects in the game.
///
/// Objects can be anything like cards, tokens, emblems, spells on the stack, etc.
///
/// Whenever an ability will refer to objects, they will almost always use object references.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Creature {
    Attached(AttachedObject),
    OneAmong(OneAmong<Self>),
    PreviouslyMentionned(PreviouslyMentionned),
    SelfReferencing(SelfReferencing),
    Reference(CreatureReference),
}

impl Creature {
    /// utility function to convert a creature to a permanent.
    pub fn to_permanent(&self) -> crate::ability_tree::object::Permanent {
        use crate::ability_tree::object::Permanent;
        match self {
            Self::Attached(attached) => Permanent::Attached(attached.clone()),
            Self::OneAmong(among) => Permanent::OneAmong(OneAmong {
                references: {
                    let mut references = crate::utils::HeapArrayVec::new();
                    for creature in among.references.iter() {
                        references.push(creature.to_permanent());
                    }
                    references
                },
                #[cfg(feature = "spanned_tree")]
                span: among.span,
            }),
            Self::PreviouslyMentionned(previously) => Permanent::PreviouslyMentionned(previously.clone()),
            Self::SelfReferencing(self_ref) => Permanent::SelfReferencing(self_ref.clone()),
            Self::Reference(creature_ref) => Permanent::Reference(crate::ability_tree::object::reference::PermanentReference {
                count: creature_ref.count.clone(),
                kind: crate::ability_tree::object::kind::PermanentKind::Creature(creature_ref.kind.clone()),
                #[cfg(feature = "spanned_tree")]
                span: creature_ref.span,
            }),
        }
    }
}

impl crate::ability_tree::AbilityTreeNode for Creature {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::Creature.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Attached(child) => children.push(child as &dyn AbilityTreeNode),
            Self::OneAmong(child) => children.push(child as &dyn AbilityTreeNode),
            Self::PreviouslyMentionned(child) => children.push(child as &dyn AbilityTreeNode),
            Self::SelfReferencing(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Reference(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "creature:")?;
        out.push_final_branch()?;
        match self {
            Self::Attached(child) => child.display(out)?,
            Self::OneAmong(child) => child.display(out)?,
            Self::PreviouslyMentionned(child) => child.display(out)?,
            Self::SelfReferencing(child) => child.display(out)?,
            Self::Reference(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "creature"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Attached(child) => child.node_span(),
            Self::OneAmong(child) => child.node_span(),
            Self::PreviouslyMentionned(child) => child.node_span(),
            Self::SelfReferencing(child) => child.node_span(),
            Self::Reference(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for Creature {
    fn dummy_init() -> Self {
        Self::Reference(crate::utils::dummy())
    }
}
