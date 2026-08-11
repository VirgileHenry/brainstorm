use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;
use crate::ability_tree::object::specified_object::Specifier;
use crate::ability_tree::object::specified_object::SpecifierOrOfAndList;

/// A list of object specifiers, grouped with a logical OR.
///
/// It means that for an object to match these specifiers,
/// it must match any one specifier in the list.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecifierOrList<T: Specifier + Node> {
    pub specifiers: crate::HeapArrayVec<T, MAX_CHILDREN_PER_NODE>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

#[cfg(feature = "spanned_tree")]
impl<T> SpecifierOrList<T>
where
    T: Specifier + Node + Clone,
    T: boseiju_span::Spanned,
{
    pub fn add_factor_specifier(&self, factor_specifier: T) -> SpecifierOrOfAndList<T> {
        let mut or_specifiers = crate::HeapArrayVec::new();
        for prev_specifier in self.specifiers.iter() {
            let mut and_specifiers = arrayvec::ArrayVec::new_const();
            and_specifiers.push(prev_specifier.clone());
            and_specifiers.push(factor_specifier.clone());
            or_specifiers.push(and_specifiers);
        }
        SpecifierOrOfAndList {
            specifiers: or_specifiers,
            span: self.span.merge(&factor_specifier.span()),
        }
    }
}
#[cfg(not(feature = "spanned_tree"))]
impl<T> SpecifierOrList<T>
where
    T: Specifier + Node + Clone,
{
    pub fn add_factor_specifier(&self, factor_specifier: T) -> SpecifierOrOfAndList<T> {
        let mut or_specifiers = crate::HeapArrayVec::new();
        for prev_specifier in self.specifiers.iter() {
            let mut and_specifiers = arrayvec::ArrayVec::new_const();
            and_specifiers.push(prev_specifier.clone());
            and_specifiers.push(factor_specifier.clone());
            or_specifiers.push(and_specifiers);
        }
        SpecifierOrOfAndList {
            specifiers: or_specifiers,
        }
    }
}

impl<T: Specifier + Node> Node for SpecifierOrList<T> {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::SpecifierOrList
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for specifier in self.specifiers.iter() {
            children.push(specifier as &dyn Node);
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "specifier or list:")?;
        for (i, specifier) in self.specifiers.iter().enumerate() {
            if i == self.specifiers.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            specifier.display(out)?;
            out.pop_branch();
        }
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "specifiers or list"
    }
}

#[cfg(feature = "spanned_tree")]
impl<T: Specifier + Node> boseiju_span::Spanned for SpecifierOrList<T> {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl<T: Specifier + Node> Default for SpecifierOrList<T> {
    fn default() -> Self {
        Self {
            specifiers: crate::HeapArrayVec::new(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
