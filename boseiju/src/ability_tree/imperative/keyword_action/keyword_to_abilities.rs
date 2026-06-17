use crate::ability_tree::ability::spell::SpellAbility;
use crate::ability_tree::imperative::KeywordAction;
use crate::ability_tree::imperative::keyword_action::ExpandedKeywordAction;
use crate::ability_tree::imperative::keyword_action::StandaloneKeywordAction;
use crate::ability_tree::terminals;
use crate::lexer::tokens::intermediates;

pub fn keyword_action_to_abilities(keyword: intermediates::KeywordAction) -> Result<KeywordAction, &'static str> {
    let keyword_action = match keyword.keyword_action {
        mtg_data::KeywordAction::Forage => terminals::StandaloneKeywordAction::Forage,
        mtg_data::KeywordAction::Investigate => terminals::StandaloneKeywordAction::Investigate,
        mtg_data::KeywordAction::Learn => terminals::StandaloneKeywordAction::Learn,
        mtg_data::KeywordAction::ManifestDread => terminals::StandaloneKeywordAction::ManifestDread,
        mtg_data::KeywordAction::OpenAnAttraction => terminals::StandaloneKeywordAction::OpenAnAttraction,
        mtg_data::KeywordAction::Planeswalk => terminals::StandaloneKeywordAction::Planeswalk,
        mtg_data::KeywordAction::Populate => terminals::StandaloneKeywordAction::Populate,
        mtg_data::KeywordAction::Proliferate => terminals::StandaloneKeywordAction::Proliferate,
        mtg_data::KeywordAction::RollToVisitYourAttractions => terminals::StandaloneKeywordAction::RollToVisitYourAttractions,
        mtg_data::KeywordAction::SetInMotion => terminals::StandaloneKeywordAction::SetInMotion,
        mtg_data::KeywordAction::Shuffle => terminals::StandaloneKeywordAction::Shuffle,
        mtg_data::KeywordAction::TimeTravel => terminals::StandaloneKeywordAction::TimeTravel,
        mtg_data::KeywordAction::VentureIntoTheDungeon => terminals::StandaloneKeywordAction::VentureIntoTheDungeon,
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
