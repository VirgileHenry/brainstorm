use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenownKeywordAbility {
    pub amount: crate::ability_tree::number::Number,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for RenownKeywordAbility {
    fn node_id(&self) -> usize {
        use crate::node_kind::KeywordAbilityNodeKind;
        use idris::Idris;

        crate::NodeKind::KeywordAbility(KeywordAbilityNodeKind::Renown).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.amount as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "renown ")?;
        self.amount.display(out)?;
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "renown keyword ability"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for RenownKeywordAbility {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for RenownKeywordAbility {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "renown"
    }
}

impl Default for RenownKeywordAbility {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
