use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevealKeywordAction {
    pub card: crate::ability_tree::object::Card,
    pub from: Option<crate::ability_tree::zone::ZoneReference>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for RevealKeywordAction {
    fn node_id(&self) -> usize {
        use crate::node_kind::KeywordActionNodeKind;
        use idris::Idris;

        crate::NodeKind::KeywordAction(KeywordActionNodeKind::Reveal).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.card as &dyn Node);
        match self.from.as_ref() {
            Some(child) => children.push(child as &dyn Node),
            None => {
                let none_node = crate::dummy_terminal::TreeNodeDummyTerminal::none_node();
                children.push(none_node as &dyn Node);
            }
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "reveal")?;
        out.push_inter_branch()?;
        write!(out, "card:")?;
        out.push_final_branch()?;
        self.card.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "from:")?;
        out.push_final_branch()?;
        match self.from.as_ref() {
            Some(child) => child.display(out)?,
            None => write!(out, "none")?,
        }
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "reveal keyword ability"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for RevealKeywordAction {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for RevealKeywordAction {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "reveal"
    }
}

impl Default for RevealKeywordAction {
    fn default() -> Self {
        Self {
            card: Default::default(),
            from: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

pub fn ability(
    _card: &crate::ability_tree::object::Card,
    _zone: Option<&crate::ability_tree::zone::ZoneReference>,
    #[cfg(feature = "spanned_tree")] span: boseiju_span::Span,
) -> crate::ability_tree::ability::spell::SpellAbility {
    /* Fixme: unimplemented */
    crate::ability_tree::ability::spell::SpellAbility {
        effects: crate::HeapArrayVec::new(),
        #[cfg(feature = "spanned_tree")]
        span,
    }
}
