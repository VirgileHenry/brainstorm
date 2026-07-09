/// Wrapper around the creature subtype.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CreatureSubtype {
    pub creature_subtype: mtg_data::CreatureType,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl CreatureSubtype {
    pub fn all() -> impl Iterator<Item = Self> {
        mtg_data::CreatureType::all().map(|creature_subtype| CreatureSubtype {
            creature_subtype,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        })
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for CreatureSubtype {
    fn span(&self) -> boseiju_span::Span {
        self.span
    }
}

impl idris::Idris for CreatureSubtype {
    const COUNT: usize = mtg_data::CreatureType::COUNT;
    fn id(&self) -> usize {
        self.creature_subtype.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        mtg_data::CreatureType::name_from_id(id)
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for CreatureSubtype {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        Ok(Self {
            creature_subtype: {
                if let Ok(subtype) = crate::parsing::from_str_singular_or_plural(&span.text) {
                    Ok(subtype)
                } else {
                    match span.text {
                        "elves" => Ok(mtg_data::CreatureType::Elf),
                        "dwarves" => Ok(mtg_data::CreatureType::Dwarf),
                        "fungi" => Ok(mtg_data::CreatureType::Fungus),
                        "heroes" => Ok(mtg_data::CreatureType::Hero),
                        "mercenaries" => Ok(mtg_data::CreatureType::Mercenary),
                        "mice" => Ok(mtg_data::CreatureType::Mouse),
                        "octopuses" => Ok(mtg_data::CreatureType::Octopus),
                        "oxen" => Ok(mtg_data::CreatureType::Ox),
                        "pegasi" => Ok(mtg_data::CreatureType::Pegasus),
                        "werewolves" => Ok(mtg_data::CreatureType::Werewolf),
                        "wolves" => Ok(mtg_data::CreatureType::Wolf),
                        _ => Err(()),
                    }
                }
            }?,
            #[cfg(feature = "spanned_tree")]
            span: span.into(),
        })
    }
}
