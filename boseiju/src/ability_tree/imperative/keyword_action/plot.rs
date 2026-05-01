use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlotKeywordAction {
    pub cost: Box<crate::ability_tree::cost::Cost>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::ability_tree::AbilityTreeNode for PlotKeywordAction {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::KeywordActionNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::KeywordAction(KeywordActionNodeKind::Plot).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(self.cost.as_ref() as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "plot ")?;
        self.cost.display(out)?;
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "plot keyword ability"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

impl idris::Idris for PlotKeywordAction {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "plot"
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PlotKeywordAction {
    fn dummy_init() -> PlotKeywordAction {
        Self {
            cost: Box::new(crate::utils::dummy()),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
