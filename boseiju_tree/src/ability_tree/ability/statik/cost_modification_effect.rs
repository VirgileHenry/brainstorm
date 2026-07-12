use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// Modification of the cost of objects.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CostModificationEffect {
    pub applies_to: crate::ability_tree::object::Spell,
    pub modification: CostModification,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for CostModificationEffect {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::CostModificationEffect.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.applies_to as &dyn Node);
        children.push(&self.modification as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "cost modification effect:")?;
        out.push_inter_branch()?;
        write!(out, "applies to:")?;
        out.push_final_branch()?;
        self.applies_to.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        self.modification.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "cost modification effect"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CostModificationEffect {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for CostModificationEffect {
    fn default() -> Self {
        Self {
            applies_to: Default::default(),
            modification: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

/// Modification of a cost.
/// Either an additional cost, a reduction cost or a "set to" cost.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CostModification {
    More(CostModificationCostMore),
    Less(CostModificationCostLess),
    Set(CostModificationCostSet),
}

impl crate::Node for CostModification {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::CostModification.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::More(child) => children.push(child as &dyn Node),
            Self::Less(child) => children.push(child as &dyn Node),
            Self::Set(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "cost modification effect:")?;
        out.push_final_branch()?;
        match self {
            Self::More(child) => child.display(out)?,
            Self::Less(child) => child.display(out)?,
            Self::Set(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "cost modification"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CostModification {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::More(child) => child.span(),
            Self::Less(child) => child.span(),
            Self::Set(child) => child.span(),
        }
    }
}

impl Default for CostModification {
    fn default() -> Self {
        Self::Less(Default::default())
    }
}

/// An "additionnal cost" cost modification.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CostModificationCostMore {
    pub more: crate::ability_tree::mana_cost::ManaCost,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for CostModificationCostMore {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::CostModificationCostMore.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.more as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "cost ")?;
        self.more.display(out)?;
        write!(out, " more to cast")?;
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "cost modification: cost more"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CostModificationCostMore {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for CostModificationCostMore {
    fn default() -> Self {
        Self {
            more: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

/// A "cost less" cost modification.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CostModificationCostLess {
    pub less: crate::ability_tree::mana_cost::ManaCost,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for CostModificationCostLess {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::CostModificationCostLess.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.less as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "cost ")?;
        self.less.display(out)?;
        write!(out, " less to cast")?;
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "cost modification: cost less"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CostModificationCostLess {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for CostModificationCostLess {
    fn default() -> Self {
        Self {
            less: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

/// A "cost set to" cost modification.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CostModificationCostSet {
    pub set: crate::ability_tree::mana_cost::ManaCost,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for CostModificationCostSet {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::CostModificationCostSet.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.set as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "cost ")?;
        self.set.display(out)?;
        write!(out, " to cast")?;
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "cost modification: set cost to"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CostModificationCostSet {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for CostModificationCostSet {
    fn default() -> Self {
        Self {
            set: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
