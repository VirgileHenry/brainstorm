use boseiju_lexer::intermediate;
use boseiju_lexer::terminal;
use boseiju_tree::ability_tree::ability::spell::SpellAbility;
use boseiju_tree::ability_tree::imperative::ExpandedKeywordAction;
use boseiju_tree::ability_tree::imperative::KeywordAction;
use boseiju_tree::ability_tree::imperative::StandaloneKeywordAction;

pub fn keyword_action_to_abilities(keyword: intermediate::KeywordAction) -> Result<KeywordAction, &'static str> {
    let keyword_action = match keyword.keyword_action {
        mtg_data::KeywordAction::Forage => terminal::StandaloneKeywordAction::Forage,
        mtg_data::KeywordAction::Investigate => terminal::StandaloneKeywordAction::Investigate,
        mtg_data::KeywordAction::Learn => terminal::StandaloneKeywordAction::Learn,
        mtg_data::KeywordAction::ManifestDread => terminal::StandaloneKeywordAction::ManifestDread,
        mtg_data::KeywordAction::OpenAnAttraction => terminal::StandaloneKeywordAction::OpenAnAttraction,
        mtg_data::KeywordAction::Planeswalk => terminal::StandaloneKeywordAction::Planeswalk,
        mtg_data::KeywordAction::Populate => terminal::StandaloneKeywordAction::Populate,
        mtg_data::KeywordAction::Proliferate => terminal::StandaloneKeywordAction::Proliferate,
        mtg_data::KeywordAction::RollToVisitYourAttractions => terminal::StandaloneKeywordAction::RollToVisitYourAttractions,
        mtg_data::KeywordAction::SetInMotion => terminal::StandaloneKeywordAction::SetInMotion,
        mtg_data::KeywordAction::Shuffle => terminal::StandaloneKeywordAction::Shuffle,
        mtg_data::KeywordAction::TimeTravel => terminal::StandaloneKeywordAction::TimeTravel,
        mtg_data::KeywordAction::VentureIntoTheDungeon => terminal::StandaloneKeywordAction::VentureIntoTheDungeon,
        _ => return Err("provided keyword is not a valid keyword ability on its own"),
    };
    Ok(KeywordAction {
        keyword: ExpandedKeywordAction::Standalone(StandaloneKeywordAction {
            keyword_action,
            #[cfg(feature = "spanned_tree")]
            span: keyword.span,
        }),
        ability: SpellAbility {
            effects: boseiju_tree::HeapArrayVec::new(),
            #[cfg(feature = "spanned_tree")]
            span: keyword.span,
        },
        #[cfg(feature = "spanned_tree")]
        span: keyword.span,
    })
}
