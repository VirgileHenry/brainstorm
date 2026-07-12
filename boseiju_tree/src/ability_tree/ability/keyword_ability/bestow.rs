use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BestowKeywordAbility {
    pub cost: crate::ability_tree::cost::Cost,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for BestowKeywordAbility {
    fn node_id(&self) -> usize {
        use crate::node_kind::KeywordAbilityNodeKind;
        use idris::Idris;

        crate::NodeKind::KeywordAbility(KeywordAbilityNodeKind::Bestow).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.cost as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "bestow ")?;
        self.cost.display(out)?;
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "bestow keyword ability"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for BestowKeywordAbility {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for BestowKeywordAbility {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "bestow"
    }
}

impl Default for BestowKeywordAbility {
    fn default() -> Self {
        Self {
            cost: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
