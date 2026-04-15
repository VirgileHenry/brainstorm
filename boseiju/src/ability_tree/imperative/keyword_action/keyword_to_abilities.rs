use crate::ability_tree::ability::spell::SpellAbility;
use crate::ability_tree::imperative::KeywordAction;
use crate::ability_tree::imperative::keyword_action::ExpandedKeywordAction;
use crate::ability_tree::imperative::keyword_action::StandaloneKeywordAction;
use crate::ability_tree::terminals;
use crate::lexer::tokens::intermediates;

pub fn keyword_action_to_abilities(keyword: intermediates::KeywordAction) -> Result<KeywordAction, &'static str> {
    let keyword_action = match keyword.keyword_action {
        mtg_data::KeywordAction::Abandon => terminals::StandaloneKeywordAction::Abandon,
        mtg_data::KeywordAction::Activate => terminals::StandaloneKeywordAction::Activate,
        mtg_data::KeywordAction::Amass => terminals::StandaloneKeywordAction::Amass,
        mtg_data::KeywordAction::Assemble => terminals::StandaloneKeywordAction::Assemble,
        mtg_data::KeywordAction::Attach => terminals::StandaloneKeywordAction::Attach,
        mtg_data::KeywordAction::Behold => terminals::StandaloneKeywordAction::Behold,
        mtg_data::KeywordAction::Bolster => terminals::StandaloneKeywordAction::Bolster,
        mtg_data::KeywordAction::Cast => terminals::StandaloneKeywordAction::Cast,
        mtg_data::KeywordAction::Clash => terminals::StandaloneKeywordAction::Clash,
        mtg_data::KeywordAction::Cloak => terminals::StandaloneKeywordAction::Cloak,
        mtg_data::KeywordAction::CollectEvidence => terminals::StandaloneKeywordAction::CollectEvidence,
        mtg_data::KeywordAction::Conjure => terminals::StandaloneKeywordAction::Conjure,
        mtg_data::KeywordAction::Connive => terminals::StandaloneKeywordAction::Connive,
        mtg_data::KeywordAction::Convert => terminals::StandaloneKeywordAction::Convert,
        mtg_data::KeywordAction::Counter => terminals::StandaloneKeywordAction::Counter,
        mtg_data::KeywordAction::Create => terminals::StandaloneKeywordAction::Create,
        mtg_data::KeywordAction::Destroy => terminals::StandaloneKeywordAction::Destroy,
        mtg_data::KeywordAction::Detain => terminals::StandaloneKeywordAction::Detain,
        mtg_data::KeywordAction::Discard => terminals::StandaloneKeywordAction::Discard,
        mtg_data::KeywordAction::Discover => terminals::StandaloneKeywordAction::Discover,
        mtg_data::KeywordAction::Double => terminals::StandaloneKeywordAction::Double,
        mtg_data::KeywordAction::Endure => terminals::StandaloneKeywordAction::Endure,
        mtg_data::KeywordAction::Exchange => terminals::StandaloneKeywordAction::Exchange,
        mtg_data::KeywordAction::Exert => terminals::StandaloneKeywordAction::Exert,
        mtg_data::KeywordAction::Exile => terminals::StandaloneKeywordAction::Exile,
        mtg_data::KeywordAction::Explore => terminals::StandaloneKeywordAction::Explore,
        mtg_data::KeywordAction::Fateseal => terminals::StandaloneKeywordAction::Fateseal,
        mtg_data::KeywordAction::Fight => terminals::StandaloneKeywordAction::Fight,
        mtg_data::KeywordAction::Food => terminals::StandaloneKeywordAction::Food,
        mtg_data::KeywordAction::Forage => terminals::StandaloneKeywordAction::Forage,
        mtg_data::KeywordAction::Goad => terminals::StandaloneKeywordAction::Goad,
        mtg_data::KeywordAction::Heist => terminals::StandaloneKeywordAction::Heist,
        mtg_data::KeywordAction::Incubate => terminals::StandaloneKeywordAction::Incubate,
        mtg_data::KeywordAction::Investigate => terminals::StandaloneKeywordAction::Investigate,
        mtg_data::KeywordAction::Learn => terminals::StandaloneKeywordAction::Learn,
        mtg_data::KeywordAction::Manifest => terminals::StandaloneKeywordAction::Manifest,
        mtg_data::KeywordAction::ManifestDread => terminals::StandaloneKeywordAction::ManifestDread,
        mtg_data::KeywordAction::Meld => terminals::StandaloneKeywordAction::Meld,
        mtg_data::KeywordAction::Mill => terminals::StandaloneKeywordAction::Mill,
        mtg_data::KeywordAction::Monstrosity => terminals::StandaloneKeywordAction::Monstrosity,
        mtg_data::KeywordAction::OpenAnAttraction => terminals::StandaloneKeywordAction::OpenAnAttraction,
        mtg_data::KeywordAction::Planeswalk => terminals::StandaloneKeywordAction::Planeswalk,
        mtg_data::KeywordAction::Play => terminals::StandaloneKeywordAction::Play,
        mtg_data::KeywordAction::Plot => terminals::StandaloneKeywordAction::Plot,
        mtg_data::KeywordAction::Populate => terminals::StandaloneKeywordAction::Populate,
        mtg_data::KeywordAction::Proliferate => terminals::StandaloneKeywordAction::Proliferate,
        mtg_data::KeywordAction::Regenerate => terminals::StandaloneKeywordAction::Regenerate,
        mtg_data::KeywordAction::Reveal => terminals::StandaloneKeywordAction::Reveal,
        mtg_data::KeywordAction::RoleToken => terminals::StandaloneKeywordAction::RoleToken,
        mtg_data::KeywordAction::RollToVisitYourAttractions => terminals::StandaloneKeywordAction::RollToVisitYourAttractions,
        mtg_data::KeywordAction::Sacrifice => terminals::StandaloneKeywordAction::Sacrifice,
        mtg_data::KeywordAction::Seek => terminals::StandaloneKeywordAction::Seek,
        mtg_data::KeywordAction::SetInMotion => terminals::StandaloneKeywordAction::SetInMotion,
        mtg_data::KeywordAction::Shuffle => terminals::StandaloneKeywordAction::Shuffle,
        mtg_data::KeywordAction::Support => terminals::StandaloneKeywordAction::Support,
        mtg_data::KeywordAction::Suspect => terminals::StandaloneKeywordAction::Suspect,
        mtg_data::KeywordAction::Tap => terminals::StandaloneKeywordAction::Tap,
        mtg_data::KeywordAction::TimeTravel => terminals::StandaloneKeywordAction::TimeTravel,
        mtg_data::KeywordAction::Transform => terminals::StandaloneKeywordAction::Transform,
        mtg_data::KeywordAction::Untap => terminals::StandaloneKeywordAction::Untap,
        mtg_data::KeywordAction::VentureIntoTheDungeon => terminals::StandaloneKeywordAction::VentureIntoTheDungeon,
        mtg_data::KeywordAction::Vote => terminals::StandaloneKeywordAction::Vote,
        _ => return Err("provided keyword is not a valid keyword ability on its own"),
    };
    Ok(KeywordAction {
        keyword: ExpandedKeywordAction::Standalone(StandaloneKeywordAction {
            keyword_action,
            #[cfg(feature = "spanned_tree")]
            span: keyword.span,
        }),
        ability: SpellAbility {
            effects: crate::utils::HeapArrayVec::new(),
            #[cfg(feature = "spanned_tree")]
            span: keyword.span,
        },
        #[cfg(feature = "spanned_tree")]
        span: keyword.span,
    })
}
