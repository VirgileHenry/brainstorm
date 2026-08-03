impl crate::card::Parse for boseiju_tree::ability_tree::type_line::TypeLine {
    type Source = String;
    type Error = TypeLineParseError;

    fn parse(from: &Self::Source) -> Result<Self, Self::Error> {
        use idris::Idris;
        let mut result = Self::empty();

        lazy_static::lazy_static!(
            static ref tokens_regex: regex::Regex = regex::Regex::new(r"\b\w+\b")
                .expect("Failed to compile the tokens regex");
        );

        let mut spans = tokens_regex
            .find_iter(&from)
            .map(|m| boseiju_lexer::LexerSpan {
                start: m.start(),
                length: m.len(),
                text: m.as_str(),
            })
            .peekable();

        /* Parse supertypes first */
        while let Some(token) = spans.peek() {
            match boseiju_lexer::terminal::Supertype::try_from(token) {
                Ok(supertype) => {
                    if result.supertypes[supertype.id()] {
                        return Err(TypeLineParseError::DuplicateElem {
                            duplicate: supertype.to_string(),
                        });
                    } else {
                        result.supertypes[supertype.id()] = true;
                        let _ = spans.next(); /* Token was peeked, pop it out */
                    }
                }
                Err(_) => break,
            }
        }

        /* Parse all types then */
        while let Some(span) = spans.peek() {
            match boseiju_lexer::terminal::CardType::try_from(span) {
                Ok(card_type) => {
                    if result.card_types[card_type.id()] {
                        return Err(TypeLineParseError::DuplicateElem {
                            duplicate: card_type.to_string(),
                        });
                    } else {
                        result.card_types[card_type.id()] = true;
                        let _ = spans.next(); /* Token was peeked, pop it out */
                    }
                }
                Err(_) => break,
            }
        }

        /* Parse subtypes, only allow them if the associated type is found */
        while let Some(token) = spans.next() {
            if result.card_types[mtg_data::CardType::Artifact.id()] {
                if let Ok(new_subtype) = boseiju_lexer::terminal::ArtifactSubtype::try_from(&token) {
                    if result.artifact[new_subtype.id()] {
                        return Err(TypeLineParseError::DuplicateElem {
                            duplicate: new_subtype.to_string(),
                        });
                    } else {
                        result.artifact[new_subtype.id()] = true;
                        continue;
                    }
                }
            }
            if result.card_types[mtg_data::CardType::Battle.id()] {
                if let Ok(new_subtype) = boseiju_lexer::terminal::BattleSubtype::try_from(&token) {
                    if result.battle[new_subtype.id()] {
                        return Err(TypeLineParseError::DuplicateElem {
                            duplicate: new_subtype.to_string(),
                        });
                    } else {
                        result.battle[new_subtype.id()] = true;
                        continue;
                    }
                }
            }
            if result.card_types[mtg_data::CardType::Creature.id()] {
                if let Ok(new_subtype) = boseiju_lexer::terminal::CreatureSubtype::try_from(&token) {
                    if result.creature[new_subtype.id()] {
                        return Err(TypeLineParseError::DuplicateElem {
                            duplicate: new_subtype.to_string(),
                        });
                    } else {
                        result.creature[new_subtype.id()] = true;
                        continue;
                    }
                }
            }
            if result.card_types[mtg_data::CardType::Enchantment.id()] {
                if let Ok(new_subtype) = boseiju_lexer::terminal::EnchantmentSubtype::try_from(&token) {
                    if result.enchantment[new_subtype.id()] {
                        return Err(TypeLineParseError::DuplicateElem {
                            duplicate: new_subtype.to_string(),
                        });
                    } else {
                        result.enchantment[new_subtype.id()] = true;
                        continue;
                    }
                }
            }
            if result.card_types[mtg_data::CardType::Instant.id()] {
                if let Ok(new_subtype) = boseiju_lexer::terminal::InstantSorcerySubtype::try_from(&token) {
                    if result.instant[new_subtype.id()] {
                        return Err(TypeLineParseError::DuplicateElem {
                            duplicate: new_subtype.to_string(),
                        });
                    } else {
                        result.instant[new_subtype.id()] = true;
                        continue;
                    }
                }
            }
            if result.card_types[mtg_data::CardType::Kindred.id()] {
                if let Ok(new_subtype) = boseiju_lexer::terminal::CreatureSubtype::try_from(&token) {
                    if result.kindred[new_subtype.id()] {
                        return Err(TypeLineParseError::DuplicateElem {
                            duplicate: new_subtype.to_string(),
                        });
                    } else {
                        result.kindred[new_subtype.id()] = true;
                        continue;
                    }
                }
            }
            if result.card_types[mtg_data::CardType::Land.id()] {
                if let Ok(new_subtype) = boseiju_lexer::terminal::LandSubtype::try_from(&token) {
                    if result.land[new_subtype.id()] {
                        return Err(TypeLineParseError::DuplicateElem {
                            duplicate: new_subtype.to_string(),
                        });
                    } else {
                        result.land[new_subtype.id()] = true;
                        continue;
                    }
                }
            }
            if result.card_types[mtg_data::CardType::Planeswalker.id()] {
                if let Ok(new_subtype) = boseiju_lexer::terminal::PlaneswalkerSubtype::try_from(&token) {
                    if result.planeswalker[new_subtype.id()] {
                        return Err(TypeLineParseError::DuplicateElem {
                            duplicate: new_subtype.to_string(),
                        });
                    } else {
                        result.planeswalker[new_subtype.id()] = true;
                        continue;
                    }
                }
            }
            if result.card_types[mtg_data::CardType::Sorcery.id()] {
                if let Ok(new_subtype) = boseiju_lexer::terminal::InstantSorcerySubtype::try_from(&token) {
                    if result.sorcery[new_subtype.id()] {
                        return Err(TypeLineParseError::DuplicateElem {
                            duplicate: new_subtype.to_string(),
                        });
                    } else {
                        result.sorcery[new_subtype.id()] = true;
                        continue;
                    }
                }
            }
            /* If we arrive here, no type managed to validated the given subtype, that's an error! */
            return Err(TypeLineParseError::UnknownElem {
                elem: token.text.to_string(),
            });
        }

        Ok(result)
    }
}

#[derive(Debug)]
pub enum TypeLineParseError {
    DuplicateElem { duplicate: String },
    UnknownElem { elem: String },
}

impl std::fmt::Display for TypeLineParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateElem { duplicate } => write!(f, "Duplicate element in type line: {duplicate}"),
            Self::UnknownElem { elem } => write!(f, "Unknown element in type line: {elem}"),
        }
    }
}

impl std::error::Error for TypeLineParseError {}
