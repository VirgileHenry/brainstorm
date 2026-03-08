use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// A condition on the number of times an ability has resolved in a turn.
///
/// Fixme: maybe better for the AI to see a number ?
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConditionNumberOfResolutions {
    FirstTimeThisAbilityResolves {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    SecondTimeThisAbilityResolves {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ThirdTimeThisAbilityResolves {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    FourthTimeThisAbilityResolves {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl crate::ability_tree::AbilityTreeNode for ConditionNumberOfResolutions {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::NumberOfResolutionsIdMarker.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::NumberOfResolutions(self.clone()).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "number of time this ability resolved:")?;
        out.push_final_branch()?;
        match self {
            Self::FirstTimeThisAbilityResolves { .. } => write!(out, "first")?,
            Self::SecondTimeThisAbilityResolves { .. } => write!(out, "second")?,
            Self::ThirdTimeThisAbilityResolves { .. } => write!(out, "third")?,
            Self::FourthTimeThisAbilityResolves { .. } => write!(out, "fourth")?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "condition: number of resolution"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::FirstTimeThisAbilityResolves { span } => *span,
            Self::SecondTimeThisAbilityResolves { span } => *span,
            Self::ThirdTimeThisAbilityResolves { span } => *span,
            Self::FourthTimeThisAbilityResolves { span } => *span,
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ConditionNumberOfResolutions {
    fn dummy_init() -> Self {
        Self::FirstTimeThisAbilityResolves {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[cfg(feature = "lexer")]
impl crate::lexer::IntoToken for ConditionNumberOfResolutions {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "this is the first time this ability has resolved this turn" => Some(Self::SecondTimeThisAbilityResolves {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "this is the second time this ability has resolved this turn" => Some(Self::SecondTimeThisAbilityResolves {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "this is the third time this ability has resolved this turn" => Some(Self::SecondTimeThisAbilityResolves {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "this is the fourth time this ability has resolved this turn" => Some(Self::SecondTimeThisAbilityResolves {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
