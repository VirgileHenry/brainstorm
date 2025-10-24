#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SagaChapter {
    pub chapter: crate::ability_tree::terminals::SagaChapterNumber,
    pub effect: crate::ability_tree::statement::Statement,
}

impl crate::ability_tree::AbilityTreeImpl for SagaChapter {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        self.effect.display(out)
    }
}
