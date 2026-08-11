use crate::MAX_CHILDREN_PER_NODE;
use crate::Node;
use crate::ability_tree::object::specified_object::CardManaValueSpecifier;
use crate::ability_tree::object::specified_object::KeywordAbilitySpecifier;

/// The  creature has subtype specifiers.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardCharacteristicSpecifier {
    ManaValue(CardManaValueSpecifier),
    KeywordAbility(KeywordAbilitySpecifier),
}

impl crate::Node for CardCharacteristicSpecifier {
    fn node_id(&self) -> crate::NodeKind {
        crate::NodeKind::CardCharacteristicSpecifier
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn Node, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::ManaValue(child) => children.push(child as &dyn Node),
            Self::KeywordAbility(child) => children.push(child as &dyn Node),
        }
        children
    }

    fn display(&self, out: &mut crate::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "creature characteristic specifier:")?;
        out.push_final_branch()?;
        match self {
            Self::ManaValue(child) => child.display(out)?,
            Self::KeywordAbility(child) => child.display(out)?,
        }
        out.pop_branch();
        Ok(())
    }

    fn node_tag(&self) -> &'static str {
        "creature characteristic specifier"
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CardCharacteristicSpecifier {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::ManaValue(child) => child.span(),
            Self::KeywordAbility(child) => child.span(),
        }
    }
}

impl Default for CardCharacteristicSpecifier {
    fn default() -> Self {
        Self::ManaValue(Default::default())
    }
}
