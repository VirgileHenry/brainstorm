use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;
use idris::Idris;

/// Classic layout for cards, "normal" layout
#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct NormalLayout {
    pub mana_cost: Option<crate::ability_tree::mana_cost::ManaCost>,
    pub card_type: crate::ability_tree::type_line::TypeLine,
    pub abilities: crate::AbilityTree,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::card::layout::LayoutImpl for NormalLayout {
    fn card_types(&self) -> crate::ability_tree::type_line::SimplifiedCardTypes {
        (&self.card_type).into()
    }

    fn mana_value(&self) -> usize {
        self.mana_cost.as_ref().map(|cost| cost.mana_value()).unwrap_or(0)
    }
}

impl Node for NormalLayout {
    fn node_id(&self) -> usize {
        use crate::node_kind::LayoutNodeKind;
        crate::NodeKind::Layout(LayoutNodeKind::Normal).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        use crate::dummy_terminal::TreeNodeDummyTerminal;
        let mut children = arrayvec::ArrayVec::<&dyn Node, _>::new();

        /*
            Always the same children order for layouts:
            mana cost 1
            mana cost 2
            card type 1
            card type 2
            ability tree 1
            ability tree 2
        */

        match self.mana_cost.as_ref() {
            Some(child) => children.push(child as &dyn Node),
            None => children.push(TreeNodeDummyTerminal::none_node() as &dyn Node),
        }
        children.push(TreeNodeDummyTerminal::empty_node() as &dyn Node);

        children.push(&self.card_type as &dyn Node);
        children.push(TreeNodeDummyTerminal::empty_node() as &dyn Node);

        children.push(&self.abilities as &dyn Node);
        children.push(TreeNodeDummyTerminal::empty_node() as &dyn Node);

        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "normal layout")?;
        out.push_inter_branch()?;
        match self.mana_cost.as_ref() {
            Some(mana_cost) => mana_cost.display(out)?,
            None => write!(out, "no mana cost")?,
        }
        out.next_inter_branch()?;
        self.card_type.display(out)?;
        out.next_final_branch()?;
        self.abilities.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "normal layout"
    }
}

impl Default for NormalLayout {
    fn default() -> Self {
        Self {
            mana_cost: Default::default(),
            card_type: Default::default(),
            abilities: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for NormalLayout {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}
