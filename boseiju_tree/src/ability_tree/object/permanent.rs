use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::AttachedObject;
use crate::ability_tree::object::OneAmong;
use crate::ability_tree::object::PreviouslyMentionned;
use crate::ability_tree::object::SelfReferencing;
use crate::ability_tree::object::reference::PermanentReference;

/// An object reference is a way to refer to one or more objects in the game.
///
/// Objects can be anything like cards, tokens, emblems, spells on the stack, etc.
///
/// Whenever an ability will refer to objects, they will almost always use object references.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Permanent {
    Attached(AttachedObject),
    OneAmong(OneAmong<Self>),
    PreviouslyMentionned(PreviouslyMentionned),
    SelfReferencing(SelfReferencing),
    Reference(PermanentReference),
}

impl Permanent {
    pub fn to_card(&self) -> crate::ability_tree::object::Card {
        use crate::ability_tree::object::Card;
        use crate::ability_tree::object::kind::CardKind;
        use crate::ability_tree::object::reference::CardReference;
        use crate::ability_tree::object::specified_object::SpecifiedCard;
        use boseiju_span::Spanned;

        match self {
            Self::Attached(attached) => Card::Attached(attached.clone()),
            Self::PreviouslyMentionned(attached) => Card::PreviouslyMentionned(attached.clone()),
            Self::SelfReferencing(attached) => Card::SelfReferencing(attached.clone()),
            Self::OneAmong(one_among) => Card::OneAmong(OneAmong {
                references: one_among.references.iter().map(|permanent| permanent.to_card()).collect(),
                #[cfg(feature = "spanned_tree")]
                span: one_among.span,
            }),
            Self::Reference(reference) => Card::Reference(CardReference {
                count: reference.count.clone(),
                card: SpecifiedCard {
                    kind: CardKind::Permanent(reference.permanent.clone()),
                    specifiers: None,
                    #[cfg(feature = "spanned_tree")]
                    span: reference.permanent.span(),
                },
                #[cfg(feature = "spanned_tree")]
                span: reference.span(),
            }),
        }
    }
}

impl crate::Node for Permanent {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::Permanent.id()
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
        write!(out, "permanent:")?;
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
        "permanent"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Permanent {
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

impl Default for Permanent {
    fn default() -> Self {
        Self::Reference(Default::default())
    }
}
