use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// Modify set power and toughness of a creature.
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PowerToughnessModifiers {
    MinusMinus(PowerToughnessModifiersMinusMinus),
    MinusPlus(PowerToughnessModifiersMinusPlus),
    PlusMinus(PowerToughnessModifiersPlusMinus),
    PlusPlus(PowerToughnessModifiersPlusPlus),
    Set(PowerToughnessModifiersSet),
}

impl Node for PowerToughnessModifiers {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::PowerToughnessModifiers
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::MinusMinus(child) => children.push(child as &dyn Node),
            Self::MinusPlus(child) => children.push(child as &dyn Node),
            Self::PlusMinus(child) => children.push(child as &dyn Node),
            Self::PlusPlus(child) => children.push(child as &dyn Node),
            Self::Set(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "power toughness modifier:")?;
        out.push_final_branch()?;
        match self {
            Self::MinusMinus(child) => child.display(out)?,
            Self::MinusPlus(child) => child.display(out)?,
            Self::PlusMinus(child) => child.display(out)?,
            Self::PlusPlus(child) => child.display(out)?,
            Self::Set(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "power / toughness modifiers"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PowerToughnessModifiers {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::MinusMinus(child) => child.span(),
            Self::MinusPlus(child) => child.span(),
            Self::PlusMinus(child) => child.span(),
            Self::PlusPlus(child) => child.span(),
            Self::Set(child) => child.span(),
        }
    }
}

impl Default for PowerToughnessModifiers {
    fn default() -> Self {
        Self::MinusMinus(Default::default())
    }
}

/// A +X/+X power and toughness modifier.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerToughnessModifiersPlusPlus {
    pub power_mod: crate::ability_tree::number::Number,
    pub toughness_mod: crate::ability_tree::number::Number,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for PowerToughnessModifiersPlusPlus {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::PowerToughnessModifiersPlusPlus
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.power_mod as &dyn Node);
        children.push(&self.toughness_mod as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "+power / +toughness mod:")?;
        out.push_inter_branch()?;
        write!(out, "power mod:")?;
        out.push_final_branch()?;
        write!(out, "+")?;
        self.power_mod.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "toughness mod:")?;
        out.push_final_branch()?;
        write!(out, "+")?;
        self.toughness_mod.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "+/+ modifiers"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PowerToughnessModifiersPlusPlus {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for PowerToughnessModifiersPlusPlus {
    fn default() -> Self {
        Self {
            power_mod: Default::default(),
            toughness_mod: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

impl idris::Idris for PowerToughnessModifiersPlusPlus {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        std::any::type_name::<Self>()
    }
}

/// A +X/-X power and toughness modifier.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerToughnessModifiersPlusMinus {
    pub power_mod: crate::ability_tree::number::Number,
    pub toughness_mod: crate::ability_tree::number::Number,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for PowerToughnessModifiersPlusMinus {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::PowerToughnessModifiersPlusMinus
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.power_mod as &dyn Node);
        children.push(&self.toughness_mod as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "+power / -toughness mod:")?;
        out.push_inter_branch()?;
        write!(out, "power mod:")?;
        out.push_final_branch()?;
        write!(out, "+")?;
        self.power_mod.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "toughness mod:")?;
        out.push_final_branch()?;
        write!(out, "-")?;
        self.toughness_mod.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "+/- modifiers"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PowerToughnessModifiersPlusMinus {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for PowerToughnessModifiersPlusMinus {
    fn default() -> Self {
        Self {
            power_mod: Default::default(),
            toughness_mod: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

impl idris::Idris for PowerToughnessModifiersPlusMinus {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        std::any::type_name::<Self>()
    }
}

/// A -X/-X power and toughness modifier.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerToughnessModifiersMinusMinus {
    pub power_mod: crate::ability_tree::number::Number,
    pub toughness_mod: crate::ability_tree::number::Number,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for PowerToughnessModifiersMinusMinus {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::PowerToughnessModifiersMinusMinus
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.power_mod as &dyn Node);
        children.push(&self.toughness_mod as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "-power / -toughness mod:")?;
        out.push_inter_branch()?;
        write!(out, "power mod:")?;
        out.push_final_branch()?;
        write!(out, "-")?;
        self.power_mod.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "toughness mod:")?;
        out.push_final_branch()?;
        write!(out, "-")?;
        self.toughness_mod.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "-/- modifiers"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PowerToughnessModifiersMinusMinus {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for PowerToughnessModifiersMinusMinus {
    fn default() -> Self {
        Self {
            power_mod: Default::default(),
            toughness_mod: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

impl idris::Idris for PowerToughnessModifiersMinusMinus {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        std::any::type_name::<Self>()
    }
}

/// A -X/-X power and toughness modifier.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerToughnessModifiersMinusPlus {
    pub power_mod: crate::ability_tree::number::Number,
    pub toughness_mod: crate::ability_tree::number::Number,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for PowerToughnessModifiersMinusPlus {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::PowerToughnessModifiersMinusPlus
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.power_mod as &dyn Node);
        children.push(&self.toughness_mod as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "-power / +toughness mod:")?;
        out.push_inter_branch()?;
        write!(out, "power mod:")?;
        out.push_final_branch()?;
        write!(out, "-")?;
        self.power_mod.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "toughness mod:")?;
        out.push_final_branch()?;
        write!(out, "+")?;
        self.toughness_mod.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "-/+ modifiers"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PowerToughnessModifiersMinusPlus {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for PowerToughnessModifiersMinusPlus {
    fn default() -> Self {
        Self {
            power_mod: Default::default(),
            toughness_mod: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

impl idris::Idris for PowerToughnessModifiersMinusPlus {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        std::any::type_name::<Self>()
    }
}

/// A -X/-X power and toughness modifier.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerToughnessModifiersSet {
    pub power: crate::ability_tree::number::Number,
    pub toughness: crate::ability_tree::number::Number,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for PowerToughnessModifiersSet {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::PowerToughnessModifiersSet
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.power as &dyn Node);
        children.push(&self.toughness as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "power / toughness set mod:")?;
        out.push_inter_branch()?;
        write!(out, "power set to:")?;
        out.push_final_branch()?;
        self.power.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "toughness set to:")?;
        out.push_final_branch()?;
        self.toughness.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "set to value p/t modifiers"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PowerToughnessModifiersSet {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for PowerToughnessModifiersSet {
    fn default() -> Self {
        Self {
            power: Default::default(),
            toughness: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

impl idris::Idris for PowerToughnessModifiersSet {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        std::any::type_name::<Self>()
    }
}
