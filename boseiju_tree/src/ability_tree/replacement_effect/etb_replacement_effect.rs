mod etb_modifier;

pub use etb_modifier::*;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

const MAX_ETB_MODIFIERS: usize = MAX_CHILDREN_PER_NODE - 1;

/// A replacement effect for the etb event.
///
/// From the CR:
/// \[614.1d\] Continuous effects that read "\[This permanent\] enters ..." \[...\] are replacement effects.
///
/// \[614.12\] Some replacement effects modify how a permanent enters the battlefield. \[...\] Such
/// effects may come from the permanent itself if they affect only that permanent\[.\]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EtbReplacementEffect {
    pub etb_event: crate::ability_tree::action::PermanentEtbAction,
    pub etb_modifiers: crate::HeapArrayVec<EtbModifier, MAX_ETB_MODIFIERS>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for EtbReplacementEffect {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::EtbReplacementEffect
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.etb_event as &dyn Node);
        for etb_modifier in self.etb_modifiers.iter() {
            children.push(etb_modifier as &dyn Node);
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "enters the battlefield replacement effect:")?;
        out.push_inter_branch()?;
        write!(out, "replaced event:")?;
        out.push_final_branch()?;
        self.etb_event.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "enters the battlefield modifiers:")?;
        for (i, etb_modifier) in self.etb_modifiers.iter().enumerate() {
            if i == self.etb_modifiers.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            etb_modifier.display(out)?;
            out.pop_branch();
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "enters the battlefield replacement effect"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EtbReplacementEffect {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for EtbReplacementEffect {
    fn default() -> Self {
        Self {
            etb_event: Default::default(),
            etb_modifiers: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
