use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// A power and toughness.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerToughness {
    pub power: crate::ability_tree::number::Number,
    pub toughness: crate::ability_tree::number::Number,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for PowerToughness {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::PowerToughness
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.power as &dyn Node);
        children.push(&self.toughness as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "power / toughness:")?;
        out.push_inter_branch()?;
        write!(out, "power:")?;
        out.push_final_branch()?;
        write!(out, "-")?;
        self.power.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "toughness:")?;
        out.push_final_branch()?;
        write!(out, "-")?;
        self.toughness.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "power and toughness"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PowerToughness {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for PowerToughness {
    fn default() -> Self {
        Self {
            power: Default::default(),
            toughness: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

impl idris::Idris for PowerToughness {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        std::any::type_name::<Self>()
    }
}
