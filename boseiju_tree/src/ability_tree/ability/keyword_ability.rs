mod affinity;
mod afterlife;
mod annihilator;
mod backup;
mod bestow;
mod blitz;
mod bloodthirst;
mod bushido;
mod cleave;
mod crew;
mod cumulative_upkeep;
mod cycling;
mod dash;
mod disguise;
mod echo;
mod enchant;
mod equip;
mod fabricate;
mod flashback;
mod freerunning;
mod kicker;
mod megamorph;
mod morph;
mod ninjutsu;
mod outlast;
mod prototype;
mod rampage;
mod reconfigure;
mod reinforce;
mod renown;
mod ripple;
mod surge;
mod suspend;
mod vanishing;
mod ward;
mod warp;

pub use affinity::AffinityKeywordAbility;
pub use afterlife::AfterlifeKeywordAbility;
pub use annihilator::AnnihilatorKeywordAbility;
pub use backup::BackupKeywordAbility;
pub use bestow::BestowKeywordAbility;
pub use blitz::BlitzKeywordAbility;
pub use bloodthirst::BloodthirstKeywordAbility;
pub use bushido::BushidoKeywordAbility;
pub use cleave::CleaveKeywordAbility;
pub use crew::CrewKeywordAbility;
pub use cumulative_upkeep::CumulativeUpkeepKeywordAbility;
pub use cycling::CyclingKeywordAbility;
pub use dash::DashKeywordAbility;
pub use disguise::DisguiseKeywordAbility;
pub use echo::EchoKeywordAbility;
pub use enchant::EnchantKeywordAbility;
pub use equip::EquipKeywordAbility;
pub use fabricate::FabricateKeywordAbility;
pub use flashback::FlashbackKeywordAbility;
pub use freerunning::FreerunningKeywordAbility;
pub use kicker::KickerKeywordAbility;
pub use megamorph::MegamorphKeywordAbility;
pub use morph::MorphKeywordAbility;
pub use ninjutsu::NinjutsuKeywordAbility;
pub use outlast::OutlastKeywordAbility;
pub use prototype::PrototypeKeywordAbility;
pub use rampage::RampageKeywordAbility;
pub use reconfigure::ReconfigureKeywordAbility;
pub use reinforce::ReinforceKeywordAbility;
pub use renown::RenownKeywordAbility;
pub use ripple::RippleKeywordAbility;
pub use surge::SurgeKeywordAbility;
pub use suspend::SuspendKeywordAbility;
pub use vanishing::VanishingKeywordAbility;
pub use ward::WardKeywordAbility;
pub use warp::WarpKeywordAbility;

use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

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
    Annihilator(AnnihilatorKeywordAbility),
    Backup(BackupKeywordAbility),
    Bestow(BestowKeywordAbility),
    Blitz(BlitzKeywordAbility),
    Bloodthirst(BloodthirstKeywordAbility),
    Bushido(BushidoKeywordAbility),
    Cleave(CleaveKeywordAbility),
    Crew(CrewKeywordAbility),
    CumulativeUpkeep(CumulativeUpkeepKeywordAbility),
    Cycling(CyclingKeywordAbility),
    Dash(DashKeywordAbility),
    Disguise(DisguiseKeywordAbility),
    Echo(EchoKeywordAbility),
    Enchant(EnchantKeywordAbility),
    Equip(EquipKeywordAbility),
    Fabricate(FabricateKeywordAbility),
    Flashback(FlashbackKeywordAbility),
    Freerunning(FreerunningKeywordAbility),
    Kicker(KickerKeywordAbility),
    Megamorph(MegamorphKeywordAbility),
    Morph(MorphKeywordAbility),
    Ninjutsu(NinjutsuKeywordAbility),
    Outlast(OutlastKeywordAbility),
    Prototype(PrototypeKeywordAbility),
    Rampage(RampageKeywordAbility),
    Reinforce(ReinforceKeywordAbility),
    Reconfigure(ReconfigureKeywordAbility),
    Renown(RenownKeywordAbility),
    Ripple(RippleKeywordAbility),
    Standalone(StandaloneKeywordAbility),
    Surge(SurgeKeywordAbility),
    Suspend(SuspendKeywordAbility),
    Vanishing(VanishingKeywordAbility),
    Ward(WardKeywordAbility),
    Warp(WarpKeywordAbility),
}

impl crate::Node for ExpandedKeywordAbility {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::ExpandedKeywordAbilityIdMarker.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Affinity(child) => children.push(child as &dyn Node),
            Self::Afterlife(child) => children.push(child as &dyn Node),
            Self::Annihilator(child) => children.push(child as &dyn Node),
            Self::Backup(child) => children.push(child as &dyn Node),
            Self::Bestow(child) => children.push(child as &dyn Node),
            Self::Blitz(child) => children.push(child as &dyn Node),
            Self::Bloodthirst(child) => children.push(child as &dyn Node),
            Self::Bushido(child) => children.push(child as &dyn Node),
            Self::Cleave(child) => children.push(child as &dyn Node),
            Self::Crew(child) => children.push(child as &dyn Node),
            Self::CumulativeUpkeep(child) => children.push(child as &dyn Node),
            Self::Cycling(child) => children.push(child as &dyn Node),
            Self::Dash(child) => children.push(child as &dyn Node),
            Self::Disguise(child) => children.push(child as &dyn Node),
            Self::Echo(child) => children.push(child as &dyn Node),
            Self::Enchant(child) => children.push(child as &dyn Node),
            Self::Equip(child) => children.push(child as &dyn Node),
            Self::Fabricate(child) => children.push(child as &dyn Node),
            Self::Flashback(child) => children.push(child as &dyn Node),
            Self::Freerunning(child) => children.push(child as &dyn Node),
            Self::Kicker(child) => children.push(child as &dyn Node),
            Self::Megamorph(child) => children.push(child as &dyn Node),
            Self::Morph(child) => children.push(child as &dyn Node),
            Self::Ninjutsu(child) => children.push(child as &dyn Node),
            Self::Outlast(child) => children.push(child as &dyn Node),
            Self::Prototype(child) => children.push(child as &dyn Node),
            Self::Rampage(child) => children.push(child as &dyn Node),
            Self::Reinforce(child) => children.push(child as &dyn Node),
            Self::Reconfigure(child) => children.push(child as &dyn Node),
            Self::Renown(child) => children.push(child as &dyn Node),
            Self::Ripple(child) => children.push(child as &dyn Node),
            Self::Standalone(child) => children.push(child as &dyn Node),
            Self::Surge(child) => children.push(child as &dyn Node),
            Self::Suspend(child) => children.push(child as &dyn Node),
            Self::Vanishing(child) => children.push(child as &dyn Node),
            Self::Ward(child) => children.push(child as &dyn Node),
            Self::Warp(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "keyword ability:")?;
        out.push_final_branch()?;
        match self {
            Self::Affinity(child) => child.display(out)?,
            Self::Afterlife(child) => child.display(out)?,
            Self::Annihilator(child) => child.display(out)?,
            Self::Backup(child) => child.display(out)?,
            Self::Bestow(child) => child.display(out)?,
            Self::Blitz(child) => child.display(out)?,
            Self::Bloodthirst(child) => child.display(out)?,
            Self::Bushido(child) => child.display(out)?,
            Self::Cleave(child) => child.display(out)?,
            Self::Crew(child) => child.display(out)?,
            Self::CumulativeUpkeep(child) => child.display(out)?,
            Self::Cycling(child) => child.display(out)?,
            Self::Dash(child) => child.display(out)?,
            Self::Disguise(child) => child.display(out)?,
            Self::Echo(child) => child.display(out)?,
            Self::Enchant(child) => child.display(out)?,
            Self::Equip(child) => child.display(out)?,
            Self::Fabricate(child) => child.display(out)?,
            Self::Flashback(child) => child.display(out)?,
            Self::Freerunning(child) => child.display(out)?,
            Self::Kicker(child) => child.display(out)?,
            Self::Megamorph(child) => child.display(out)?,
            Self::Morph(child) => child.display(out)?,
            Self::Ninjutsu(child) => child.display(out)?,
            Self::Outlast(child) => child.display(out)?,
            Self::Prototype(child) => child.display(out)?,
            Self::Rampage(child) => child.display(out)?,
            Self::Reinforce(child) => child.display(out)?,
            Self::Reconfigure(child) => child.display(out)?,
            Self::Renown(child) => child.display(out)?,
            Self::Ripple(child) => child.display(out)?,
            Self::Standalone(child) => child.display(out)?,
            Self::Surge(child) => child.display(out)?,
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ExpandedKeywordAbility {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Affinity(child) => child.span(),
            Self::Afterlife(child) => child.span(),
            Self::Annihilator(child) => child.span(),
            Self::Backup(child) => child.span(),
            Self::Bestow(child) => child.span(),
            Self::Blitz(child) => child.span(),
            Self::Bloodthirst(child) => child.span(),
            Self::Bushido(child) => child.span(),
            Self::Cleave(child) => child.span(),
            Self::Crew(child) => child.span(),
            Self::CumulativeUpkeep(child) => child.span(),
            Self::Cycling(child) => child.span(),
            Self::Dash(child) => child.span(),
            Self::Disguise(child) => child.span(),
            Self::Echo(child) => child.span(),
            Self::Enchant(child) => child.span(),
            Self::Equip(child) => child.span(),
            Self::Fabricate(child) => child.span(),
            Self::Flashback(child) => child.span(),
            Self::Freerunning(child) => child.span(),
            Self::Kicker(child) => child.span(),
            Self::Megamorph(child) => child.span(),
            Self::Morph(child) => child.span(),
            Self::Ninjutsu(child) => child.span(),
            Self::Outlast(child) => child.span(),
            Self::Prototype(child) => child.span(),
            Self::Rampage(child) => child.span(),
            Self::Reinforce(child) => child.span(),
            Self::Reconfigure(child) => child.span(),
            Self::Renown(child) => child.span(),
            Self::Ripple(child) => child.span(),
            Self::Standalone(child) => child.span(),
            Self::Surge(child) => child.span(),
            Self::Suspend(child) => child.span(),
            Self::Vanishing(child) => child.span(),
            Self::Ward(child) => child.span(),
            Self::Warp(child) => child.span(),
        }
    }
}

impl Default for ExpandedKeywordAbility {
    fn default() -> Self {
        Self::Standalone(Default::default())
    }
}

/// Wrapper around the mtg type for the standalone keyword ability.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandaloneKeywordAbility {
    pub keyword_ability: boseiju_lexer::terminal::StandaloneKeywordAbility,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for StandaloneKeywordAbility {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::ExpandedKeywordAbility(self.keyword_ability.clone()).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        use idris::Idris;
        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = crate::NodeKind::ExpandedKeywordAbility(self.keyword_ability.clone()).id();
        let child = crate::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "standalone: {}", self.keyword_ability)
    }

    fn node_tag(&self) -> &'static str {
        "standalone keyword ability"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for StandaloneKeywordAbility {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for StandaloneKeywordAbility {
    const COUNT: usize = boseiju_lexer::terminal::StandaloneKeywordAbility::COUNT;
    fn id(&self) -> usize {
        self.keyword_ability.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        boseiju_lexer::terminal::StandaloneKeywordAbility::name_from_id(id)
    }
}

impl Default for StandaloneKeywordAbility {
    fn default() -> Self {
        Self {
            keyword_ability: boseiju_lexer::terminal::StandaloneKeywordAbility::Haste,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
