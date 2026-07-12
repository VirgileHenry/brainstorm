mod enchantment_specifier;

pub use enchantment_specifier::*;

use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::kind::EnchantmentKind;
use crate::ability_tree::object::specified_object::Specifiers;

/// A specified Enchantment.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecifiedEnchantment {
    pub kind: EnchantmentKind,
    pub specifiers: Option<Specifiers<EnchantmentSpecifier>>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl SpecifiedEnchantment {
    pub fn add_factor_specifier(&self, factor_specifier: EnchantmentSpecifier) -> Self {
        #[cfg(feature = "spanned_tree")]
        let factor_specifier_span = {
            use boseiju_span::Spanned;
            factor_specifier.span()
        };
        match &self.specifiers {
            Some(prev_specifiers) => SpecifiedEnchantment {
                kind: self.kind.clone(),
                specifiers: Some(prev_specifiers.add_factor_specifier(factor_specifier)),
                #[cfg(feature = "spanned_tree")]
                span: factor_specifier_span.merge(&self.span),
            },
            None => SpecifiedEnchantment {
                kind: self.kind.clone(),
                specifiers: Some(Specifiers::Single(factor_specifier)),
                #[cfg(feature = "spanned_tree")]
                span: factor_specifier_span.merge(&self.span),
            },
        }
    }
}

impl Node for SpecifiedEnchantment {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::SpecifiedEnchantment.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        use crate::dummy_terminal::TreeNodeDummyTerminal;

        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.kind as &dyn Node);
        match self.specifiers.as_ref() {
            Some(specifiers) => children.push(specifiers as &dyn Node),
            None => children.push(TreeNodeDummyTerminal::none_node() as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "specified enchantment:")?;
        out.push_inter_branch()?;
        write!(out, "kind:")?;
        out.push_final_branch()?;
        self.kind.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "specifier(s):")?;
        out.push_final_branch()?;
        match self.specifiers.as_ref() {
            Some(specifiers) => specifiers.display(out)?,
            None => write!(out, "none")?,
        }
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "specified enchantment"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for SpecifiedEnchantment {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for SpecifiedEnchantment {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            specifiers: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
