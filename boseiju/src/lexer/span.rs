#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span<'src> {
    pub start: usize,
    pub length: usize,
    pub text: &'src str,
}
