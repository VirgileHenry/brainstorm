use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachKeywordAction {
    pub object: crate::ability_tree::object::Permanent, /* Fixme: equipment, aura or fortification only */
    pub to: crate::ability_tree::object::Permanent,     /* Fixme: permanent or player */
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl crate::Node for AttachKeywordAction {
    fn node_id(&self) -> usize {
        use crate::node_kind::KeywordActionNodeKind;
        use idris::Idris;

        crate::NodeKind::KeywordAction(KeywordActionNodeKind::Attach).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.object as &dyn Node);
        children.push(&self.to as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "attach:")?;
        out.push_inter_branch()?;
        write!(out, "object:")?;
        out.push_final_branch()?;
        self.object.display(out)?;
        out.pop_branch();
        out.next_final_branch()?;
        write!(out, "to:")?;
        out.push_final_branch()?;
        self.to.display(out)?;
        out.pop_branch();
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "attach keyword ability"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for AttachKeywordAction {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for AttachKeywordAction {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "attach"
    }
}

impl Default for AttachKeywordAction {
    fn default() -> Self {
        Self {
            object: Default::default(),
            to: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

pub fn ability(
    _object: &crate::ability_tree::object::Permanent,
    _to: &crate::ability_tree::object::Permanent,
    #[cfg(feature = "spanned_tree")] span: boseiju_span::Span,
) -> crate::ability_tree::ability::spell::SpellAbility {
    /* Fixme: unimplemented */
    crate::ability_tree::ability::spell::SpellAbility {
        effects: crate::HeapArrayVec::new(),
        #[cfg(feature = "spanned_tree")]
        span,
    }
}
