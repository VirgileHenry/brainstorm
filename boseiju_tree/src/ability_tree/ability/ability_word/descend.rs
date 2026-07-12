use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DescendAbilityWord {
    pub amount: crate::ability_tree::number::Number,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for DescendAbilityWord {
    fn node_id(&self) -> usize {
        use crate::node_kind::AbilityWordNodeKind;
        use idris::Idris;

        crate::NodeKind::AbilityWord(AbilityWordNodeKind::Descend).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.amount as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "descend ")?;
        self.amount.display(out)?;
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "descend ability word"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for DescendAbilityWord {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for DescendAbilityWord {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "descend"
    }
}

impl Default for DescendAbilityWord {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
