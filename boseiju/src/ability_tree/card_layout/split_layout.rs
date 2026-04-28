use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::type_line::SimplifiedCardTypes;

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SplitLayout {
    pub first_split: crate::ability_tree::card_layout::CardFace,
    pub second_split: crate::ability_tree::card_layout::CardFace,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl super::LayoutImpl for SplitLayout {
    fn card_types(&self) -> crate::ability_tree::type_line::SimplifiedCardTypes {
        let first_simplified_type: SimplifiedCardTypes = (&self.first_split.card_type).into();
        let second_simplified_type: SimplifiedCardTypes = (&self.second_split.card_type).into();
        first_simplified_type + second_simplified_type
    }

    fn mana_value(&self) -> usize {
        let first_cmc = self.first_split.mana_cost.as_ref().map(|cost| cost.mana_value()).unwrap_or(0);
        let second_cmc = self
            .second_split
            .mana_cost
            .as_ref()
            .map(|cost| cost.mana_value())
            .unwrap_or(0);
        first_cmc + second_cmc
    }

    #[cfg(feature = "parser")]
    fn from_raw_card(raw_card: &mtg_cardbase::Card) -> Result<Self, String> {
        use crate::ability_tree::card_layout::CardFace;

        let faces = match raw_card.card_faces.as_ref() {
            Some(faces) => faces,
            None => return Err(format!("Missing card faces! (ಠ_ಠ)")),
        };

        let (first_face, second_face) = match faces.as_slice() {
            [first, second] => (first, second),
            _other => return Err(format!("Split card should have exactly two faces! o(>< )o")),
        };

        Ok(SplitLayout {
            first_split: CardFace::from_card_face(first_face)?,
            second_split: CardFace::from_card_face(second_face)?,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        })
    }

    fn layout_debug_display<W: std::io::Write>(&self, _output: &mut W) -> std::io::Result<()> {
        unimplemented!()
    }
}

impl AbilityTreeNode for SplitLayout {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::LayoutNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::Layout(LayoutNodeKind::Split).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::<&dyn AbilityTreeNode, _>::new();

        /* ===== First split ===== */
        children.push(&self.first_split);

        /* ===== Second split ===== */
        children.push(&self.second_split);

        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "normal layout")?;
        out.push_inter_branch()?;
        self.first_split.display(out)?;
        out.next_final_branch()?;
        self.second_split.display(out)?;
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
