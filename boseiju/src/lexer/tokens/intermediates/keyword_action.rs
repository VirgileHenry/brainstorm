use crate::lexer::IntoToken;
use crate::lexer::tokens::tensed::Tensed;

/// Wrapper around the keyword action.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct KeywordAction {
    pub keyword_action: mtg_data::KeywordAction,
    #[cfg(feature = "spanned_tree")]
    span: crate::ability_tree::span::TreeSpan,
}

#[cfg(feature = "spanned_tree")]
impl crate::ability_tree::span::Spanned for KeywordAction {
    fn span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

pub type TensedKeywordAction = Tensed<KeywordAction>;

#[cfg(feature = "lexer")]
impl IntoToken for TensedKeywordAction {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        use std::str::FromStr;

        if let Ok(keyword_action) = mtg_data::KeywordAction::from_str(span.text) {
            Some(Tensed::base_form(KeywordAction {
                keyword_action,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }))
        } else {
            /* Some special cases for past tenses, etc. */
            match span.text {
                "activated" => Some(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Activate,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "activates" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Activate,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "activating" => Some(Tensed::present_participle(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Activate,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "adapts" => Some(Tensed::present_participle(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Adapt,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "assembles" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Assemble,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "attaches" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Attach,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "blights" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Cast,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "casts" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Cast,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "casting" => Some(Tensed::present_participle(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Cast,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "cloaks" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Cloak,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "connives" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Connive,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "creates" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Create,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "created" => Some(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Create,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "destroys" => Some(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Destroy,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "destroyed" => Some(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Destroy,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "discards" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Discard,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "discarded" => Some(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Discard,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "discarding" => Some(Tensed::present_participle(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Discard,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "discovers" => Some(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Discover,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "discovered" => Some(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Discover,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "endures" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Endure,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "exerted" => Some(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Exert,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "exiles" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Exert,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "explores" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Explore,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "incubates" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Incubate,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "investigated" => Some(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Investigate,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "investigates" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Investigate,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "foraging" => Some(Tensed::present_participle(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Forage,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "fought" => Some(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Fight,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "manifested" => Some(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Manifest,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "manifests" => Some(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Manifest,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "manifests dread" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::ManifestDread,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "manifested dread" => Some(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::ManifestDread,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "mills" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Mill,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "milled" => Some(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Mill,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "played" => Some(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Play,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "plays" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Play,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "plotting" => Some(Tensed::present_participle(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Plot,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "reveals" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Regenerate,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "regenerated" => Some(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Regenerate,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "regenerates" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Regenerate,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "sacrifices" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Search,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "sacrificed" => Some(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Search,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "sacrificing" => Some(Tensed::present_participle(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Search,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "scries" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Scry,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "scrying" => Some(Tensed::present_participle(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Scry,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "searches" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Search,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "searched" => Some(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Search,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "searching" => Some(Tensed::present_participle(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Search,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "surveils" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Surveil,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "surveilled" => Some(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Surveil,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "taps" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Tap,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "tapping" => Some(Tensed::present_participle(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Tap,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "transforms" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Transform,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "untaps" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Untap,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "voted" => Some(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Vote,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "votes" => Some(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Vote,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "voting" => Some(Tensed::present_participle(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Vote,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "waterbending" => Some(Tensed::present_participle(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Waterbend,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                _ => None,
            }
        }
    }
}

impl idris::Idris for KeywordAction {
    const COUNT: usize = mtg_data::KeywordAction::COUNT;
    fn id(&self) -> usize {
        self.keyword_action.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        mtg_data::KeywordAction::name_from_id(id)
    }
}
