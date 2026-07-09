#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tensed<T> {
    tense: Tense,
    token: T,
}

impl<T> Tensed<T> {
    /// Create a new tensed token with the imperative form.
    ///
    /// These are imperatives, meaning the game is instructing someone to perform an action.
    /// Although linguistically this is simply the base form, in Magic it almost always functions as an imperative.
    ///
    /// Examples: Choose, Draw, Discard, Exile.
    pub fn base_form(token: T) -> Self {
        Self {
            tense: Tense::BaseForm,
            token,
        }
    }

    /// Create a new tensed token with the past participle form of a verb.
    ///
    /// Refer to objects that have been interacted with previously.
    ///
    /// Examples: Chosen, Drawn, Discarded, Exiled
    pub fn past_participle(token: T) -> Self {
        Self {
            tense: Tense::PastParticiple,
            token,
        }
    }

    /// Create a new tensed token with the present participle form.
    ///
    /// Magic generally avoids "-ing" forms because instructions are written imperatively.
    /// Still, they do appear in some replacement abilities: "while voting, you may vote one more time"
    ///
    /// Examples: Choosing, Drawing, Discarding, Exiling
    pub fn present_participle(token: T) -> Self {
        Self {
            tense: Tense::PresentParticiple,
            token,
        }
    }

    /// Create a new tensed token with the simple past form.
    ///
    /// Instead, it's referring to a completed action.
    /// Magic often uses it in relative clauses.
    ///
    /// Examples: Chose, Drew, Discarded, Exiled
    pub fn simple_past(token: T) -> Self {
        Self {
            tense: Tense::SimplePast,
            token,
        }
    }

    /// Create a new tensed token with third Person of the Singular Present form.
    ///
    /// This is used whenever the subject isn't "you", but another noun.
    /// It is used when referring to any player doing an action, or instructing
    /// any player to do an action.
    ///
    /// For example, "each player draws a card", or "whenever a player draws a card"
    ///
    /// Examples: Chooses, Draws, Discards, Exiles.
    pub fn third_person_singular_present(token: T) -> Self {
        Self {
            tense: Tense::ThirdPersonSingularPresent,
            token,
        }
    }
}

impl<T: idris::Idris> idris::Idris for Tensed<T> {
    const COUNT: usize = { T::COUNT * <Tense as idris::Idris>::COUNT };
    fn id(&self) -> usize {
        self.token.id() * <Tense as idris::Idris>::COUNT + self.tense.id()
    }
    fn name_from_id(id: usize) -> &'static str {
        T::name_from_id(id / <Tense as idris::Idris>::COUNT)
    }
}

#[cfg(feature = "spanned_tree")]
impl<T: boseiju_span::Spanned> boseiju_span::Spanned for Tensed<T> {
    fn span(&self) -> boseiju_span::Span {
        self.token.span()
    }
}

#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Tense {
    /// The base form form of a verb.
    ///
    /// These are base forms, meaning the game is instructing someone to perform an action.
    /// Although linguistically this is simply the base form, in Magic it functions as an imperative.
    ///
    /// Examples: Choose, Draw, Discard, Exile.
    BaseForm,
    /// The past participle form of a verb.
    ///
    /// Refer to objects that have been interacted with previously.
    ///
    /// Examples: Chosen, Drawn, Discarded, Exiled
    PastParticiple,
    /// The present participle form of a verb.
    ///
    /// Magic generally avoids "-ing" forms because instructions are written imperatively.
    /// Still, they do appear in some replacement abilities: "while voting, you may vote one more time"
    ///
    /// Examples: Choosing, Drawing, Discarding, Exiling
    PresentParticiple,
    /// The simple past form of a verb.
    ///
    /// Instead, it's referring to a completed action.
    /// Magic often uses it in relative clauses.
    ///
    /// Examples: Chose, Drew, Discarded, Exiled
    SimplePast,
    /// Third Person of the Singular Present form
    ///
    /// This is used whenever the subject isn't "you", but another noun.
    /// It is used when referring to any player doing an action, or instructing
    /// any player to do an action.
    ///
    /// For example, "each player draws a card", or "whenever a player draws a card"
    ///
    /// Examples: Chooses, Draws, Discards, Exiles.
    ThirdPersonSingularPresent,
}
