use crate::tree::ability::spell::SpellAbility;
use crate::tree::imperative::KeywordAction;
use crate::tree::imperative::keyword_action::ExpandedKeywordAction;
use crate::tree::imperative::keyword_action::StandaloneKeywordAction;
use boseiju_lexer::intermediate;
use boseiju_lexer::terminal;

pub fn keyword_action_to_abilities(keyword: intermediates::TensedKeywordAction) -> Result<KeywordAction, &'static str> {
    /* Fixme: let's think about this */

    let keyword_action = match keyword.token().keyword_action {
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

    use boseiju_span::Spanned;
    Ok(KeywordAction {
        keyword: ExpandedKeywordAction::Standalone(StandaloneKeywordAction {
            keyword_action,
            #[cfg(feature = "spanned_tree")]
            span: keyword.span(),
        }),
        ability: SpellAbility {
            effects: crate::HeapArrayVec::new(),
            #[cfg(feature = "spanned_tree")]
            span: keyword.span(),
        },
        #[cfg(feature = "spanned_tree")]
        span: keyword.span(),
    })
}
