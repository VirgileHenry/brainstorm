use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

/// The  creature has subtype specifiers.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordAbilitySpecifier {
    pub keyword_ability: Box<crate::ability_tree::ability::KeywordAbility>, /* Fixme: overkill ? */
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for KeywordAbilitySpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::KeywordAbilitySpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(self.keyword_ability.as_ref() as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for KeywordAbilitySpecifier {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl Default for KeywordAbilitySpecifier {
    fn default() -> Self {
        Self {
            keyword_ability: Box::new(Default::default()),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
