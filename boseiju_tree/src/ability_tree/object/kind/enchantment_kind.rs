use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// An object reference is a way to refer to one or more objects in the game.
///
/// Objects can be anything like cards, tokens, emblems, spells on the stack, etc.
///
/// Whenever an ability will refer to objects, they will almost always use object references.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnchantmentKind {
    Enchantment {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

impl crate::Node for EnchantmentKind {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::EnchantmentKind
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {


        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Enchantment { .. } => {
                let node_id = crate::NodeKind::EnchantmentBasicKind;
                let child = crate::dummy_terminal::TreeNodeDummyTerminal::new(node_id);
                children.push(child as &dyn Node)
            }
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "enchantment kind:")?;
        out.push_final_branch()?;
        match self {
            Self::Enchantment { .. } => write!(out, "enchantment")?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "enchantment kind"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for EnchantmentKind {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Enchantment { span } => *span,
        }
    }
}

impl Default for EnchantmentKind {
    fn default() -> Self {
        Self::Enchantment {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
