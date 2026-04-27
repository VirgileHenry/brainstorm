mod affinity;
mod afterlife;
mod bestow;
mod blitz;
mod bloodthirst;
mod bushido;
mod cleave;
mod crew;
mod cycling;
mod dash;
mod disguise;
mod echo;
mod enchant;
mod equip;
mod flashback;
mod freerunning;
mod keyword_to_abilities;
mod kicker;
mod megamorph;
mod morph;
mod ninjutsu;
mod outlast;
mod prototype;
mod reinforce;
mod renown;
mod ripple;
mod suspend;
mod vanishing;
mod ward;
mod warp;

pub use keyword_to_abilities::keyword_to_abilities;

pub use affinity::AffinityKeywordAbility;
pub use afterlife::AfterlifeKeywordAbility;
pub use bestow::BestowKeywordAbility;
pub use blitz::BlitzKeywordAbility;
pub use bloodthirst::BloodthirstKeywordAbility;
pub use bushido::BushidoKeywordAbility;
pub use cleave::CleaveKeywordAbility;
pub use crew::CrewKeywordAbility;
pub use cycling::CyclingKeywordAbility;
pub use dash::DashKeywordAbility;
pub use disguise::DisguiseKeywordAbility;
pub use echo::EchoKeywordAbility;
pub use enchant::EnchantKeywordAbility;
pub use equip::EquipKeywordAbility;
pub use flashback::FlashbackKeywordAbility;
pub use freerunning::FreerunningKeywordAbility;
pub use kicker::KickerKeywordAbility;
pub use megamorph::MegamorphKeywordAbility;
pub use morph::MorphKeywordAbility;
pub use ninjutsu::NinjutsuKeywordAbility;
pub use outlast::OutlastKeywordAbility;
pub use prototype::PrototypeKeywordAbility;
pub use reinforce::ReinforceKeywordAbility;
pub use renown::RenownKeywordAbility;
pub use ripple::RippleKeywordAbility;
pub use suspend::SuspendKeywordAbility;
pub use vanishing::VanishingKeywordAbility;
pub use ward::WardKeywordAbility;
pub use warp::WarpKeywordAbility;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// This is basically a 1-1 copy of the [`mtg_data::KeywordAbility`],
/// expect all keyword abilities required additional text also have this text.
///
/// For instance, "Ward" on its own isn't truly a keyword abilty: It's "ward: pay 2 life".
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpandedKeywordAbility {
    Affinity(AffinityKeywordAbility),
    Afterlife(AfterlifeKeywordAbility),
    Bestow(BestowKeywordAbility),
    Blitz(BlitzKeywordAbility),
    Bloodthirst(BloodthirstKeywordAbility),
    Bushido(BushidoKeywordAbility),
    Cleave(CleaveKeywordAbility),
    Crew(CrewKeywordAbility),
    Cycling(CyclingKeywordAbility),
    Dash(DashKeywordAbility),
    Disguise(DisguiseKeywordAbility),
    Echo(EchoKeywordAbility),
    Enchant(EnchantKeywordAbility),
    Equip(EquipKeywordAbility),
    Flashback(FlashbackKeywordAbility),
    Freerunning(FreerunningKeywordAbility),
    Kicker(KickerKeywordAbility),
    Megamorph(MegamorphKeywordAbility),
    Morph(MorphKeywordAbility),
    Ninjutsu(NinjutsuKeywordAbility),
    Outlast(OutlastKeywordAbility),
    Prototype(PrototypeKeywordAbility),
    Reinforce(ReinforceKeywordAbility),
    Renown(RenownKeywordAbility),
    Ripple(RippleKeywordAbility),
    Standalone(StandaloneKeywordAbility),
    Suspend(SuspendKeywordAbility),
    Vanishing(VanishingKeywordAbility),
    Ward(WardKeywordAbility),
    Warp(WarpKeywordAbility),
}

impl crate::ability_tree::AbilityTreeNode for ExpandedKeywordAbility {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ExpandedKeywordAbilityIdMarker.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Affinity(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Afterlife(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Bestow(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Blitz(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Bloodthirst(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Bushido(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Cleave(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Crew(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Cycling(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Dash(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Disguise(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Echo(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Enchant(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Equip(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Flashback(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Freerunning(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Kicker(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Megamorph(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Morph(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Ninjutsu(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Outlast(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Prototype(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Reinforce(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Renown(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Ripple(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Standalone(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Suspend(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Vanishing(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Ward(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Warp(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "keyword ability:")?;
        out.push_final_branch()?;
        match self {
            Self::Affinity(child) => child.display(out)?,
            Self::Afterlife(child) => child.display(out)?,
            Self::Bestow(child) => child.display(out)?,
            Self::Blitz(child) => child.display(out)?,
            Self::Bloodthirst(child) => child.display(out)?,
            Self::Bushido(child) => child.display(out)?,
            Self::Cleave(child) => child.display(out)?,
            Self::Crew(child) => child.display(out)?,
            Self::Cycling(child) => child.display(out)?,
            Self::Dash(child) => child.display(out)?,
            Self::Disguise(child) => child.display(out)?,
            Self::Echo(child) => child.display(out)?,
            Self::Enchant(child) => child.display(out)?,
            Self::Equip(child) => child.display(out)?,
            Self::Flashback(child) => child.display(out)?,
            Self::Freerunning(child) => child.display(out)?,
            Self::Kicker(child) => child.display(out)?,
            Self::Megamorph(child) => child.display(out)?,
            Self::Morph(child) => child.display(out)?,
            Self::Ninjutsu(child) => child.display(out)?,
            Self::Outlast(child) => child.display(out)?,
            Self::Prototype(child) => child.display(out)?,
            Self::Reinforce(child) => child.display(out)?,
            Self::Renown(child) => child.display(out)?,
            Self::Ripple(child) => child.display(out)?,
            Self::Standalone(child) => child.display(out)?,
            Self::Suspend(child) => child.display(out)?,
            Self::Vanishing(child) => child.display(out)?,
            Self::Ward(child) => child.display(out)?,
            Self::Warp(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "keyword ability"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Affinity(child) => child.node_span(),
            Self::Afterlife(child) => child.node_span(),
            Self::Bestow(child) => child.node_span(),
            Self::Blitz(child) => child.node_span(),
            Self::Bloodthirst(child) => child.node_span(),
            Self::Bushido(child) => child.node_span(),
            Self::Cleave(child) => child.node_span(),
            Self::Crew(child) => child.node_span(),
            Self::Cycling(child) => child.node_span(),
            Self::Dash(child) => child.node_span(),
            Self::Disguise(child) => child.node_span(),
            Self::Echo(child) => child.node_span(),
            Self::Enchant(child) => child.node_span(),
            Self::Equip(child) => child.node_span(),
            Self::Flashback(child) => child.node_span(),
            Self::Freerunning(child) => child.node_span(),
            Self::Kicker(child) => child.node_span(),
            Self::Megamorph(child) => child.node_span(),
            Self::Morph(child) => child.node_span(),
            Self::Ninjutsu(child) => child.node_span(),
            Self::Outlast(child) => child.node_span(),
            Self::Prototype(child) => child.node_span(),
            Self::Reinforce(child) => child.node_span(),
            Self::Renown(child) => child.node_span(),
            Self::Ripple(child) => child.node_span(),
            Self::Standalone(child) => child.node_span(),
            Self::Suspend(child) => child.node_span(),
            Self::Vanishing(child) => child.node_span(),
            Self::Ward(child) => child.node_span(),
            Self::Warp(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ExpandedKeywordAbility {
    fn dummy_init() -> Self {
        Self::Standalone(crate::utils::dummy())
    }
}

/// Wrapper around the mtg type for the standalone keyword ability.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandaloneKeywordAbility {
    pub keyword_ability: crate::ability_tree::terminals::StandaloneKeywordAbility,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for StandaloneKeywordAbility {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ExpandedKeywordAbility(self.keyword_ability.clone()).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use idris::Idris;
        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = crate::ability_tree::NodeKind::ExpandedKeywordAbility(self.keyword_ability.clone()).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "standalone: {}", self.keyword_ability)
    }

    fn node_tag(&self) -> &'static str {
        "standalone keyword ability"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

impl idris::Idris for StandaloneKeywordAbility {
    const COUNT: usize = crate::ability_tree::terminals::StandaloneKeywordAbility::COUNT;
    fn id(&self) -> usize {
        self.keyword_ability.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        crate::ability_tree::terminals::StandaloneKeywordAbility::name_from_id(id)
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for StandaloneKeywordAbility {
    fn dummy_init() -> Self {
        Self {
            keyword_ability: crate::ability_tree::terminals::StandaloneKeywordAbility::Haste,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
