use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

use crate::ability_tree::object::specified_object::Specifiers;
use crate::ability_tree::player::player_specifier::PlayerSpecifier;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecifiedPlayer {
    pub count: crate::ability_tree::quantifier::Quantifier,
    pub specifiers: Option<Specifiers<PlayerSpecifier>>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for SpecifiedPlayer {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::SpecifiedPlayer.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.count as &dyn Node);
        match self.specifiers.as_ref() {
            Some(specifiers) => children.push(specifiers as &dyn Node),
            None => children.push(crate::dummy_terminal::TreeNodeDummyTerminal::none_node() as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "specified player:")?;
        out.push_inter_branch()?;
        self.count.display(out)?;
        out.next_final_branch()?;
        write!(out, "specifier(s):")?;
        out.push_final_branch()?;
        match self.specifiers.as_ref() {
            Some(specifiers) => specifiers.display(out)?,
            None => write!(out, "none")?,
        }
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "specified player"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for SpecifiedPlayer {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for SpecifiedPlayer {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "SpecifiedPlayer"
    }
}

impl Default for SpecifiedPlayer {
    fn default() -> Self {
        Self {
            count: Default::default(),
            specifiers: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
