mod and_list;
mod common_specifier;
mod or_list;
mod or_of_ands;

pub use and_list::SpecifierAndList;
pub use common_specifier::*;
pub use or_list::SpecifierOrList;
pub use or_of_ands::SpecifierOrOfAndList;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// Marker trait to regroup the specifiers of the various specified objects.
pub trait Specifier {}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Specifiers<T: Specifier + Node> {
    Single(T),
    And(and_list::SpecifierAndList<T>),
    Or(SpecifierOrList<T>),
    OrOfAnd(SpecifierOrOfAndList<T>),
}

impl<T> Specifiers<T>
where
    T: Specifier + Node + Clone,
{
    pub fn merge_specifiers(&self, other: Self) -> Self {
        match (self, other) {
            (current, Self::Single(single)) => current.add_factor_specifier(single),
            (Self::Single(single), current) => current.add_factor_specifier(single.clone()),
            _ => unimplemented!("fuck me"),
        }
    }
}

#[cfg(feature = "spanned_tree")]
impl<T> Specifiers<T>
where
    T: Specifier + Node + Clone,
    T: boseiju_span::Spanned,
{
    pub fn add_factor_specifier(&self, factor_specifier: T) -> Self {
        use boseiju_span::Spanned;

        match self {
            Self::Single(specifier) => Self::And(SpecifierAndList {
                span: self.span().merge(&factor_specifier.span()),
                specifiers: {
                    let mut specifiers = crate::HeapArrayVec::new();
                    specifiers.push(specifier.clone());
                    specifiers.push(factor_specifier);
                    specifiers
                },
            }),
            Self::And(and) => Self::And(SpecifierAndList {
                span: and.span.merge(&factor_specifier.span()),
                specifiers: {
                    let mut and_specifiers = and.specifiers.clone();
                    and_specifiers.push(factor_specifier);
                    and_specifiers
                },
            }),
            Self::Or(or) => Self::OrOfAnd({
                let mut or_specifiers = crate::HeapArrayVec::new();
                for specifier in or.specifiers.iter() {
                    let mut and_specifiers = arrayvec::ArrayVec::new_const();
                    and_specifiers.push(specifier.clone());
                    and_specifiers.push(factor_specifier.clone());
                    or_specifiers.push(and_specifiers);
                }
                SpecifierOrOfAndList {
                    specifiers: or_specifiers,
                    span: or.span.merge(&factor_specifier.span()),
                }
            }),
            Self::OrOfAnd(or_of_and) => Self::OrOfAnd({
                let mut or_specifiers = or_of_and.specifiers.clone();
                for and_specifiers in or_specifiers.iter_mut() {
                    and_specifiers.push(factor_specifier.clone());
                }
                SpecifierOrOfAndList {
                    specifiers: or_specifiers,
                    span: or_of_and.span.merge(&factor_specifier.span()),
                }
            }),
        }
    }
}
#[cfg(not(feature = "spanned_tree"))]
impl<T> Specifiers<T>
where
    T: Specifier + Node + Clone,
{
    pub fn add_factor_specifier(&self, factor_specifier: T) -> Self {
        match self {
            Self::Single(specifier) => Self::And(SpecifierAndList {
                specifiers: {
                    let mut specifiers = crate::HeapArrayVec::new();
                    specifiers.push(specifier.clone());
                    specifiers.push(factor_specifier);
                    specifiers
                },
            }),
            Self::And(and) => Self::And(SpecifierAndList {
                specifiers: {
                    let mut and_specifiers = and.specifiers.clone();
                    and_specifiers.push(factor_specifier);
                    and_specifiers
                },
            }),
            Self::Or(or) => Self::OrOfAnd({
                let mut or_specifiers = crate::HeapArrayVec::new();
                for specifier in or.specifiers.iter() {
                    let mut and_specifiers = arrayvec::ArrayVec::new_const();
                    and_specifiers.push(specifier.clone());
                    and_specifiers.push(factor_specifier.clone());
                    or_specifiers.push(and_specifiers);
                }
                SpecifierOrOfAndList {
                    specifiers: or_specifiers,
                }
            }),
            Self::OrOfAnd(or_of_and) => Self::OrOfAnd({
                let mut or_specifiers = or_of_and.specifiers.clone();
                for and_specifiers in or_specifiers.iter_mut() {
                    and_specifiers.push(factor_specifier.clone());
                }
                SpecifierOrOfAndList {
                    specifiers: or_specifiers,
                }
            }),
        }
    }
}

impl<T: Specifier + Node> crate::Node for Specifiers<T> {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::ObjectSpecifiers.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Single(child) => children.push(child as &dyn Node),
            Self::And(child) => children.push(child as &dyn Node),
            Self::Or(child) => children.push(child as &dyn Node),
            Self::OrOfAnd(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl<T: Specifier + Node + boseiju_span::Spanned> boseiju_span::Spanned for Specifiers<T> {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Single(child) => child.span(),
            Self::And(child) => child.span(),
            Self::Or(child) => child.span(),
            Self::OrOfAnd(child) => child.span(),
        }
    }
}

impl<T: Specifier + Node + Default> Default for Specifiers<T> {
    fn default() -> Self {
        Self::Single(Default::default())
    }
}
