use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A condition that is met when a given object matches given object specifiers.
///
/// For example, "if it is a zombie card".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionCreatureMatchSpecifier {
    pub creature: crate::ability_tree::object::Creature,
    pub specifier: crate::ability_tree::object::specified_object::CreatureSpecifier,
    pub shall_match: bool,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for ConditionCreatureMatchSpecifier {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::ConditionCreatureMatchSpecifier
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.creature as &dyn Node);
        children.push(&self.specifier as &dyn Node);
        children
    }

    fn data(&self) -> Option<crate::AbTreeNodeData> {
        Some(crate::AbTreeNodeData::Boolean { value: self.shall_match })
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "object matches specifiers:")?;
        out.push_inter_branch()?;
        self.creature.display(out)?;
        out.next_final_branch()?;
        self.specifier.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "object is of kind"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ConditionCreatureMatchSpecifier {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for ConditionCreatureMatchSpecifier {
    fn default() -> Self {
        Self {
            creature: Default::default(),
            specifier: Default::default(),
            shall_match: false,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
