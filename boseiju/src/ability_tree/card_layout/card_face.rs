use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CardFace {
    pub mana_cost: Option<crate::ability_tree::terminals::ManaCost>,
    pub card_type: crate::ability_tree::type_line::TypeLine,
    pub abilities: crate::AbilityTree,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl CardFace {
    #[cfg(feature = "parser")]
    pub fn from_raw_card(raw_card: &mtg_cardbase::Card) -> Result<Self, String> {
        use crate::lexer::IntoToken;

        let type_line_text = raw_card.type_line.to_ascii_lowercase();
        let type_line_span = crate::lexer::Span {
            start: 0,
            length: type_line_text.len(),
            text: type_line_text.as_str(),
        };

        Ok(CardFace {
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

    #[cfg(feature = "parser")]
    pub fn from_card_face(card_face: &mtg_cardbase::CardFace) -> Result<CardFace, String> {
        use crate::lexer::IntoToken;

        let type_line = match card_face.type_line.as_ref() {
            Some(type_line) => type_line,
            None => return Err(format!("Missing type_line !")),
        };
        let type_line_text = type_line.to_ascii_lowercase();
        let type_line_span = crate::lexer::Span {
            start: 0,
            length: type_line_text.len(),
            text: type_line_text.as_str(),
        };

        Ok(CardFace {
            mana_cost: match card_face.mana_cost.as_ref() {
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
                .ok_or_else(|| format!("Failed to parse card type: {}", type_line))?,
            abilities: match card_face.oracle_text.as_ref() {
                Some(oracle_text) => crate::AbilityTree::from_oracle_text(oracle_text, &card_face.name)
                    .map_err(|e| format!("Failed to parse oracle text to ability tree: {e}"))?,
                None => crate::AbilityTree::empty(),
            },
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        })
    }
}

impl AbilityTreeNode for CardFace {
    fn node_id(&self) -> usize {
        use idris::Idris;

        crate::ability_tree::NodeKind::Face.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal;

        let mut children = arrayvec::ArrayVec::<&dyn AbilityTreeNode, _>::new();

        /* ==== Mana cost ==== */
        match self.mana_cost.as_ref() {
            Some(child) => children.push(child),
            None => children.push(TreeNodeDummyTerminal::none_node()),
        }

        /* ==== Card type ==== */
        children.push(&self.card_type);

        /* ==== Ability tree ==== */
        children.push(&self.abilities);

        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "card face")?;
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
        "card face"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}
