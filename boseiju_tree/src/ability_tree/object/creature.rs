use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;
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
    pub fn to_permanent(&self) -> crate::ability_tree::object::Permanent {
        use crate::ability_tree::object::Permanent;
        use crate::ability_tree::object::kind::PermanentKind;
        use crate::ability_tree::object::reference::PermanentReference;
        use crate::ability_tree::object::specified_object::SpecifiedPermanent;
        #[cfg(feature = "spanned_tree")]
        use boseiju_span::Spanned;

        match self {
            Self::Attached(attached) => Permanent::Attached(attached.clone()),
            Self::PreviouslyMentionned(attached) => Permanent::PreviouslyMentionned(attached.clone()),
            Self::SelfReferencing(attached) => Permanent::SelfReferencing(attached.clone()),
            Self::OneAmong(one_among) => Permanent::OneAmong(OneAmong {
                references: one_among
                    .references
                    .iter()
                    .map(|permanent| permanent.to_permanent())
                    .collect(),
                #[cfg(feature = "spanned_tree")]
                span: one_among.span,
            }),
            Self::Reference(reference) => Permanent::Reference(PermanentReference {
                count: reference.count.clone(),
                permanent: SpecifiedPermanent {
                    kind: PermanentKind::Creature(reference.creature.clone()),
                    specifiers: None,
                    #[cfg(feature = "spanned_tree")]
                    span: reference.creature.span(),
                },
                #[cfg(feature = "spanned_tree")]
                span: reference.span(),
            }),
        }
    }
}

impl crate::Node for Creature {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::Creature.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Attached(child) => children.push(child as &dyn Node),
            Self::OneAmong(child) => children.push(child as &dyn Node),
            Self::PreviouslyMentionned(child) => children.push(child as &dyn Node),
            Self::SelfReferencing(child) => children.push(child as &dyn Node),
            Self::Reference(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Creature {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Attached(child) => child.span(),
            Self::OneAmong(child) => child.span(),
            Self::PreviouslyMentionned(child) => child.span(),
            Self::SelfReferencing(child) => child.span(),
            Self::Reference(child) => child.span(),
        }
    }
}

impl Default for Creature {
    fn default() -> Self {
        Self::Reference(Default::default())
    }
}
