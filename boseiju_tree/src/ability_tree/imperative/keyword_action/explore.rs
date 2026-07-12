use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExploreKeywordAction {
    pub creature: crate::ability_tree::object::Creature,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for ExploreKeywordAction {
    fn node_id(&self) -> usize {
        use crate::node_kind::KeywordActionNodeKind;
        use idris::Idris;

        crate::NodeKind::KeywordAction(KeywordActionNodeKind::Explore).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.creature as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "explore ")?;
        self.creature.display(out)?;
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "explore keyword ability"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ExploreKeywordAction {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for ExploreKeywordAction {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "explore"
    }
}

impl Default for ExploreKeywordAction {
    fn default() -> Self {
        Self {
            creature: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

pub fn ability(
    _creature: &crate::ability_tree::object::Creature,
    #[cfg(feature = "spanned_tree")] span: boseiju_span::Span,
) -> crate::ability_tree::ability::spell::SpellAbility {
    /* Fixme: unimplemented */
    crate::ability_tree::ability::spell::SpellAbility {
        effects: crate::HeapArrayVec::new(),
        #[cfg(feature = "spanned_tree")]
        span,
    }
}
