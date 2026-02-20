use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Layout of a token
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct TokenLayout {
    pub name: String,
    pub card_type: crate::ability_tree::type_line::TypeLine,
    pub abilities: crate::AbilityTree,
}

impl AbilityTreeNode for TokenLayout {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::TokenLayout.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.card_type as &dyn AbilityTreeNode);
        children.push(&self.abilities as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "token:")?;
        out.push_inter_branch()?;
        write!(out, "types:")?;
        out.push_final_branch()?;
        self.card_type.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "abilities:")?;
        out.push_final_branch()?;
        self.abilities.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }
}

impl super::LayoutImpl for TokenLayout {
    fn card_types(&self) -> arrayvec::ArrayVec<mtg_data::CardType, 4> {
        self.card_type.card_types()
    }

    fn mana_value(&self) -> usize {
        0
    }

    #[cfg(feature = "parser")]
    fn from_raw_card(raw_card: &mtg_cardbase::Card) -> Result<Self, String> {
        Ok(TokenLayout {
            name: raw_card.name.clone(),
            card_type: crate::ability_tree::type_line::TypeLine::parse(&raw_card.type_line, raw_card)
                .map_err(|e| format!("Failed to parse card type: {e}"))?,
            abilities: match raw_card.oracle_text.as_ref() {
                Some(oracle_text) => crate::AbilityTree::from_oracle_text(oracle_text, &raw_card.name)
                    .map_err(|e| format!("Failed to parse oracle text to ability tree: {e}"))?,
                None => crate::AbilityTree::empty(),
            },
        })
    }

    fn layout_debug_display<W: std::io::Write>(&self, output: &mut W) -> std::io::Result<()> {
        writeln!(output, "│ ╰─ Token:")?;
        writeln!(output, "│    ├─ Type Line: {}", self.card_type)?;
        write!(output, "│    ╰─ Abilities: ")?;
        self.abilities.display_from_root(output, "│       ")?;
        writeln!(output, "")?;
        Ok(())
    }
}
