use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuspendKeywordAbility {
    pub cost: crate::ability_tree::cost::Cost,
    pub amount: crate::ability_tree::number::Number,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for SuspendKeywordAbility {
    fn node_id(&self) -> usize {
        use crate::node_kind::KeywordAbilityNodeKind;
        use idris::Idris;

        crate::NodeKind::KeywordAbility(KeywordAbilityNodeKind::Suspend).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.amount as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "suspend ")?;
        self.amount.display(out)?;
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "suspend keyword ability"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for SuspendKeywordAbility {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for SuspendKeywordAbility {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "suspend"
    }
}

impl Default for SuspendKeywordAbility {
    fn default() -> Self {
        Self {
            cost: Default::default(),
            amount: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
