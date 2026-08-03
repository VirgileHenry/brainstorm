use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// Fixme: doc
///
/// Fixme: they should have real children, the ai can make a diff
/// between the count and target nodes currently
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CountSpecifier {
    A {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    All {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Count(crate::ability_tree::number::Number),  /* Fixme */
    Target(crate::ability_tree::number::Number), /* Fixme */
    TheNext {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

impl Node for CountSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::CountSpecifierIdMarker.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Target(child) => children.push(child as &dyn Node),
            _ => children.push(crate::dummy_terminal::TreeNodeDummyTerminal::new(
                crate::NodeKind::CountSpecifier(self.clone()).id(),
            ) as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "count specifier:")?;
        out.push_final_branch()?;
        match self {
            Self::A { .. } => write!(out, "a")?,
            Self::All { .. } => write!(out, "all")?,
            Self::Count(num) => {
                write!(out, "count")?;
                out.push_final_branch()?;
                num.display(out)?;
                out.pop_branch();
            }
            Self::Target(num) => {
                write!(out, "target")?;
                out.push_final_branch()?;
                num.display(out)?;
                out.pop_branch();
            }
            Self::TheNext { .. } => write!(out, "the next")?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "object count"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CountSpecifier {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::A { span } => *span,
            Self::All { span } => *span,
            Self::Count(child) => child.span(),
            Self::Target(child) => child.span(),
            Self::TheNext { span } => *span,
        }
    }
}

impl Default for CountSpecifier {
    fn default() -> Self {
        Self::All {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
