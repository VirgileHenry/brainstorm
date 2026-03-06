use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::lexer::IntoToken;

/// A cost requiring mana.
///
/// See also <https://mtg.fandom.com/wiki/Mana_cost>
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManaCost {
    pub cost: arrayvec::ArrayVec<crate::ability_tree::terminals::Mana, MAX_CHILDREN_PER_NODE>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl ManaCost {
    pub fn mana_value(&self) -> usize {
        self.cost.iter().map(|mana| mana.mana_value()).sum()
    }
}

impl AbilityTreeNode for ManaCost {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ManaCost.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for cost in self.cost.iter() {
            children.push(cost as &dyn AbilityTreeNode);
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "mana cost: ")?;
        for mana in self.cost.iter() {
            mana.display(out)?;
        }
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "mana cost"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

impl IntoToken for ManaCost {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        let mut cost = arrayvec::ArrayVec::new_const();
        /* Yeah, yeah, it's not that hard and may not need a regex. Whatever for now. */
        lazy_static::lazy_static!(
            static ref mana_cost_regex: regex::Regex = regex::Regex::new(r"(\{[^{}]+\})")
                .expect("Failed to compile the mana cost iterator regex: {e}");
        );

        for capture in mana_cost_regex.captures_iter(&span.text) {
            let span = crate::lexer::Span {
                start: span.start + capture.get_match().start(),
                length: capture.get_match().len(),
                text: capture.get_match().as_str(),
            };
            let mana = crate::ability_tree::terminals::Mana::try_from_span(&span)?;
            cost.push(mana);
        }

        Some(ManaCost {
            cost,
            #[cfg(feature = "spanned_tree")]
            span: span.into(),
        })
    }
}

impl std::ops::Deref for ManaCost {
    type Target = [crate::ability_tree::terminals::Mana];
    fn deref(&self) -> &Self::Target {
        self.cost.as_slice()
    }
}

impl std::ops::DerefMut for ManaCost {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.cost.as_mut_slice()
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ManaCost {
    fn dummy_init() -> Self {
        Self {
            cost: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
