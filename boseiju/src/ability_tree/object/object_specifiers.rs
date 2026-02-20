use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum ObjectSpecifiers {
    Single(ObjectSpecifier),
    And(SpecifierAndList),
    Or(SpecifierOrList),
    OrOfAnd(SpecifierOrOfAndList),
}

impl crate::ability_tree::AbilityTreeNode for ObjectSpecifiers {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ObjectSpecifiers.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Single(child) => children.push(child as &dyn AbilityTreeNode),
            Self::And(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Or(child) => children.push(child as &dyn AbilityTreeNode),
            Self::OrOfAnd(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "object specifiers:")?;
        out.push_final_branch()?;
        match self {
            Self::Single(child) => child.display(out)?,
            Self::And(child) => child.display(out)?,
            Self::Or(child) => child.display(out)?,
            Self::OrOfAnd(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}

/// A list of object specifiers, grouped with a logical AND.
///
/// It means that for an object to match these specifiers,
/// it must match all of them.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct SpecifierAndList {
    specifiers: arrayvec::ArrayVec<ObjectSpecifier, MAX_CHILDREN_PER_NODE>,
}

impl AbilityTreeNode for SpecifierAndList {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::SpecifierAndList.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for specifier in self.specifiers.iter() {
            children.push(specifier as &dyn AbilityTreeNode);
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "specifier and list:")?;
        for (i, specifier) in self.specifiers.iter().enumerate() {
            if i == self.specifiers.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            specifier.display(out)?;
            out.pop_branch();
        }
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for SpecifierAndList {
    fn dummy_init() -> Self {
        Self {
            specifiers: arrayvec::ArrayVec::new_const(),
        }
    }
}

/// A list of object specifiers, grouped with a logical OR.
///
/// It means that for an object to match these specifiers,
/// it must match any one specifier in the list.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct SpecifierOrList {
    specifiers: arrayvec::ArrayVec<ObjectSpecifier, MAX_CHILDREN_PER_NODE>,
}

impl SpecifierOrList {
    pub fn add_factor_specifier(&self, factor_specifier: ObjectSpecifier) -> SpecifierOrOfAndList {
        let mut or_specifiers = arrayvec::ArrayVec::new_const();
        for prev_specifier in self.specifiers.iter() {
            let mut and_specifiers = arrayvec::ArrayVec::new_const();
            and_specifiers.push(prev_specifier.clone());
            and_specifiers.push(factor_specifier.clone());
            or_specifiers.push(and_specifiers);
        }
        SpecifierOrOfAndList {
            specifiers: or_specifiers,
        }
    }
}

impl AbilityTreeNode for SpecifierOrList {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::SpecifierOrList.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        for specifier in self.specifiers.iter() {
            children.push(specifier as &dyn AbilityTreeNode);
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "specifier or list:")?;
        for (i, specifier) in self.specifiers.iter().enumerate() {
            if i == self.specifiers.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            specifier.display(out)?;
            out.pop_branch();
        }
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for SpecifierOrList {
    fn dummy_init() -> Self {
        Self {
            specifiers: arrayvec::ArrayVec::new_const(),
        }
    }
}

const OR_OF_AND_LIST_OUTER_ARRAY_LENGTH: usize = 3;
const OR_OF_AND_LIST_INNER_ARRAY_LENGTH: usize = MAX_CHILDREN_PER_NODE / OR_OF_AND_LIST_OUTER_ARRAY_LENGTH;

/// A list of list of object specifiers, where the outer list is grouped
/// with a logical OR, and the inner list is grouped with a logical AND.
///
/// It means that for an object to match these specifiers,
/// it must match all the specifiers of any one of the sublists.
///
/// This is required since sometimes, there are specifiers that applies to
/// OR lists: "basic plains or forest" means "(basic AND plains) OR (basic AND forest)".
///
/// This structure represent properly this case.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct SpecifierOrOfAndList {
    specifiers: arrayvec::ArrayVec<
        arrayvec::ArrayVec<ObjectSpecifier, OR_OF_AND_LIST_INNER_ARRAY_LENGTH>,
        OR_OF_AND_LIST_OUTER_ARRAY_LENGTH,
    >,
}

impl AbilityTreeNode for SpecifierOrOfAndList {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::SpecifierOrOfAndList.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        /* We want to flatten our list of list will keeping semantics of which specifiers are the ors and ands.
         * To do this, we use the position of the nodes in the array, so the layout will always be:
         * [[s1, s2, s3, s4], [s1, s2, s3, s4], [s1, s2, s3, s4]]
         * with the specifiers maybe being the empty node.
         */
        let mut children = arrayvec::ArrayVec::new_const();
        for and_specifier in self.specifiers.iter() {
            /* Create an iterator of dummy empty nodes as ability tree */
            let dummy_nodes = std::iter::repeat(crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::empty_node());
            let dummy_nodes = dummy_nodes.map(|c| c as &dyn AbilityTreeNode);

            /* Iterate over the specifiers, filled up with dummy nodes if there are less than OR_OF_AND_LIST_INNER_ARRAY_LENGTH specifiers */
            for specifier in and_specifier
                .iter()
                .map(|s| s as &dyn AbilityTreeNode)
                .chain(dummy_nodes)
                .take(OR_OF_AND_LIST_INNER_ARRAY_LENGTH)
            {
                children.push(specifier as &dyn AbilityTreeNode);
            }
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "specifier or list:")?;
        for (i, and_specifier) in self.specifiers.iter().enumerate() {
            if i == self.specifiers.len() - 1 {
                out.push_final_branch()?;
            } else {
                out.push_inter_branch()?;
            }
            write!(out, "specifier and list:")?;
            for (j, specifier) in and_specifier.iter().enumerate() {
                if j == self.specifiers.len() - 1 {
                    out.push_final_branch()?;
                } else {
                    out.push_inter_branch()?;
                }
                specifier.display(out)?;
                out.pop_branch();
            }
            out.pop_branch();
        }
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for SpecifierOrOfAndList {
    fn dummy_init() -> Self {
        Self {
            specifiers: arrayvec::ArrayVec::new_const(),
        }
    }
}

/// Fixme: doc
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum ObjectSpecifier {
    Color(mtg_data::Color),
    Control(crate::ability_tree::terminals::ControlSpecifier),
    Cast(crate::ability_tree::terminals::CastSpecifier),
    Kind(crate::ability_tree::object::ObjectKind),
    NotOfAKind(crate::ability_tree::object::ObjectKind),
    Another(AnotherObjectSpecifier),
}

impl AbilityTreeNode for ObjectSpecifier {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ObjectSpecifier.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Color(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Control(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Cast(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Kind(child) => children.push(child as &dyn AbilityTreeNode),
            Self::NotOfAKind(child) => children.push(child as &dyn AbilityTreeNode),
            Self::Another(child) => children.push(child as &dyn AbilityTreeNode),
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            ObjectSpecifier::Color(_) => write!(out, "color specifier:")?,
            ObjectSpecifier::Kind(_) => write!(out, "kind specifier:")?,
            ObjectSpecifier::NotOfAKind(_) => write!(out, "not of a kind specifier:")?,
            ObjectSpecifier::Control(_) => write!(out, "control specifier:")?,
            ObjectSpecifier::Cast(_) => write!(out, "cast specifier:")?,
            ObjectSpecifier::Another(_) => write!(out, "not self specifier:")?,
        }
        out.push_final_branch()?;
        match self {
            ObjectSpecifier::Color(color) => color.display(out)?,
            ObjectSpecifier::Kind(object) => object.display(out)?,
            ObjectSpecifier::NotOfAKind(object) => object.display(out)?,
            ObjectSpecifier::Control(control) => control.display(out)?,
            ObjectSpecifier::Cast(cast) => cast.display(out)?,
            ObjectSpecifier::Another(another) => another.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }
}

/// Marker struct for the special object specifier "another",
/// which means "any that is not myself".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct AnotherObjectSpecifier;

impl AbilityTreeNode for AnotherObjectSpecifier {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        crate::ability_tree::NodeKind::Terminal(TerminalNodeKind::AnotherObjectSpecifier).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        arrayvec::ArrayVec::new_const()
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }
}

impl std::fmt::Display for AnotherObjectSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "another")
    }
}
