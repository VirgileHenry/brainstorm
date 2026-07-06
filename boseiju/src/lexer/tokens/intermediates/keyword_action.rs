use crate::lexer::IntoToken;

/// Wrapper around the keyword action.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct KeywordAction {
    pub keyword_action: mtg_data::KeywordAction,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

#[cfg(feature = "lexer")]
impl IntoToken for KeywordAction {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        /* Not a fan of "from_str_singular_or_plural" for a places where "s" are tense forms */
        if let Some(keyword_action) = crate::utils::from_str_singular_or_plural(span.text) {
            Some(Self {
                keyword_action,
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })
        } else {
            /* Some special cases for past tenses, etc. */
            match span.text {
                "activated" | "activating" => Some(Self {
                    keyword_action: mtg_data::KeywordAction::Activate,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "attaches" => Some(Self {
                    keyword_action: mtg_data::KeywordAction::Attach,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "casting" => Some(Self {
                    keyword_action: mtg_data::KeywordAction::Cast,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "created" => Some(Self {
                    keyword_action: mtg_data::KeywordAction::Create,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "destroyed" => Some(Self {
                    keyword_action: mtg_data::KeywordAction::Destroy,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "discarded" | "discarding" => Some(Self {
                    keyword_action: mtg_data::KeywordAction::Discard,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "discovered" => Some(Self {
                    keyword_action: mtg_data::KeywordAction::Discover,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "exerted" => Some(Self {
                    keyword_action: mtg_data::KeywordAction::Exert,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "investigated" => Some(Self {
                    keyword_action: mtg_data::KeywordAction::Investigate,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "foraging" => Some(Self {
                    keyword_action: mtg_data::KeywordAction::Forage,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "fought" => Some(Self {
                    keyword_action: mtg_data::KeywordAction::Fight,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "manifested" => Some(Self {
                    keyword_action: mtg_data::KeywordAction::Manifest,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "manifests dread" | "manifested dread" => Some(Self {
                    keyword_action: mtg_data::KeywordAction::ManifestDread,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "milled" => Some(Self {
                    keyword_action: mtg_data::KeywordAction::Mill,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "played" => Some(Self {
                    keyword_action: mtg_data::KeywordAction::Play,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "plotting" => Some(Self {
                    keyword_action: mtg_data::KeywordAction::Plot,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "regenerated" => Some(Self {
                    keyword_action: mtg_data::KeywordAction::Regenerate,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "sacrificed" | "sacrificing" => Some(Self {
                    keyword_action: mtg_data::KeywordAction::Search,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "scrying" | "scries" => Some(Self {
                    keyword_action: mtg_data::KeywordAction::Scry,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "searches" | "searched" | "searching" => Some(Self {
                    keyword_action: mtg_data::KeywordAction::Search,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "surveilled" => Some(Self {
                    keyword_action: mtg_data::KeywordAction::Surveil,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "tapping" => Some(Self {
                    keyword_action: mtg_data::KeywordAction::Tap,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "voted" | "voting" => Some(Self {
                    keyword_action: mtg_data::KeywordAction::Vote,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
                "waterbending" => Some(Self {
                    keyword_action: mtg_data::KeywordAction::Waterbend,
                    #[cfg(feature = "spanned_tree")]
                    span: span.into(),
                }),
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
