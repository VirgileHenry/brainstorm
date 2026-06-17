mod keyword_to_abilities;

pub mod activate;
pub mod adapt;
pub mod airbend;
pub mod amass;
pub mod attach;
pub mod behold;
pub mod blight;
pub mod bolster;
pub mod cast;
pub mod clash;
pub mod cloak;
pub mod collect_evidence;
pub mod connive;
pub mod convert;
pub mod counter;
pub mod create;
pub mod destroy;
pub mod detain;
pub mod discard;
pub mod discover;
pub mod double;
pub mod earthbend;
pub mod endure;
pub mod exchange;
pub mod exert;
pub mod exile;
pub mod explore;
pub mod fateseal;
pub mod fight;
pub mod goad;
pub mod incubate;
pub mod manifest;
pub mod meld;
pub mod mill;
pub mod monstrosity;
pub mod play;
pub mod plot;
pub mod regenerate;
pub mod reveal;
pub mod sacrifice;
pub mod scry;
pub mod search;
pub mod support;
pub mod surveil;
pub mod suspect;
pub mod tap;
pub mod transform;
pub mod untap;
pub mod vote;
pub mod waterbend;

pub use keyword_to_abilities::keyword_action_to_abilities;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A Keyword Action.
///
/// From the comprehensive rules:
/// A verb, such as “destroy” or “cast,” used as a game term rather than
/// as its normal English meaning. See rule 701, “Keyword Actions.”
///
/// See also <https://mtg.fandom.com/wiki/Keyword_action>
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordAction {
    pub keyword: ExpandedKeywordAction,
    pub ability: crate::ability_tree::ability::spell::SpellAbility,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for KeywordAction {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::KeywordActionIdMarker.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut abilities = arrayvec::ArrayVec::new_const();
        abilities.push(&self.keyword as &dyn AbilityTreeNode);
        abilities.push(&self.ability as &dyn AbilityTreeNode);
        abilities
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "keyword ability:")?;
        out.push_inter_branch()?;
        write!(out, "keyword: ")?;
        self.keyword.display(out)?;
        out.next_final_branch()?;
        write!(out, "expanded ability: ")?;
        self.ability.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "keyword ability"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for KeywordAction {
    fn dummy_init() -> Self {
        Self {
            keyword: crate::utils::dummy(),
            ability: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

/// This is basically a 1-1 copy of the [`mtg_data::KeywordAbility`],
/// expect all keyword abilities required additional text also have this text.
///
/// For instance, "Ward" on its own isn't truly a keyword abilty: It's "ward: pay 2 life".
///
/// Fixme: destroy shall be here, and the imperative is to move a creature from battlefield to graveyard.
/// We shall ensure the thing gets the state "destroyed", so indestructible can be written as
/// "event cant happen" -> "creature has state" -> "destroyed"
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpandedKeywordAction {
    Adapt(adapt::AdaptKeywordAction),
    Airbend(airbend::AirbendKeywordAction),
    Amass(amass::AmassKeywordAction),
    Attach(attach::AttachKeywordAction),
    Behold(behold::BeholdKeywordAction),
    Blight(blight::BlightKeywordAction),
    Bolster(bolster::BolsterKeywordAction),
    Cast(cast::CastKeywordAction),
    Clash(clash::ClashKeywordAction),
    Cloak(cloak::CloakKeywordAction),
    CollectEvidence(collect_evidence::CollectEvidenceKeywordAction),
    Connive(connive::ConniveKeywordAction),
    Convert(convert::ConvertKeywordAction),
    Counter(counter::CounterKeywordAction),
    Create(create::CreateKeywordAction),
    Destroy(destroy::DestroyKeywordAction),
    Detain(detain::DetainKeywordAction),
    Discard(discard::DiscardKeywordAction),
    Discover(discover::DiscoverKeywordAction),
    Earthbend(earthbend::EarthbendKeywordAction),
    Endure(endure::EndureKeywordAction),
    Exert(exert::ExertKeywordAction),
    Exile(exile::ExileKeywordAction),
    Explore(explore::ExploreKeywordAction),
    Fateseal(fateseal::FatesealKeywordAction),
    Goad(goad::GoadKeywordAction),
    Incubate(incubate::IncubateKeywordAction),
    Manifest(manifest::ManifestKeywordAction),
    Mill(mill::MillKeywordAction),
    Monstrosity(monstrosity::MonstrosityKeywordAction),
    Play(play::PlayKeywordAction),
    Plot(plot::PlotKeywordAction),
    Regenerate(regenerate::RegenerateKeywordAction),
    Reveal(reveal::RevealKeywordAction),
    Sacrifice(sacrifice::SacrificeKeywordAction),
    Scry(scry::ScryKeywordAction),
    Search(search::SearchKeywordAction),
    Standalone(StandaloneKeywordAction),
    Support(support::SupportKeywordAction),
    Surveil(surveil::SurveilKeywordAction),
    Suspect(suspect::SuspectKeywordAction),
    Tap(tap::TapKeywordAction),
    Transform(transform::TransformKeywordAction),
    Untap(untap::UntapKeywordAction),
    Waterbend(waterbend::WaterbendKeywordAction),
}

impl crate::ability_tree::AbilityTreeNode for ExpandedKeywordAction {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ExpandedKeywordAbilityIdMarker.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Adapt(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Airbend(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Amass(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Attach(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Behold(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Blight(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Bolster(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Cast(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Clash(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Cloak(child) => children.push(child as &dyn AbilityTreeNode),
            Self::CollectEvidence(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Connive(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Convert(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Counter(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Create(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Destroy(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Detain(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Discard(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Discover(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Earthbend(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Endure(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Exert(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Exile(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Explore(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Fateseal(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Goad(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Incubate(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Manifest(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Mill(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Monstrosity(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Play(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Plot(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Regenerate(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Reveal(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Sacrifice(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Scry(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Search(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Standalone(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Support(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Surveil(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Suspect(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Tap(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Transform(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Untap(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Waterbend(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "keyword ability:")?;
        out.push_final_branch()?;
        match self {
            Self::Adapt(child) => child.display(out)?,
            Self::Airbend(child) => child.display(out)?,
            Self::Amass(child) => child.display(out)?,
            Self::Attach(child) => child.display(out)?,
            Self::Behold(child) => child.display(out)?,
            Self::Blight(child) => child.display(out)?,
            Self::Bolster(child) => child.display(out)?,
            Self::Cast(child) => child.display(out)?,
            Self::Clash(child) => child.display(out)?,
            Self::Cloak(child) => child.display(out)?,
            Self::CollectEvidence(child) => child.display(out)?,
            Self::Connive(child) => child.display(out)?,
            Self::Convert(child) => child.display(out)?,
            Self::Counter(child) => child.display(out)?,
            Self::Create(child) => child.display(out)?,
            Self::Destroy(child) => child.display(out)?,
            Self::Detain(child) => child.display(out)?,
            Self::Discard(child) => child.display(out)?,
            Self::Discover(child) => child.display(out)?,
            Self::Earthbend(child) => child.display(out)?,
            Self::Endure(child) => child.display(out)?,
            Self::Exert(child) => child.display(out)?,
            Self::Exile(child) => child.display(out)?,
            Self::Explore(child) => child.display(out)?,
            Self::Fateseal(child) => child.display(out)?,
            Self::Goad(child) => child.display(out)?,
            Self::Incubate(child) => child.display(out)?,
            Self::Manifest(child) => child.display(out)?,
            Self::Mill(child) => child.display(out)?,
            Self::Monstrosity(child) => child.display(out)?,
            Self::Play(child) => child.display(out)?,
            Self::Plot(child) => child.display(out)?,
            Self::Regenerate(child) => child.display(out)?,
            Self::Reveal(child) => child.display(out)?,
            Self::Sacrifice(child) => child.display(out)?,
            Self::Scry(child) => child.display(out)?,
            Self::Search(child) => child.display(out)?,
            Self::Standalone(child) => child.display(out)?,
            Self::Support(child) => child.display(out)?,
            Self::Surveil(child) => child.display(out)?,
            Self::Suspect(child) => child.display(out)?,
            Self::Tap(child) => child.display(out)?,
            Self::Transform(child) => child.display(out)?,
            Self::Untap(child) => child.display(out)?,
            Self::Waterbend(child) => child.display(out)?,
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
            Self::Adapt(child) => child.node_span(),
            Self::Airbend(child) => child.node_span(),
            Self::Amass(child) => child.node_span(),
            Self::Attach(child) => child.node_span(),
            Self::Behold(child) => child.node_span(),
            Self::Blight(child) => child.node_span(),
            Self::Bolster(child) => child.node_span(),
            Self::Cast(child) => child.node_span(),
            Self::Clash(child) => child.node_span(),
            Self::Cloak(child) => child.node_span(),
            Self::CollectEvidence(child) => child.node_span(),
            Self::Connive(child) => child.node_span(),
            Self::Convert(child) => child.node_span(),
            Self::Counter(child) => child.node_span(),
            Self::Create(child) => child.node_span(),
            Self::Destroy(child) => child.node_span(),
            Self::Detain(child) => child.node_span(),
            Self::Discard(child) => child.node_span(),
            Self::Discover(child) => child.node_span(),
            Self::Earthbend(child) => child.node_span(),
            Self::Endure(child) => child.node_span(),
            Self::Exert(child) => child.node_span(),
            Self::Exile(child) => child.node_span(),
            Self::Explore(child) => child.node_span(),
            Self::Fateseal(child) => child.node_span(),
            Self::Goad(child) => child.node_span(),
            Self::Incubate(child) => child.node_span(),
            Self::Manifest(child) => child.node_span(),
            Self::Mill(child) => child.node_span(),
            Self::Monstrosity(child) => child.node_span(),
            Self::Play(child) => child.node_span(),
            Self::Plot(child) => child.node_span(),
            Self::Regenerate(child) => child.node_span(),
            Self::Reveal(child) => child.node_span(),
            Self::Sacrifice(child) => child.node_span(),
            Self::Scry(child) => child.node_span(),
            Self::Search(child) => child.node_span(),
            Self::Standalone(child) => child.node_span(),
            Self::Support(child) => child.node_span(),
            Self::Surveil(child) => child.node_span(),
            Self::Suspect(child) => child.node_span(),
            Self::Tap(child) => child.node_span(),
            Self::Transform(child) => child.node_span(),
            Self::Untap(child) => child.node_span(),
            Self::Waterbend(child) => child.node_span(),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ExpandedKeywordAction {
    fn dummy_init() -> Self {
        Self::Standalone(crate::utils::dummy())
    }
}

/// Wrapper around the mtg type for the standalone keyword ability.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandaloneKeywordAction {
    pub keyword_action: crate::ability_tree::terminals::StandaloneKeywordAction,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for StandaloneKeywordAction {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ExpandedKeywordAction(self.keyword_action.clone()).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use idris::Idris;
        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = crate::ability_tree::NodeKind::ExpandedKeywordAction(self.keyword_action.clone()).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "standalone: {}", self.keyword_action)
    }

    fn node_tag(&self) -> &'static str {
        "standalone keyword ability"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

impl idris::Idris for StandaloneKeywordAction {
    const COUNT: usize = crate::ability_tree::terminals::StandaloneKeywordAbility::COUNT;
    fn id(&self) -> usize {
        self.keyword_action.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        crate::ability_tree::terminals::StandaloneKeywordAbility::name_from_id(id)
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for StandaloneKeywordAction {
    fn dummy_init() -> Self {
        Self {
            keyword_action: crate::ability_tree::terminals::StandaloneKeywordAction::Investigate,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
