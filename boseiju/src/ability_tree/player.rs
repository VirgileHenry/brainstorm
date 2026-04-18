use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerSpecifier {
    All {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    AnOpponent {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Any {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    EachOpponent {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ObjectController(PlayerSpecifierObjectController),
    TargetOpponent {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ObjectOwner(PlayerSpecifierObjectOwner),
    ToYourLeft {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ToYourRight {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    You {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    /* Fixme: previously mentionned opponent (they) */
}

impl AbilityTreeNode for PlayerSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PlayerSpecifierIdMarker.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::ObjectController(child) => children.push(child as &dyn AbilityTreeNode),
            other => children.push(crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(
                crate::ability_tree::NodeKind::PlayerSpecifier(other.clone()).id(),
            ) as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            Self::All { .. } => write!(out, "all players")?,
            Self::AnOpponent { .. } => write!(out, "an opponent")?,
            Self::Any { .. } => write!(out, "a player")?,
            Self::EachOpponent { .. } => write!(out, "each opponent")?,
            Self::ObjectController(child) => {
                write!(out, "object controller's:")?;
                out.push_final_branch()?;
                child.display(out)?;
                out.pop_branch();
            }
            Self::TargetOpponent { .. } => write!(out, "target opponent")?,
            Self::ObjectOwner(child) => {
                write!(out, "object owner's:")?;
                out.push_final_branch()?;
                child.display(out)?;
                out.pop_branch();
            }
            Self::ToYourLeft { .. } => write!(out, "the player to your left")?,
            Self::ToYourRight { .. } => write!(out, "the player to your right")?,
            Self::You { .. } => write!(out, "you")?,
        }
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "player specifier"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::All { span } => *span,
            Self::AnOpponent { span } => *span,
            Self::Any { span } => *span,
            Self::EachOpponent { span } => *span,
            Self::ObjectController(child) => child.span,
            Self::TargetOpponent { span } => *span,
            Self::ObjectOwner(child) => child.span,
            Self::ToYourLeft { span } => *span,
            Self::ToYourRight { span } => *span,
            Self::You { span } => *span,
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PlayerSpecifier {
    fn dummy_init() -> Self {
        Self::You {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerSpecifierObjectController {
    pub object: Box<crate::ability_tree::object::ObjectReference>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for PlayerSpecifierObjectController {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PlayerSpecifierObjectController.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(self.object.as_ref() as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "object's controller:")?;
        out.push_final_branch()?;
        self.object.display(out)?;
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "object's controller"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

impl idris::Idris for PlayerSpecifierObjectController {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "PlayerSpecifierObjectController"
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PlayerSpecifierObjectController {
    fn dummy_init() -> Self {
        Self {
            object: Box::new(crate::utils::dummy()),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerSpecifierObjectOwner {
    pub object: Box<crate::ability_tree::object::ObjectReference>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl AbilityTreeNode for PlayerSpecifierObjectOwner {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::PlayerSpecifierObjectOwner.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(self.object.as_ref() as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
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

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        self.span
    }
}

impl idris::Idris for PlayerSpecifierObjectOwner {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "PlayerSpecifierObjectOwner"
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for PlayerSpecifierObjectOwner {
    fn dummy_init() -> Self {
        Self {
            object: Box::new(crate::utils::dummy()),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
