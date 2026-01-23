pub mod activated;
pub mod keyword;
pub mod saga_chapter;
pub mod spell;
pub mod statik;
pub mod triggered;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum Ability {
    Activated(activated::ActivatedAbility),
    Keyword(keyword::KeywordAbility),
    SagaChapter(saga_chapter::SagaChapter),
    Spell(spell::SpellAbility),
    Static(statik::StaticAbility),
    Triggered(triggered::TriggeredAbility),
}

impl crate::ability_tree::AbilityTreeImpl for Ability {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        match self {
            Ability::Activated(activated) => activated.display(out)?,
            Ability::Keyword(keyword) => keyword.display(out)?,
            Ability::SagaChapter(chapter) => chapter.display(out)?,
            Ability::Spell(spell) => spell.display(out)?,
            Ability::Static(statik) => statik.display(out)?,
            Ability::Triggered(triggered) => triggered.display(out)?,
        }
        Ok(())
    }
}
