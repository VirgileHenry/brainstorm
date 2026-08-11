use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeholdKeywordAction {
    pub amount: crate::ability_tree::number::Number,
    pub creature_subtype: boseiju_lexer::terminal::CreatureSubtype,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for BeholdKeywordAction {
    fn node_id(&self) -> crate::NodeKind {
        use crate::node_kind::KeywordActionNodeKind;
        crate::NodeKind::KeywordAction(KeywordActionNodeKind::Behold)
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.amount as &dyn Node);
        children.push(&self.creature_subtype as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "behold ")?;
        self.amount.display(out)?;
        self.creature_subtype.display(out)?;
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "behold keyword ability"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for BeholdKeywordAction {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for BeholdKeywordAction {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl Default for BeholdKeywordAction {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            creature_subtype: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

pub fn ability(
    _amount: &crate::ability_tree::number::Number,
    _creature_subtype: &boseiju_lexer::terminal::CreatureSubtype,
    #[cfg(feature = "spanned_tree")] span: boseiju_span::Span,
) -> crate::ability_tree::ability::spell::SpellAbility {
    /* Fixme: unimplemented */
    crate::ability_tree::ability::spell::SpellAbility {
        effects: crate::HeapArrayVec::new(),
        #[cfg(feature = "spanned_tree")]
        span,
    }
}
