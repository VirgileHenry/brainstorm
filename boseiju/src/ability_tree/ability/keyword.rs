#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum KeywordAbility {
    SingleKeyword(mtg_data::KeywordAbility),
    Ward(crate::ability_tree::cost::Cost),
}

impl crate::ability_tree::AbilityTreeImpl for KeywordAbility {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            KeywordAbility::SingleKeyword(keyword) => write!(out, "Keyword: {}", keyword)?,
            KeywordAbility::Ward(cost) => {
                write!(out, "Keyword: Ward, ward cost: ")?;
                cost.display(out)?;
            }
        }
        Ok(())
    }
}
