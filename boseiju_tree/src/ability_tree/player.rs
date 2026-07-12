use crate::Node;
use crate::MAX_CHILDREN_PER_NODE;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerSpecifier {
    All {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    AnOpponent {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Any {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    EachOpponent {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ObjectController(ObjectController),
    ObjectOwner(ObjectOwner),
    PerviouslyMentionnedPlayer {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TargetOpponent {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ToYourLeft {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ToYourRight {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    You {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    /* Fixme: previously mentionned opponent (they) */
}

impl Node for PlayerSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::PlayerSpecifierIdMarker.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::ObjectController(child) => children.push(child as &dyn Node),
            Self::ObjectOwner(child) => children.push(child as &dyn Node),
            other => children.push(crate::dummy_terminal::TreeNodeDummyTerminal::new(
                crate::NodeKind::PlayerSpecifier(other.clone()).id(),
            ) as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
            Self::ObjectOwner(child) => {
                write!(out, "object owner's:")?;
                out.push_final_branch()?;
                child.display(out)?;
                out.pop_branch();
            }
            Self::TargetOpponent { .. } => write!(out, "target opponent")?,
            Self::PerviouslyMentionnedPlayer { .. } => write!(out, "target opponent")?,
            Self::ToYourLeft { .. } => write!(out, "the player to your left")?,
            Self::ToYourRight { .. } => write!(out, "the player to your right")?,
            Self::You { .. } => write!(out, "you")?,
        }
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "player specifier"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PlayerSpecifier {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::All { span } => *span,
            Self::AnOpponent { span } => *span,
            Self::Any { span } => *span,
            Self::EachOpponent { span } => *span,
            Self::ObjectController(child) => child.span,
            Self::ObjectOwner(child) => child.span,
            Self::TargetOpponent { span } => *span,
            Self::PerviouslyMentionnedPlayer { span } => *span,
            Self::ToYourLeft { span } => *span,
            Self::ToYourRight { span } => *span,
            Self::You { span } => *span,
        }
    }
}

impl Default for PlayerSpecifier {
    fn default() -> Self {
        Self::You {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectController {
    pub object: Box<crate::ability_tree::object::Permanent>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for ObjectController {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::PlayerSpecifierObjectController.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(self.object.as_ref() as &dyn Node);
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
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
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for ObjectController {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for ObjectController {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "PlayerSpecifierObjectController"
    }
}

impl Default for ObjectController {
    fn default() -> Self {
        Self {
            object: Box::new(Default::default()),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectOwner {
    pub object: Box<crate::ability_tree::object::Card>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Node for ObjectOwner {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::NodeKind::PlayerSpecifierObjectOwner.id()
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
        "PlayerSpecifierObjectOwner"
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
