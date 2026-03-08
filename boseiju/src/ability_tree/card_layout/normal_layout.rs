use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct NormalLayout {
    pub mana_cost: Option<crate::ability_tree::terminals::ManaCost>,
    pub card_type: crate::ability_tree::type_line::TypeLine,
    pub abilities: crate::AbilityTree,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl super::LayoutImpl for NormalLayout {
    fn card_types(&self) -> arrayvec::ArrayVec<mtg_data::CardType, 4> {
        self.card_type.card_types()
    }

    fn mana_value(&self) -> usize {
        self.mana_cost.as_ref().map(|cost| cost.mana_value()).unwrap_or(0)
    }

    #[cfg(feature = "parser")]
    fn from_raw_card(raw_card: &mtg_cardbase::Card) -> Result<Self, String> {
        use crate::lexer::IntoToken;

        let type_line_text = raw_card.type_line.to_ascii_lowercase();
        let type_line_span = crate::lexer::Span {
            start: 0,
            length: type_line_text.len(),
            text: type_line_text.as_str(),
        };

        Ok(NormalLayout {
            mana_cost: match raw_card.mana_cost.as_ref() {
                Some(mana_cost) => {
                    let mana_cost_span = crate::lexer::Span {
                        start: 0,
                        length: mana_cost.len(),
                        text: mana_cost,
                    };
                    Some(
                        crate::ability_tree::terminals::ManaCost::try_from_span(&mana_cost_span)
                            .ok_or_else(|| format!("Failed to parse mana cost from: {mana_cost}"))?,
                    )
                }
                None => None,
            },
            card_type: crate::ability_tree::type_line::TypeLine::try_from_span(&type_line_span)
                .ok_or_else(|| format!("Failed to parse card type: {}", raw_card.type_line))?,
            abilities: match raw_card.oracle_text.as_ref() {
                Some(oracle_text) => crate::AbilityTree::from_oracle_text(oracle_text, &raw_card.name)
                    .map_err(|e| format!("Failed to parse oracle text to ability tree: {e}"))?,
                None => crate::AbilityTree::empty(),
            },
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
        use crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal;

        let mut children = arrayvec::ArrayVec::<&dyn AbilityTreeNode, _>::new();

        /* ==== Mana cost ==== */
        match self.mana_cost.as_ref() {
            Some(child) => children.push(child),
            None => children.push(TreeNodeDummyTerminal::none_node()),
        }
        /* Alignement with other layouts that can have two mana costs */
        children.push(TreeNodeDummyTerminal::empty_node());

        /* ==== Card type ==== */
        children.push(&self.card_type);
        /* Alignement with other layouts that can have multiple card types */
        children.push(TreeNodeDummyTerminal::empty_node());

        /* ==== Ability tree ==== */
        children.push(&self.abilities);
        /* Alignement with other layouts that can have multiple card types */
        children.push(TreeNodeDummyTerminal::empty_node());

        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
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

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}
