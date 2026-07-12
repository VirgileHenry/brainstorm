use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;
use crate::ability_tree::imperative::Imperative;

/// An imperative list is a list of imperative that should be executed.
///
/// The inner item is actually a conditional imperative, since there are list that contains
/// imperative and conditional ones. For example, Chart a Course states:
/// "Draw two cards. Then discard a card unless you attacked this turn."
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImperativeList {
    pub imperatives: crate::HeapArrayVec<Imperative, MAX_CHILDREN_PER_NODE>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for ImperativeList {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::ImperativeList.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for imperative in self.imperatives.iter() {
            children.push(imperative as &dyn Node);
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "imperative list:")?;
        for (i, imperative) in self.imperatives.iter().enumerate() {
            if i == self.imperatives.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            imperative.display(out)?;
            out.pop_branch();
        }
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "imperative list"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ImperativeList {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for ImperativeList {
    fn default() -> Self {
        Self {
            imperatives: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
