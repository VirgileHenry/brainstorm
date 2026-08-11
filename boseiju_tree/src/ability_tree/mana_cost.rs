use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A cost requiring mana.
///
/// See also <https://mtg.fandom.com/wiki/Mana_cost>
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManaCost {
    pub symbols: crate::HeapArrayVec<boseiju_lexer::terminal::ManaSymbol, MAX_CHILDREN_PER_NODE>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl ManaCost {
    pub fn mana_value(&self) -> usize {
        self.symbols.iter().map(|symbol| symbol.mana_value()).sum()
    }
}

impl Node for ManaCost {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::ManaCost
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for cost in self.symbols.iter() {
            children.push(cost as &dyn Node);
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "mana cost: ")?;
        for mana in self.symbols.iter() {
            mana.display(out)?;
        }
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "mana cost"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ManaCost {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl std::ops::Deref for ManaCost {
    type Target = [boseiju_lexer::terminal::ManaSymbol];
    fn deref(&self) -> &Self::Target {
        self.symbols.as_slice()
    }
}

impl std::ops::DerefMut for ManaCost {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.symbols.as_mut_slice()
    }
}

impl Default for ManaCost {
    fn default() -> Self {
        Self {
            symbols: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
