use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchKeywordAction {
    pub card: crate::ability_tree::object::Card,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for SearchKeywordAction {
    fn node_id(&self) -> usize {
        use crate::node_kind::KeywordActionNodeKind;
        use idris::Idris;

        crate::NodeKind::KeywordAction(KeywordActionNodeKind::Search).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.card as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "search your library for ")?;
        self.card.display(out)?;
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "search keyword ability"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for SearchKeywordAction {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for SearchKeywordAction {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "search"
    }
}

impl Default for SearchKeywordAction {
    fn default() -> Self {
        Self {
            card: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

pub fn ability(
    _card: &crate::ability_tree::object::Card,
    #[cfg(feature = "spanned_tree")] span: boseiju_span::Span,
) -> crate::ability_tree::ability::spell::SpellAbility {
    /* Fixme: unimplemented */
    crate::ability_tree::ability::spell::SpellAbility {
        effects: crate::HeapArrayVec::new(),
        #[cfg(feature = "spanned_tree")]
        span,
    }
}
