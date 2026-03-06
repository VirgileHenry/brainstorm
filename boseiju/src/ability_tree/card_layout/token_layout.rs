use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Layout of a token
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TokenLayout {
    pub name: String,
    pub card_type: crate::ability_tree::type_line::TypeLine,
    pub color: crate::ability_tree::terminals::Color,
    pub abilities: crate::AbilityTree,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for TokenLayout {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::TokenLayout.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.card_type as &dyn AbilityTreeNode);
        children.push(&self.color as &dyn AbilityTreeNode);
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
        out.next_inter_branch()?;
        write!(out, "color:")?;
        out.push_final_branch()?;
        self.color.display(out)?;
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

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
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
        use crate::lexer::IntoToken;

        let type_line_span = crate::lexer::Span {
            start: 0,
            length: raw_card.type_line.len(),
            text: raw_card.type_line.as_str(),
        };

        Ok(TokenLayout {
            name: raw_card.name.clone(),
            card_type: crate::ability_tree::type_line::TypeLine::try_from_span(&type_line_span)
                .ok_or_else(|| format!("Failed to parse card type: {}", raw_card.type_line))?,
            color: unimplemented!(),
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

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for TokenLayout {
    fn dummy_init() -> Self {
        Self {
            name: String::with_capacity(0),
            card_type: crate::utils::dummy(),
            abilities: crate::utils::dummy(),
            color: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
