#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum ObjectKind {
    ArtifactSubtype(mtg_data::ArtifactType),
    BattleSubtype(mtg_data::BattleType),
    Card,
    CreatureSubtype(mtg_data::CreatureType),
    EnchantmentSubtype(mtg_data::EnchantmentType),
    InstantSubtype(mtg_data::SpellType),
    LandSubtype(mtg_data::LandType),
    Permanent,
    PlaneswalkerSubtype(mtg_data::PlaneswalkerType),
    Spell,
    Supertype(mtg_data::Supertype),
    Type(mtg_data::CardType),
    SorcerySubtype(mtg_data::SpellType),
}

impl std::fmt::Display for ObjectKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectKind::ArtifactSubtype(subtype) => subtype.fmt(f),
            ObjectKind::BattleSubtype(subtype) => subtype.fmt(f),
            ObjectKind::Card => write!(f, "card"),
            ObjectKind::CreatureSubtype(subtype) => subtype.fmt(f),
            ObjectKind::EnchantmentSubtype(subtype) => subtype.fmt(f),
            ObjectKind::InstantSubtype(subtype) => subtype.fmt(f),
            ObjectKind::LandSubtype(subtype) => subtype.fmt(f),
            ObjectKind::Permanent => write!(f, "permanent"),
            ObjectKind::PlaneswalkerSubtype(subtype) => subtype.fmt(f),
            ObjectKind::Spell => write!(f, "spell"),
            ObjectKind::Supertype(supertype) => supertype.fmt(f),
            ObjectKind::Type(ty) => ty.fmt(f),
            ObjectKind::SorcerySubtype(subtype) => subtype.fmt(f),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ObjectReference {
    SelfReferencing,
    SpecifiedObj {
        amount: crate::ability_tree::terminals::CountSpecifier,
        specifier: ObjectSpecifier,
    },
}

impl crate::ability_tree::AbilityTreeImpl for ObjectReference {
    fn display<W: std::io::Write>(
        &self,
        out: &mut crate::utils::TreeFormatter<'_, W>,
    ) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            ObjectReference::SelfReferencing => write!(out, "Self Referencing (~)"),
            ObjectReference::SpecifiedObj {
                amount,
                specifier: specifiers,
            } => {
                write!(out, "Specified Object:")?;
                out.push_inter_branch()?;
                write!(out, "Amount:")?;
                out.push_final_branch()?;
                write!(out, "{amount}")?;
                out.pop_branch();
                out.next_final_branch()?;
                write!(out, "Specifier(s):")?;
                out.push_final_branch()?;
                specifiers.display(out)?;
                out.pop_branch();
                out.pop_branch();
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ObjectSpecifier {
    And(Box<ObjectSpecifier>, Box<ObjectSpecifier>),
    Color(mtg_data::Color),
    Control(crate::ability_tree::terminals::ControlSpecifier),
    Kind(ObjectKind),
    Or(Box<ObjectSpecifier>, Box<ObjectSpecifier>),
}

impl crate::ability_tree::AbilityTreeImpl for ObjectSpecifier {
    fn display<W: std::io::Write>(
        &self,
        out: &mut crate::utils::TreeFormatter<'_, W>,
    ) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            ObjectSpecifier::And(spec1, spec2) => {
                spec1.display(out)?;
                write!(out, " and ")?;
                spec2.display(out)?;
                Ok(())
            }
            ObjectSpecifier::Color(color) => write!(out, "color specifier: {color}"),
            ObjectSpecifier::Kind(object) => write!(out, "kind specifier: {object}"),
            ObjectSpecifier::Control(control) => write!(out, "control specifier: {control}"),
            ObjectSpecifier::Or(spec1, spec2) => {
                spec1.display(out)?;
                write!(out, " or ")?;
                spec2.display(out)?;
                Ok(())
            }
        }
    }
}
