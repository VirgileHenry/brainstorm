use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

const MAX_DAMAGES_DEALT: usize = MAX_CHILDREN_PER_NODE - 1;

/// Imperative to deal damage to things.
///
/// The imperative can deals multiple damages to multiple targets,
/// for instance Drakuseth, Maw of Flames states: "it deals 4 damage to any
/// target and 3 damage to each of up to two other targets."
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DealsDamageImperative {
    pub dealer: crate::ability_tree::object::Card,
    pub damages: crate::HeapArrayVec<DamagesDealt, MAX_DAMAGES_DEALT>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for DealsDamageImperative {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::DealsDamageImperative.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.dealer as &dyn Node);
        for damage in self.damages.iter() {
            children.push(damage as &dyn Node);
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "deals damage:")?;
        out.push_inter_branch()?;
        write!(out, "dealer:")?;
        out.push_final_branch()?;
        self.dealer.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "damages dealt:")?;
        for (i, damage) in self.damages.iter().enumerate() {
            if i == self.damages.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            damage.display(out)?;
            out.pop_branch();
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "deal damage imperative"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for DealsDamageImperative {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for DealsDamageImperative {
    fn default() -> Self {
        Self {
            dealer: Default::default(),
            damages: crate::HeapArrayVec::new(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

/// Single damage dealing action.
///
/// This can be the same damage to multiple targets, or shared damage among targets,
/// or anything that was mentionned in a single damage dealing action.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DamagesDealt {
    pub to: crate::ability_tree::object::DamageReceiver,
    pub amount: crate::ability_tree::number::Number,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for DamagesDealt {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::DamagesDealt.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.to as &dyn Node);
        children.push(&self.amount as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "damage dealt:")?;
        out.push_inter_branch()?;
        write!(out, "to:")?;
        out.push_final_branch()?;
        self.to.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "amount:")?;
        out.push_final_branch()?;
        self.amount.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "damage dealt"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for DamagesDealt {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for DamagesDealt {
    fn default() -> Self {
        Self {
            to: Default::default(),
            amount: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
