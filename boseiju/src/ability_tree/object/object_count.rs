use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CountSpecifier {
    A {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    All {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Target(crate::ability_tree::number::Number),
    AllOthers {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl CountSpecifier {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::A { span } => *span,
            Self::All { span } => *span,
            Self::Target(child) => child.node_span(),
            Self::AllOthers { span } => *span,
        }
    }
}

impl AbilityTreeNode for CountSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CountSpecifierIdMarker.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Target(child) => children.push(child as &dyn AbilityTreeNode),
            _ => children.push(crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(
                crate::ability_tree::NodeKind::CountSpecifier(self.clone()).id(),
            ) as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "count specifier:")?;
        out.push_final_branch()?;
        match self {
            Self::A { .. } => write!(out, "a")?,
            Self::All { .. } => write!(out, "all")?,
            Self::Target(num) => {
                write!(out, "target")?;
                out.push_final_branch()?;
                num.display(out)?;
                out.pop_branch();
            }
            Self::AllOthers { .. } => write!(out, "all others")?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "object count"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::A { span } => *span,
            Self::All { span } => *span,
            Self::Target(child) => child.node_span(),
            Self::AllOthers { span } => *span,
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CountSpecifier {
    fn dummy_init() -> Self {
        Self::All {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
