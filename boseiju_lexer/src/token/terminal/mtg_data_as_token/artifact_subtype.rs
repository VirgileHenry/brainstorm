/// Wrapper around the artifact subtype.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ArtifactSubtype {
    pub artifact_subtype: mtg_data::ArtifactType,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl ArtifactSubtype {
    pub fn all() -> impl Iterator<Item = Self> {
        mtg_data::ArtifactType::all().map(|artifact_subtype| ArtifactSubtype {
            artifact_subtype,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        })
    }
}

impl Default for ArtifactSubtype {
    fn default() -> Self {
        Self {
            artifact_subtype: mtg_data::ArtifactType::Clue,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ArtifactSubtype {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for ArtifactSubtype {
    const COUNT: usize = mtg_data::ArtifactType::COUNT;
    fn id(&self) -> usize {
        self.artifact_subtype.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        mtg_data::ArtifactType::name_from_id(id)
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for ArtifactSubtype {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        Ok(Self {
            artifact_subtype: crate::parsing::from_str_singular_or_plural(&span.text)?,
            #[cfg(feature = "spanned_tree")]
            span: span.into(),
        })
    }
}

impl std::fmt::Display for ArtifactSubtype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.artifact_subtype.fmt(f)
    }
}
