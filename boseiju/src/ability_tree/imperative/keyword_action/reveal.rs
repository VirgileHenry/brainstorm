use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevealKeywordAction {
    pub card: crate::ability_tree::object::Card,
    pub from: Option<crate::ability_tree::zone::ZoneReference>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::ability_tree::AbilityTreeNode for RevealKeywordAction {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::KeywordActionNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::KeywordAction(KeywordActionNodeKind::Reveal).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.card as &dyn AbilityTreeNode);
        match self.from.as_ref() {
            Some(child) => children.push(child as &dyn AbilityTreeNode),
            None => {
                let none_node = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::none_node();
                children.push(none_node as &dyn AbilityTreeNode);
            }
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
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

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
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

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for RevealKeywordAction {
    fn dummy_init() -> RevealKeywordAction {
        Self {
            card: crate::utils::dummy(),
            from: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

pub fn ability(
    _card: &crate::ability_tree::object::Card,
    _zone: Option<&crate::ability_tree::zone::ZoneReference>,
    #[cfg(feature = "spanned_tree")] span: crate::ability_tree::span::TreeSpan,
) -> crate::ability_tree::ability::spell::SpellAbility {
    /* Fixme: unimplemented */
    crate::ability_tree::ability::spell::SpellAbility {
        effects: crate::utils::HeapArrayVec::new(),
        #[cfg(feature = "spanned_tree")]
        span,
    }
}
