mod creature_specifier;

pub use creature_specifier::*;

use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::object::kind::CreatureKind;
use crate::ability_tree::object::specified_object::Specifiers;

/// A specified creature.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecifiedCreature {
    pub kind: CreatureKind,
    pub specifiers: Option<Specifiers<CreatureSpecifier>>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl SpecifiedCreature {
    pub fn add_factor_specifier(&self, factor_specifier: CreatureSpecifier) -> Self {
        #[cfg(feature = "spanned_tree")]
        let factor_specifier_span = {
            use boseiju_span::Spanned;
            factor_specifier.span()
        };
        match &self.specifiers {
            Some(prev_specifiers) => SpecifiedCreature {
                kind: self.kind.clone(),
                specifiers: Some(prev_specifiers.add_factor_specifier(factor_specifier)),
                #[cfg(feature = "spanned_tree")]
                span: factor_specifier_span.merge(&self.span),
            },
            None => SpecifiedCreature {
                kind: self.kind.clone(),
                specifiers: Some(Specifiers::Single(factor_specifier)),
                #[cfg(feature = "spanned_tree")]
                span: factor_specifier_span.merge(&self.span),
            },
        }
    }
}

impl Node for SpecifiedCreature {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::SpecifiedCreature.id()
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
        write!(out, "specified creature:")?;
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
        "specified creature"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for SpecifiedCreature {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for SpecifiedCreature {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            specifiers: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
