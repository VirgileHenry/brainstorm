use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct NormalLayout {
    pub face: crate::ability_tree::card_layout::CardFace,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl super::LayoutImpl for NormalLayout {
    fn card_types(&self) -> crate::ability_tree::type_line::SimplifiedCardTypes {
        (&self.face.card_type).into()
    }

    fn mana_value(&self) -> usize {
        self.face.mana_cost.as_ref().map(|cost| cost.mana_value()).unwrap_or(0)
    }

    #[cfg(feature = "parser")]
    fn from_raw_card(raw_card: &mtg_cardbase::Card) -> Result<Self, String> {
        use crate::ability_tree::card_layout::CardFace;

        Ok(NormalLayout {
            face: CardFace::from_raw_card(raw_card)?,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        })
    }

    fn layout_debug_display<W: std::io::Write>(&self, _output: &mut W) -> std::io::Result<()> {
        unimplemented!()
    }
}

impl AbilityTreeNode for NormalLayout {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::LayoutNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::Layout(LayoutNodeKind::Normal).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::<&dyn AbilityTreeNode, _>::new();

        /* ==== Face ==== */
        children.push(&self.face);

        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "normal layout")?;
        out.push_final_branch()?;
        self.face.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "normal layout"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}
