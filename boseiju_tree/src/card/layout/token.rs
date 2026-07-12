use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;
use idris::Idris;

/// Layout of a token
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TokenLayout {
    pub name: String,
    pub card_type: crate::ability_tree::type_line::TypeLine,
    pub color_identity: crate::ability_tree::colors::Colors,
    pub abilities: crate::AbilityTree,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::card::layout::LayoutImpl for TokenLayout {
    fn card_types(&self) -> crate::ability_tree::type_line::SimplifiedCardTypes {
        (&self.card_type).into()
    }

    fn mana_value(&self) -> usize {
        0
    }
}

impl Node for TokenLayout {
    fn node_id(&self) -> usize {
        use crate::node_kind::LayoutNodeKind;
        crate::NodeKind::Layout(LayoutNodeKind::Token).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        use crate::dummy_terminal::TreeNodeDummyTerminal;
        let mut children = arrayvec::ArrayVec::new_const();

        /*
            Always the same children order for layouts:
            mana cost 1
            mana cost 2
            card type 1
            card type 2
            ability tree 1
            ability tree 2
            color identity
        */

        children.push(TreeNodeDummyTerminal::empty_node() as &dyn Node);
        children.push(TreeNodeDummyTerminal::empty_node() as &dyn Node);

        children.push(&self.card_type as &dyn Node);
        children.push(TreeNodeDummyTerminal::empty_node() as &dyn Node);

        children.push(&self.abilities as &dyn Node);
        children.push(TreeNodeDummyTerminal::empty_node() as &dyn Node);

        children.push(&self.color_identity as &dyn Node);

        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "token:")?;
        out.push_inter_branch()?;
        write!(out, "types:")?;
        out.push_final_branch()?;
        self.card_type.display(out)?;
        out.pop_branch();
        out.next_inter_branch()?;
        write!(out, "color:")?;
        out.push_final_branch()?;
        self.color_identity.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "abilities:")?;
        out.push_final_branch()?;
        self.abilities.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "token description"
    }
}

impl Default for TokenLayout {
    fn default() -> Self {
        Self {
            name: "Default Token Layout".to_string(),
            card_type: Default::default(),
            color_identity: Default::default(),
            abilities: Default::default(),
            span: Default::default(),
        }
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for TokenLayout {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}
