use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A modification to the power / toughness of a creature.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreaturePowerToughnessModifier {
    pub modifier: crate::ability_tree::power_toughness::PowerToughnessModifiers,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for CreaturePowerToughnessModifier {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::CreaturePowerToughnessModifier
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.modifier as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "object power/toughness modifier:")?;
        out.push_final_branch()?;
        self.modifier.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "object power/toughness modifier"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CreaturePowerToughnessModifier {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for CreaturePowerToughnessModifier {
    fn default() -> Self {
        Self {
            modifier: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
