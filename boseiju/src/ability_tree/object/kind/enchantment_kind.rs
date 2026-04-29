use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::OneAmong;
use crate::ability_tree::object::specified_object::EnchantmentSpecifier;
use crate::ability_tree::object::specified_object::SpecifiedEnchantment;

/// An object reference is a way to refer to one or more objects in the game.
///
/// Objects can be anything like cards, tokens, emblems, spells on the stack, etc.
///
/// Whenever an ability will refer to objects, they will almost always use object references.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnchantmentKind {
    OneAmong(OneAmong<Self>),
    Specified(SpecifiedEnchantment),
}

impl EnchantmentKind {
    pub fn add_factor_specifier(&self, factor_specifier: EnchantmentSpecifier) -> Self {
        match self {
            Self::OneAmong(one_among) => {
                let mut references = crate::utils::HeapArrayVec::new();
                for prev in one_among.references.iter() {
                    references.push(prev.add_factor_specifier(factor_specifier.clone()));
                }
                Self::OneAmong(OneAmong {
                    references,
                    #[cfg(feature = "spanned_tree")]
                    span: one_among.node_span().merge(&factor_specifier.node_span()),
                })
            }
            Self::Specified(specified) => Self::Specified(specified.add_factor_specifier(factor_specifier)),
        }
    }
}

impl crate::ability_tree::AbilityTreeNode for EnchantmentKind {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::EnchantmentKind.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::OneAmong(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Specified(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "enchantment reference:")?;
        out.push_final_branch()?;
        match self {
            Self::OneAmong(child) => child.display(out)?,
            Self::Specified(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "enchantment reference"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::OneAmong(child) => child.node_span(),
            Self::Specified(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for EnchantmentKind {
    fn dummy_init() -> Self {
        Self::Specified(crate::utils::dummy())
    }
}
