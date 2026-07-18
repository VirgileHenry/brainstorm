use crate::token::tensed::Tensed;

/// Wrapper around the keyword action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct KeywordAction {
    pub keyword_action: mtg_data::KeywordAction,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for KeywordAction {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

pub type TensedKeywordAction = Tensed<KeywordAction>;

impl<'src> TryFrom<&crate::LexerSpan<'src>> for TensedKeywordAction {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        use std::str::FromStr;

        if let Ok(keyword_action) = mtg_data::KeywordAction::from_str(span.text) {
            Ok(Tensed::base_form(KeywordAction {
                keyword_action,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }))
        } else {
            /* Some special cases for past tenses, etc. */
            match span.text {
                "activated" => Ok(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Activate,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "activates" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Activate,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "activating" => Ok(Tensed::present_participle(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Activate,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "adapts" => Ok(Tensed::present_participle(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Adapt,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "assembles" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Assemble,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "attaches" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Attach,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "blights" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Cast,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "casts" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Cast,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "casting" => Ok(Tensed::present_participle(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Cast,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "cloaks" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Cloak,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "connives" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Connive,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "creates" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Create,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "created" => Ok(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Create,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "destroys" => Ok(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Destroy,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "destroyed" => Ok(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Destroy,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "discards" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Discard,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "discarded" => Ok(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Discard,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "discarding" => Ok(Tensed::present_participle(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Discard,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "discovers" => Ok(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Discover,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "discovered" => Ok(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Discover,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "endures" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Endure,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "exerted" => Ok(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Exert,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "exiles" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Exert,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "explores" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Explore,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "incubates" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Incubate,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "investigated" => Ok(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Investigate,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "investigates" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Investigate,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "foraging" => Ok(Tensed::present_participle(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Forage,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "fought" => Ok(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Fight,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "manifested" => Ok(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Manifest,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "manifests" => Ok(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Manifest,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "manifests dread" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::ManifestDread,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "manifested dread" => Ok(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::ManifestDread,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "mills" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Mill,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "milled" => Ok(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Mill,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "played" => Ok(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Play,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "plays" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Play,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "plotting" => Ok(Tensed::present_participle(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Plot,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "reveals" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Regenerate,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "regenerated" => Ok(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Regenerate,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "regenerates" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Regenerate,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "sacrifices" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Search,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "sacrificed" => Ok(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Search,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "sacrificing" => Ok(Tensed::present_participle(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Search,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "scries" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Scry,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "scrying" => Ok(Tensed::present_participle(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Scry,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "searches" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Search,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "searched" => Ok(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Search,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "searching" => Ok(Tensed::present_participle(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Search,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "surveils" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Surveil,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "surveilled" => Ok(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Surveil,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "taps" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Tap,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "tapping" => Ok(Tensed::present_participle(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Tap,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "transforms" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Transform,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "untaps" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Untap,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "voted" => Ok(Tensed::simple_past(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Vote,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "votes" => Ok(Tensed::third_person_singular_present(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Vote,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "voting" => Ok(Tensed::present_participle(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Vote,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                "waterbending" => Ok(Tensed::present_participle(KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Waterbend,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                })),
                _ => Err(()),
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
