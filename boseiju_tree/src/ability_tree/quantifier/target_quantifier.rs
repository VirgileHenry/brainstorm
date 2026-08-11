use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A condition that is met when a given event has occured in a given timeframe.
///
/// Examples are, "if you attacked this turn" or "if a creature died this turn".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetQuantifier {
    pub number: crate::ability_tree::number::Number,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for TargetQuantifier {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::TargetQuantifier
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.number as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "target quantifier:")?;
        out.push_final_branch()?;
        self.number.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "target quantifier"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for TargetQuantifier {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for TargetQuantifier {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl Default for TargetQuantifier {
    fn default() -> Self {
        Self {
            number: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
