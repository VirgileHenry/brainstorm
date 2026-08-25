use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

const MAX_MANA_POSSIBILITIES: usize = MAX_CHILDREN_PER_NODE - 1;

/// Deed to add mana to the mana pool.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddMana<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    pub player: crate::ability_tree::player::PlayerReference<Q>,
    pub possibilities: crate::HeapArrayVec<ManaToAdd, MAX_MANA_POSSIBILITIES>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl<Q> crate::Node for AddMana<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::DeedKind(crate::node_kind::DeedNodeKind::PayMana)
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.player as &dyn Node);
        for mana in self.possibilities.iter() {
            children.push(mana as &dyn Node);
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "pay mana:")?;
        out.push_inter_branch()?;
        write!(out, "player:")?;
        out.push_final_branch()?;
        self.player.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "possiblities:")?;
        for (i, possibility) in self.possibilities.iter().enumerate() {
            if i == self.possibilities.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            possibility.display(out)?;
            out.pop_branch();
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "pay mana imperative"
    }
}

#[cfg(feature = "spanned_tree")]
impl<Q> boseiju_span::Spanned for AddMana<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
{
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl<Q> Default for AddMana<Q>
where
    Q: crate::ability_tree::quantifier::Quantifier,
    Q: Default,
{
    fn default() -> Self {
        Self {
            player: Default::default(),
            possibilities: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

pub type AddManaActive = AddMana<crate::ability_tree::quantifier::ActiveQuantifier>;
pub type AddManaPassive = AddMana<crate::ability_tree::quantifier::PassiveQuantifier>;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManaToAdd {
    AnyColor(ManaToAddOfAnyColor),
    Symbols(ManaToAddSymbols),
}

impl crate::Node for ManaToAdd {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::ManaToAdd
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::AnyColor(child) => children.push(child as &dyn Node),
            Self::Symbols(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "mana to add")?;
        out.push_final_branch()?;
        match self {
            Self::AnyColor(child) => child.display(out)?,
            Self::Symbols(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "mana to add"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ManaToAdd {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::AnyColor(child) => child.span(),
            Self::Symbols(child) => child.span(),
        }
    }
}

impl Default for ManaToAdd {
    fn default() -> Self {
        Self::Symbols(Default::default())
    }
}

/// List of mana symbols to add to the mana pool.
///
/// For instance:
/// - {R}
/// - {R}{R}{R}{R}{R}
/// - {B}{R}{G}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManaToAddSymbols {
    pub symbols: crate::HeapArrayVec<boseiju_lexer::terminal::ManaSymbol, MAX_CHILDREN_PER_NODE>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for ManaToAddSymbols {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::ManaToAddSymbols
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for symbol in self.symbols.iter() {
            children.push(symbol as &dyn Node);
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "mana symbols")?;
        out.push_final_branch()?;
        for symbol in self.symbols.iter() {
            symbol.display(out)?;
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "mana to add kind"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ManaToAddSymbols {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for ManaToAddSymbols {
    fn default() -> Self {
        Self {
            symbols: crate::HeapArrayVec::new(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

/// A mana of any color to add.
///
/// This is the equivalent of "<amount> mana of any color"
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManaToAddOfAnyColor {
    pub amount: crate::ability_tree::number::Number,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for ManaToAddOfAnyColor {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::ManaToAddOfAnyColor
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.amount as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "mana of any color:")?;
        out.push_final_branch()?;
        write!(out, "amount:")?;
        out.push_final_branch()?;
        self.amount.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "mana to add kind"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ManaToAddOfAnyColor {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for ManaToAddOfAnyColor {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
