use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::type_line::SimplifiedCardTypes;

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FlipLayout {
    pub up_face: crate::ability_tree::card_layout::CardFace,
    pub down_face: crate::ability_tree::card_layout::CardFace,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl super::LayoutImpl for FlipLayout {
    fn card_types(&self) -> crate::ability_tree::type_line::SimplifiedCardTypes {
        let first_simplified_type: SimplifiedCardTypes = (&self.up_face.card_type).into();
        let second_simplified_type: SimplifiedCardTypes = (&self.down_face.card_type).into();
        first_simplified_type + second_simplified_type
    }

    fn mana_value(&self) -> usize {
        let first_cmc = self.up_face.mana_cost.as_ref().map(|cost| cost.mana_value()).unwrap_or(0);
        let second_cmc = self.down_face.mana_cost.as_ref().map(|cost| cost.mana_value()).unwrap_or(0);
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
            _other => return Err(format!("Flip card should have exactly two faces! o(>< )o")),
        };

        Ok(FlipLayout {
            up_face: CardFace::from_card_face(first_face)?,
            down_face: CardFace::from_card_face(second_face)?,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        })
    }

    fn layout_debug_display<W: std::io::Write>(&self, _output: &mut W) -> std::io::Result<()> {
        unimplemented!()
    }
}

impl AbilityTreeNode for FlipLayout {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::LayoutNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::Layout(LayoutNodeKind::Flip).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::<&dyn AbilityTreeNode, _>::new();

        /* ===== First split ===== */
        children.push(&self.up_face);

        /* ===== Second split ===== */
        children.push(&self.down_face);

        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "flip layout")?;
        out.push_inter_branch()?;
        self.up_face.display(out)?;
        out.next_final_branch()?;
        self.down_face.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "flip layout"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}
