use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// The  creature has subtype specifiers.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatureKeywordAbilitySpecifier {
    pub keyword_ability: Box<crate::ability_tree::ability::KeywordAbility>, /* Fixme: overkill ? */
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::ability_tree::AbilityTreeNode for CreatureKeywordAbilitySpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::CreatureKeywordAbilitySpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(self.keyword_ability.as_ref() as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "creature keyword ability specifier:")?;
        out.push_final_branch()?;
        self.keyword_ability.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "creature keyword ability specifier"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for CreatureKeywordAbilitySpecifier {
    fn dummy_init() -> Self {
        Self {
            keyword_ability: Box::new(crate::utils::dummy()),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
