#[derive(Debug, Clone)]
pub struct ParserRuleDeclarationLocation {
    pub file: &'static str,
    pub line: u32,
}

impl std::fmt::Display for ParserRuleDeclarationLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.file, self.line)
    }
}
