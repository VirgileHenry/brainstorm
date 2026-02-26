use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum CountSpecifier {
    A,
    All,
    Target(crate::ability_tree::number::Number),
    AllOthers,
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
            Self::A => write!(out, "a")?,
            Self::All => write!(out, "all")?,
            Self::Target(num) => {
                write!(out, "target")?;
                out.push_final_branch()?;
                num.display(out)?;
                out.pop_branch();
            }
            Self::AllOthers => write!(out, "all others")?,
        }
        out.pop_branch();
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CountSpecifier {
    fn dummy_init() -> Self {
        Self::All
    }
}
