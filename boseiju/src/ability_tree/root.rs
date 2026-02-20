use crate::ability_tree::*;

/// One or more abilities.
///
/// This is the root of the Magic: the Gathering texts,
/// and can represent on its own the full text box of a card.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct AbilityTree {
    pub abilities: arrayvec::ArrayVec<ability::WrittenOrKeywordAbilty, MAX_CHILDREN_PER_NODE>,
}

impl AbilityTree {
    pub fn empty() -> AbilityTree {
        AbilityTree {
            abilities: arrayvec::ArrayVec::new_const(),
        }
    }

    pub fn display_from_root<W: std::io::Write>(&self, output: &mut W, prefix: &str) -> std::io::Result<()> {
        let mut tree_formatter = crate::utils::TreeFormatter::new(output, 64, prefix);
        self.display(&mut tree_formatter)
    }

    #[cfg(feature = "parser")]
    pub fn from_oracle_text(oracle_text: &str, card_name: &str) -> Result<AbilityTree, crate::error::BoseijuError> {
        let preprocessed = crate::lexer::preprocess(card_name, oracle_text);
        let tokens = crate::lexer::lex(&preprocessed)?;
        let result = crate::parser::parse(&tokens)?;
        Ok(result)
    }
}

impl crate::ability_tree::AbilityTreeNode for AbilityTree {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::AbilityTree.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for child in self.abilities.iter() {
            children.push(child as &dyn AbilityTreeNode);
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "ability tree:")?;
        for ability in self.abilities.iter().take(self.abilities.len().saturating_sub(1)) {
            out.push_inter_branch()?;
            ability.display(out)?;
            out.pop_branch();
        }
        if let Some(ability) = self.abilities.last() {
            out.push_final_branch()?;
            ability.display(out)?;
            out.pop_branch();
        }
        Ok(())
    }
}

impl Default for AbilityTree {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for AbilityTree {
    fn dummy_init() -> Self {
        Self {
            abilities: crate::utils::dummy(),
        }
    }
}
