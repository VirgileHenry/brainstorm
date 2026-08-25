use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;

type Object = crate::ability_tree::object::Card<crate::ability_tree::quantifier::ActiveQuantifier>;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectOwner {
    pub object: Box<Object>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for ObjectOwner {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::PlayerReferenceObjectOwner
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(self.object.as_ref() as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "object's owner:")?;
        out.push_final_branch()?;
        self.object.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "object's owner"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ObjectOwner {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for ObjectOwner {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl Default for ObjectOwner {
    fn default() -> Self {
        Self {
            object: Box::new(Default::default()),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
