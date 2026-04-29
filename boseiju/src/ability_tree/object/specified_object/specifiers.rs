mod and_list;
mod common_specifier;
mod or_list;
mod or_of_ands;

pub use and_list::SpecifierAndList;
pub use common_specifier::*;
pub use or_list::SpecifierOrList;
pub use or_of_ands::SpecifierOrOfAndList;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Marker trait to regroup the specifiers of the various specified objects.
pub trait Specifier {}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Specifiers<T: Specifier + AbilityTreeNode> {
    Single(T),
    And(and_list::SpecifierAndList<T>),
    Or(SpecifierOrList<T>),
    OrOfAnd(SpecifierOrOfAndList<T>),
}

impl<T: Specifier + AbilityTreeNode + Clone> Specifiers<T> {
    pub fn add_factor_specifier(&self, factor_specifier: T) -> Self {
        match self {
            Self::Single(specifier) => Self::And(SpecifierAndList {
                #[cfg(feature = "spanned_tree")]
                span: self.node_span().merge(&factor_specifier.node_span()),
                specifiers: {
                    let mut specifiers = arrayvec::ArrayVec::new_const();
                    specifiers.push(specifier.clone());
                    specifiers.push(factor_specifier);
                    specifiers
                },
            }),
            Self::And(and) => Self::And(SpecifierAndList {
                #[cfg(feature = "spanned_tree")]
                span: and.span.merge(&factor_specifier.node_span()),
                specifiers: {
                    let mut and_specifiers = and.specifiers.clone();
                    and_specifiers.push(factor_specifier);
                    and_specifiers
                },
            }),
            Self::Or(or) => Self::OrOfAnd({
                let mut or_specifiers = arrayvec::ArrayVec::new_const();
                for specifier in or.specifiers.iter() {
                    let mut and_specifiers = arrayvec::ArrayVec::new_const();
                    and_specifiers.push(specifier.clone());
                    and_specifiers.push(factor_specifier.clone());
                    or_specifiers.push(and_specifiers);
                }
                SpecifierOrOfAndList {
                    specifiers: or_specifiers,
                    #[cfg(feature = "spanned_tree")]
                    span: or.span.merge(&factor_specifier.node_span()),
                }
            }),
            Self::OrOfAnd(or_of_and) => Self::OrOfAnd({
                let mut or_specifiers = or_of_and.specifiers.clone();
                for and_specifiers in or_specifiers.iter_mut() {
                    and_specifiers.push(factor_specifier.clone());
                }
                SpecifierOrOfAndList {
                    specifiers: or_specifiers,
                    #[cfg(feature = "spanned_tree")]
                    span: or_of_and.span.merge(&factor_specifier.node_span()),
                }
            }),
        }
    }

    pub fn merge_specifiers(&self, other: Self) -> Self {
        match other {
            Self::Single(single) => self.add_factor_specifier(single),
            _ => unimplemented!("fuck me"),
        }
    }
}

impl<T: Specifier + AbilityTreeNode> crate::ability_tree::AbilityTreeNode for Specifiers<T> {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ObjectSpecifiers.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Single(child) => children.push(child as &dyn AbilityTreeNode),
            Self::And(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Or(child) => children.push(child as &dyn AbilityTreeNode),
            Self::OrOfAnd(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        match self {
            Self::Single(child) => child.display(out)?,
            Self::And(child) => child.display(out)?,
            Self::Or(child) => child.display(out)?,
            Self::OrOfAnd(child) => child.display(out)?,
        }
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "object specifiers"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Single(child) => child.node_span(),
            Self::And(child) => child.node_span(),
            Self::Or(child) => child.node_span(),
            Self::OrOfAnd(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl<T: Specifier + AbilityTreeNode + crate::utils::DummyInit> crate::utils::DummyInit for Specifiers<T> {
    fn dummy_init() -> Self {
        Self::Single(crate::utils::dummy())
    }
}
