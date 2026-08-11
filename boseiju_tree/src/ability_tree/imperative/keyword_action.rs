// mod keyword_to_abilities;

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

// pub use keyword_to_abilities::keyword_action_to_abilities;

use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

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
    pub span: boseiju_span::Span,
}

impl Node for KeywordAction {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::KeywordActionIdMarker
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut abilities = arrayvec::ArrayVec::new_const();
        abilities.push(&self.keyword as &dyn Node);
        abilities.push(&self.ability as &dyn Node);
        abilities
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for KeywordAction {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for KeywordAction {
    fn default() -> Self {
        Self {
            keyword: Default::default(),
            ability: Default::default(),
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

impl crate::Node for ExpandedKeywordAction {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::ExpandedKeywordAbilityIdMarker
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Adapt(child) => children.push(child as &dyn Node),
            Self::Airbend(child) => children.push(child as &dyn Node),
            Self::Amass(child) => children.push(child as &dyn Node),
            Self::Attach(child) => children.push(child as &dyn Node),
            Self::Behold(child) => children.push(child as &dyn Node),
            Self::Blight(child) => children.push(child as &dyn Node),
            Self::Bolster(child) => children.push(child as &dyn Node),
            Self::Cast(child) => children.push(child as &dyn Node),
            Self::Clash(child) => children.push(child as &dyn Node),
            Self::Cloak(child) => children.push(child as &dyn Node),
            Self::CollectEvidence(child) => children.push(child as &dyn Node),
            Self::Connive(child) => children.push(child as &dyn Node),
            Self::Convert(child) => children.push(child as &dyn Node),
            Self::Counter(child) => children.push(child as &dyn Node),
            Self::Create(child) => children.push(child as &dyn Node),
            Self::Destroy(child) => children.push(child as &dyn Node),
            Self::Detain(child) => children.push(child as &dyn Node),
            Self::Discard(child) => children.push(child as &dyn Node),
            Self::Discover(child) => children.push(child as &dyn Node),
            Self::Earthbend(child) => children.push(child as &dyn Node),
            Self::Endure(child) => children.push(child as &dyn Node),
            Self::Exert(child) => children.push(child as &dyn Node),
            Self::Exile(child) => children.push(child as &dyn Node),
            Self::Explore(child) => children.push(child as &dyn Node),
            Self::Fateseal(child) => children.push(child as &dyn Node),
            Self::Goad(child) => children.push(child as &dyn Node),
            Self::Incubate(child) => children.push(child as &dyn Node),
            Self::Manifest(child) => children.push(child as &dyn Node),
            Self::Mill(child) => children.push(child as &dyn Node),
            Self::Monstrosity(child) => children.push(child as &dyn Node),
            Self::Play(child) => children.push(child as &dyn Node),
            Self::Plot(child) => children.push(child as &dyn Node),
            Self::Regenerate(child) => children.push(child as &dyn Node),
            Self::Reveal(child) => children.push(child as &dyn Node),
            Self::Sacrifice(child) => children.push(child as &dyn Node),
            Self::Scry(child) => children.push(child as &dyn Node),
            Self::Search(child) => children.push(child as &dyn Node),
            Self::Standalone(child) => children.push(child as &dyn Node),
            Self::Support(child) => children.push(child as &dyn Node),
            Self::Surveil(child) => children.push(child as &dyn Node),
            Self::Suspect(child) => children.push(child as &dyn Node),
            Self::Tap(child) => children.push(child as &dyn Node),
            Self::Transform(child) => children.push(child as &dyn Node),
            Self::Untap(child) => children.push(child as &dyn Node),
            Self::Waterbend(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ExpandedKeywordAction {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::Adapt(child) => child.span(),
            Self::Airbend(child) => child.span(),
            Self::Amass(child) => child.span(),
            Self::Attach(child) => child.span(),
            Self::Behold(child) => child.span(),
            Self::Blight(child) => child.span(),
            Self::Bolster(child) => child.span(),
            Self::Cast(child) => child.span(),
            Self::Clash(child) => child.span(),
            Self::Cloak(child) => child.span(),
            Self::CollectEvidence(child) => child.span(),
            Self::Connive(child) => child.span(),
            Self::Convert(child) => child.span(),
            Self::Counter(child) => child.span(),
            Self::Create(child) => child.span(),
            Self::Destroy(child) => child.span(),
            Self::Detain(child) => child.span(),
            Self::Discard(child) => child.span(),
            Self::Discover(child) => child.span(),
            Self::Earthbend(child) => child.span(),
            Self::Endure(child) => child.span(),
            Self::Exert(child) => child.span(),
            Self::Exile(child) => child.span(),
            Self::Explore(child) => child.span(),
            Self::Fateseal(child) => child.span(),
            Self::Goad(child) => child.span(),
            Self::Incubate(child) => child.span(),
            Self::Manifest(child) => child.span(),
            Self::Mill(child) => child.span(),
            Self::Monstrosity(child) => child.span(),
            Self::Play(child) => child.span(),
            Self::Plot(child) => child.span(),
            Self::Regenerate(child) => child.span(),
            Self::Reveal(child) => child.span(),
            Self::Sacrifice(child) => child.span(),
            Self::Scry(child) => child.span(),
            Self::Search(child) => child.span(),
            Self::Standalone(child) => child.span(),
            Self::Support(child) => child.span(),
            Self::Surveil(child) => child.span(),
            Self::Suspect(child) => child.span(),
            Self::Tap(child) => child.span(),
            Self::Transform(child) => child.span(),
            Self::Untap(child) => child.span(),
            Self::Waterbend(child) => child.span(),
        }
    }
}

impl Default for ExpandedKeywordAction {
    fn default() -> Self {
        Self::Standalone(Default::default())
    }
}

/// Wrapper around the mtg type for the standalone keyword ability.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandaloneKeywordAction {
    pub keyword_action: boseiju_lexer::terminal::StandaloneKeywordAction,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for StandaloneKeywordAction {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::ExpandedKeywordAction(self.keyword_action.clone())
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = crate::NodeKind::ExpandedKeywordAction(self.keyword_action.clone());
        let child = crate::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "standalone: {}", self.keyword_action)
    }

    fn node_tag(&self) -> &'static str {
        "standalone keyword ability"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for StandaloneKeywordAction {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for StandaloneKeywordAction {
    const COUNT: usize = boseiju_lexer::terminal::StandaloneKeywordAbility::COUNT;
    fn id(&self) -> usize {
        self.keyword_action.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        boseiju_lexer::terminal::StandaloneKeywordAbility::name_from_id(id)
    }
}

impl Default for StandaloneKeywordAction {
    fn default() -> Self {
        Self {
            keyword_action: boseiju_lexer::terminal::StandaloneKeywordAction::Investigate,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
