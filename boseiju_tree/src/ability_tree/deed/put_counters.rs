use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

const MAX_COUNTER_AMOUNT: usize = MAX_CHILDREN_PER_NODE - 1;

/// Deed forcounters that are put on permanents.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PutCounters<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    pub object: crate::ability_tree::object::Permanent<Q>,
    pub counters: crate::HeapArrayVec<CounterOnPermanent, MAX_COUNTER_AMOUNT>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl<Q> Node for PutCounters<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::DeedKind(crate::node_kind::DeedNodeKind::PutCounters)
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.object as &dyn Node);
        for counter in self.counters.iter() {
            children.push(counter as &dyn Node);
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "put counters:")?;
        out.push_inter_branch()?;
        write!(out, "on object:")?;
        out.push_final_branch()?;
        self.object.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "counters:")?;
        for (i, counter) in self.counters.iter().enumerate() {
            if i == self.counters.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            counter.display(out)?;
            out.pop_branch();
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "put counters imperative"
    }
}

#[cfg(feature = "spanned_tree")]
impl<Q> boseiju_span::Spanned for PutCounters<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl<Q> Default for PutCounters<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
    Q: Default,
{
    fn default() -> Self {
        Self {
            object: Default::default(),
            counters: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

pub type PutCountersActive = PutCounters<crate::ability_tree::quantifier::ActiveQuantifier>;
pub type PutCountersPassive = PutCounters<crate::ability_tree::quantifier::PassiveQuantifier>;

/// An amount and a kind of counters to be put on a permanent.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CounterOnPermanent {
    pub amount: crate::ability_tree::number::Number,
    pub counter: CounterKind,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for CounterOnPermanent {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::CounterOnPermanent
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.amount as &dyn Node);
        children.push(&self.counter as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "counter on permanent:")?;
        out.push_inter_branch()?;
        write!(out, "amount:")?;
        out.push_final_branch()?;
        self.amount.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "of counter:")?;
        out.push_final_branch()?;
        self.counter.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "counters on permanent"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CounterOnPermanent {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

/// Kind of counter that is put on a permanent.
///
/// It's either a given kind of counter, e.g. "put a shield counter" or
/// a previously mentionned kind of counter, e.g. "that many counters on...".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CounterKind {
    NewCounter(boseiju_lexer::terminal::Counter),
}

impl Node for CounterKind {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::CounterKind
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::NewCounter(counter) => children.push(counter as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "counter kind:")?;
        out.push_final_branch()?;
        match self {
            Self::NewCounter(counter) => counter.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "counter kind"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CounterKind {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::NewCounter(child) => child.span(),
        }
    }
}

impl Default for CounterKind {
    fn default() -> Self {
        Self::NewCounter(Default::default())
    }
}
