use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::specified_object::Specifier;
use crate::ability_tree::object::specified_object::SpecifierOrOfAndList;

/// A list of object specifiers, grouped with a logical OR.
///
/// It means that for an object to match these specifiers,
/// it must match any one specifier in the list.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecifierOrList<T: Specifier + AbilityTreeNode> {
    pub specifiers: crate::utils::HeapArrayVec<T, MAX_CHILDREN_PER_NODE>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl<T: Specifier + AbilityTreeNode + Clone> SpecifierOrList<T> {
    pub fn add_factor_specifier(&self, factor_specifier: T) -> SpecifierOrOfAndList<T> {
        let mut or_specifiers = crate::utils::HeapArrayVec::new();
        for prev_specifier in self.specifiers.iter() {
            let mut and_specifiers = arrayvec::ArrayVec::new_const();
            and_specifiers.push(prev_specifier.clone());
            and_specifiers.push(factor_specifier.clone());
            or_specifiers.push(and_specifiers);
        }
        SpecifierOrOfAndList {
            specifiers: or_specifiers,
            #[cfg(feature = "spanned_tree")]
            span: self.span.merge(&factor_specifier.node_span()),
        }
    }
}

impl<T: Specifier + AbilityTreeNode> AbilityTreeNode for SpecifierOrList<T> {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::SpecifierOrList.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for specifier in self.specifiers.iter() {
            children.push(specifier as &dyn AbilityTreeNode);
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
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

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl<T: Specifier + AbilityTreeNode> crate::utils::DummyInit for SpecifierOrList<T> {
    fn dummy_init() -> Self {
        Self {
            specifiers: crate::utils::HeapArrayVec::new(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
