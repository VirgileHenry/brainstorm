/// Special non terminal token for the clause that is seen quite a bit,
/// "in addition to paying its other cost".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InAdditionToPayingItsOtherCost {
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl InAdditionToPayingItsOtherCost {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "in addition to paying its other costs" => Some(Self {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}

impl idris::Idris for InAdditionToPayingItsOtherCost {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "InAdditionToPayingItsOtherCost"
    }
}
