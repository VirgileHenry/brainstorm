pub mod intermediate;
pub mod tensed;
pub mod terminal;

#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    AbilityKind(intermediate::AbilityKind),
    AbilityProperty(intermediate::AbilityProperty),
    AbilityWord(intermediate::AbilityWord),
    ActionKeyword(intermediate::TensedActionKeyword),
    AdverbialAdditive(intermediate::AdverbialAdditive),
    AdverbialManner(intermediate::AdverbialManner),
    AdverbialPositional(intermediate::AdverbialPositional),
    AdverbialRestrictive(intermediate::AdverbialRestrictive),
    AmbiguousToken(intermediate::AmbiguousToken),
    ArtifactSubtype(terminal::ArtifactSubtype),
    AttachedObject(intermediate::AttachedObject),
    BackwardDuration(terminal::BackwardDuration),
    BattleSubtype(terminal::BattleSubtype),
    Bid(intermediate::Bid),
    CardActions(intermediate::CardActions),
    CardFace(intermediate::CardFace),
    CardOwnName(intermediate::CardOwnName),
    CardProperty(intermediate::CardProperty),
    CardState(intermediate::CardState),
    CardType(terminal::CardType),
    CoinFlip(intermediate::CoinFlip),
    Color(terminal::Color),
    ControlFlow(intermediate::ControlFlow),
    CountSpecifier(intermediate::CountSpecifier),
    Counter(terminal::Counter),
    CreatureGrouping(intermediate::CreatureGrouping),
    CreatureSubtype(terminal::CreatureSubtype),
    DamageKind(terminal::DamageKind),
    DayNight(intermediate::DayNight),
    Die(intermediate::DieRoll),
    Direction(intermediate::Direction),
    EnchantmentSubtype(terminal::EnchantmentSubtype),
    EnglishAdjective(intermediate::EnglishAdjective),
    EnglishAdverb(intermediate::EnglishAdverb),
    EnglishArticle(intermediate::EnglishArticle),
    EnglishComparison(intermediate::EnglishComparison),
    EnglishConditional(intermediate::EnglishConditional),
    EnglishConjunction(intermediate::EnglishConjunction),
    EnglishDemonstrative(intermediate::EnglishDemonstrative),
    EnglishDeterminer(intermediate::EnglishDeterminer),
    EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary),
    EnglishNegation(intermediate::EnglishNegation),
    EnglishPossessive(intermediate::EnglishPossessive),
    EnglishPreposition(intermediate::EnglishPreposition),
    EnglishPronoun(intermediate::EnglishPronoun),
    EnglishTemporal(intermediate::EnglishTemporal),
    EnglishVerb(intermediate::TensedEnglishVerb),
    EnglishWh(intermediate::EnglishWh),
    FlavorWord(terminal::FlavorWord),
    FormatSpecific(intermediate::FormatSpecific),
    ForwardDuration(terminal::ForwardDuration),
    GameTerm(intermediate::GameTerm),
    GlobalZone(intermediate::GlobalZone),
    InstantSorcerySubtype(terminal::InstantSorcerySubtype),
    KeywordAbility(intermediate::KeywordAbility),
    KeywordAction(intermediate::TensedKeywordAction),
    LandSubtype(terminal::LandSubtype),
    Legality(intermediate::Legality),
    ManaSymbol(terminal::ManaSymbol),
    MiscObjectName(intermediate::MiscObjectName),
    Mode(intermediate::Mode),
    NamedCard(terminal::NamedCard),
    NamedChoice(terminal::NamedChoice),
    NamedDungeon(terminal::NamedDungeon),
    NamedExpansion(terminal::NamedExpansion),
    NamedMeld(terminal::NamedMeld),
    NamedPartner(terminal::NamedPartner),
    NamedToken(terminal::NamedToken),
    NamedTransformation(terminal::NamedTransformation),
    NamedVote(terminal::NamedVote),
    NonKind(intermediate::NonKind),
    Number(intermediate::Number),
    NumberOperation(intermediate::NumberOperation),
    Order(terminal::Order),
    OwnableZone(terminal::OwnableZone),
    OwnerSpecifier(terminal::OwnerSpecifier),
    PartnerKind(intermediate::PartnerKind),
    Phase(terminal::Phase),
    PlaneswalkerSubtype(terminal::PlaneswalkerSubtype),
    PlayerAction(intermediate::TensedPlayerAction),
    PlayerDesignation(intermediate::PlayerDesignation),
    PlayerProperties(intermediate::PlayerProperties),
    PlayerSpecifier(intermediate::PlayerSpecifier),
    SagaChapterNumber(terminal::SagaChapterNumber),
    Step(terminal::Step),
    Supertype(terminal::Supertype),
    Symbol(intermediate::Symbol),
    TapUntapCost(intermediate::TapUntapCost),
    UncardSpecialTerm(intermediate::UncardSpecialTerm),
    WinLoseClause(intermediate::WinLoseClause),
}

impl Token {
    pub fn try_from_span(span: crate::span::LexerSpan) -> Option<Token> {
        if let Ok(token) = intermediate::AbilityKind::try_from(&span) {
            Some(Self::AbilityKind(token))
        } else if let Ok(token) = intermediate::AbilityProperty::try_from(&span) {
            Some(Self::AbilityProperty(token))
        } else if let Ok(token) = intermediate::AbilityWord::try_from(&span) {
            Some(Self::AbilityWord(token))
        } else if let Ok(token) = intermediate::TensedActionKeyword::try_from(&span) {
            Some(Self::ActionKeyword(token))
        } else if let Ok(token) = intermediate::AdverbialAdditive::try_from(&span) {
            Some(Self::AdverbialAdditive(token))
        } else if let Ok(token) = intermediate::AdverbialManner::try_from(&span) {
            Some(Self::AdverbialManner(token))
        } else if let Ok(token) = intermediate::AdverbialPositional::try_from(&span) {
            Some(Self::AdverbialPositional(token))
        } else if let Ok(token) = intermediate::AdverbialRestrictive::try_from(&span) {
            Some(Self::AdverbialRestrictive(token))
        } else if let Ok(token) = intermediate::AmbiguousToken::try_from(&span) {
            Some(Self::AmbiguousToken(token))
        } else if let Ok(token) = terminal::ArtifactSubtype::try_from(&span) {
            Some(Self::ArtifactSubtype(token))
        } else if let Ok(token) = intermediate::AttachedObject::try_from(&span) {
            Some(Self::AttachedObject(token))
        } else if let Ok(token) = terminal::BackwardDuration::try_from(&span) {
            Some(Self::BackwardDuration(token))
        } else if let Ok(token) = terminal::BattleSubtype::try_from(&span) {
            Some(Self::BattleSubtype(token))
        } else if let Ok(token) = intermediate::Bid::try_from(&span) {
            Some(Self::Bid(token))
        } else if let Ok(token) = intermediate::CardActions::try_from(&span) {
            Some(Self::CardActions(token))
        } else if let Ok(token) = intermediate::CardFace::try_from(&span) {
            Some(Self::CardFace(token))
        } else if let Ok(token) = intermediate::CardOwnName::try_from(&span) {
            Some(Self::CardOwnName(token))
        } else if let Ok(token) = intermediate::CardProperty::try_from(&span) {
            Some(Self::CardProperty(token))
        } else if let Ok(token) = intermediate::CardState::try_from(&span) {
            Some(Self::CardState(token))
        } else if let Ok(token) = terminal::CardType::try_from(&span) {
            Some(Self::CardType(token))
        } else if let Ok(token) = intermediate::CoinFlip::try_from(&span) {
            Some(Self::CoinFlip(token))
        } else if let Ok(token) = terminal::Color::try_from(&span) {
            Some(Self::Color(token))
        } else if let Ok(token) = intermediate::ControlFlow::try_from(&span) {
            Some(Self::ControlFlow(token))
        } else if let Ok(token) = intermediate::CountSpecifier::try_from(&span) {
            Some(Self::CountSpecifier(token))
        } else if let Ok(token) = terminal::Counter::try_from(&span) {
            Some(Self::Counter(token))
        } else if let Ok(token) = intermediate::CreatureGrouping::try_from(&span) {
            Some(Self::CreatureGrouping(token))
        } else if let Ok(token) = terminal::CreatureSubtype::try_from(&span) {
            Some(Self::CreatureSubtype(token))
        } else if let Ok(token) = terminal::DamageKind::try_from(&span) {
            Some(Self::DamageKind(token))
        } else if let Ok(token) = intermediate::DayNight::try_from(&span) {
            Some(Self::DayNight(token))
        } else if let Ok(token) = intermediate::DieRoll::try_from(&span) {
            Some(Self::Die(token))
        } else if let Ok(token) = intermediate::Direction::try_from(&span) {
            Some(Self::Direction(token))
        } else if let Ok(token) = terminal::EnchantmentSubtype::try_from(&span) {
            Some(Self::EnchantmentSubtype(token))
        } else if let Ok(token) = intermediate::EnglishAdjective::try_from(&span) {
            Some(Self::EnglishAdjective(token))
        } else if let Ok(token) = intermediate::EnglishAdverb::try_from(&span) {
            Some(Self::EnglishAdverb(token))
        } else if let Ok(token) = intermediate::EnglishArticle::try_from(&span) {
            Some(Self::EnglishArticle(token))
        } else if let Ok(token) = intermediate::EnglishComparison::try_from(&span) {
            Some(Self::EnglishComparison(token))
        } else if let Ok(token) = intermediate::EnglishConditional::try_from(&span) {
            Some(Self::EnglishConditional(token))
        } else if let Ok(token) = intermediate::EnglishConjunction::try_from(&span) {
            Some(Self::EnglishConjunction(token))
        } else if let Ok(token) = intermediate::EnglishDemonstrative::try_from(&span) {
            Some(Self::EnglishDemonstrative(token))
        } else if let Ok(token) = intermediate::EnglishDeterminer::try_from(&span) {
            Some(Self::EnglishDeterminer(token))
        } else if let Ok(token) = intermediate::EnglishModalAuxiliary::try_from(&span) {
            Some(Self::EnglishModalAuxiliary(token))
        } else if let Ok(token) = intermediate::EnglishNegation::try_from(&span) {
            Some(Self::EnglishNegation(token))
        } else if let Ok(token) = intermediate::EnglishPossessive::try_from(&span) {
            Some(Self::EnglishPossessive(token))
        } else if let Ok(token) = intermediate::EnglishPreposition::try_from(&span) {
            Some(Self::EnglishPreposition(token))
        } else if let Ok(token) = intermediate::EnglishPronoun::try_from(&span) {
            Some(Self::EnglishPronoun(token))
        } else if let Ok(token) = intermediate::EnglishTemporal::try_from(&span) {
            Some(Self::EnglishTemporal(token))
        } else if let Ok(token) = intermediate::TensedEnglishVerb::try_from(&span) {
            Some(Self::EnglishVerb(token))
        } else if let Ok(token) = intermediate::EnglishWh::try_from(&span) {
            Some(Self::EnglishWh(token))
        } else if let Ok(token) = terminal::FlavorWord::try_from(&span) {
            Some(Self::FlavorWord(token))
        } else if let Ok(token) = intermediate::FormatSpecific::try_from(&span) {
            Some(Self::FormatSpecific(token))
        } else if let Ok(token) = terminal::ForwardDuration::try_from(&span) {
            Some(Self::ForwardDuration(token))
        } else if let Ok(token) = intermediate::GameTerm::try_from(&span) {
            Some(Self::GameTerm(token))
        } else if let Ok(token) = intermediate::GlobalZone::try_from(&span) {
            Some(Self::GlobalZone(token))
        } else if let Ok(token) = terminal::InstantSorcerySubtype::try_from(&span) {
            Some(Self::InstantSorcerySubtype(token))
        } else if let Ok(token) = intermediate::KeywordAbility::try_from(&span) {
            Some(Self::KeywordAbility(token))
        } else if let Ok(token) = intermediate::TensedKeywordAction::try_from(&span) {
            Some(Self::KeywordAction(token))
        } else if let Ok(token) = terminal::LandSubtype::try_from(&span) {
            Some(Self::LandSubtype(token))
        } else if let Ok(token) = intermediate::Legality::try_from(&span) {
            Some(Self::Legality(token))
        } else if let Ok(token) = terminal::ManaSymbol::try_from(&span) {
            Some(Self::ManaSymbol(token))
        } else if let Ok(token) = intermediate::MiscObjectName::try_from(&span) {
            Some(Self::MiscObjectName(token))
        } else if let Ok(token) = intermediate::Mode::try_from(&span) {
            Some(Self::Mode(token))
        } else if let Ok(token) = terminal::NamedCard::try_from(&span) {
            Some(Self::NamedCard(token))
        } else if let Ok(token) = terminal::NamedChoice::try_from(&span) {
            Some(Self::NamedChoice(token))
        } else if let Ok(token) = terminal::NamedDungeon::try_from(&span) {
            Some(Self::NamedDungeon(token))
        } else if let Ok(token) = terminal::NamedExpansion::try_from(&span) {
            Some(Self::NamedExpansion(token))
        } else if let Ok(token) = terminal::NamedMeld::try_from(&span) {
            Some(Self::NamedMeld(token))
        } else if let Ok(token) = terminal::NamedPartner::try_from(&span) {
            Some(Self::NamedPartner(token))
        } else if let Ok(token) = terminal::NamedToken::try_from(&span) {
            Some(Self::NamedToken(token))
        } else if let Ok(token) = terminal::NamedTransformation::try_from(&span) {
            Some(Self::NamedTransformation(token))
        } else if let Ok(token) = terminal::NamedVote::try_from(&span) {
            Some(Self::NamedVote(token))
        } else if let Ok(token) = intermediate::NonKind::try_from(&span) {
            Some(Self::NonKind(token))
        } else if let Ok(token) = intermediate::Number::try_from(&span) {
            Some(Self::Number(token))
        } else if let Ok(token) = intermediate::NumberOperation::try_from(&span) {
            Some(Self::NumberOperation(token))
        } else if let Ok(token) = terminal::Order::try_from(&span) {
            Some(Self::Order(token))
        } else if let Ok(token) = terminal::OwnableZone::try_from(&span) {
            Some(Self::OwnableZone(token))
        } else if let Ok(token) = terminal::OwnerSpecifier::try_from(&span) {
            Some(Self::OwnerSpecifier(token))
        } else if let Ok(token) = intermediate::PartnerKind::try_from(&span) {
            Some(Self::PartnerKind(token))
        } else if let Ok(token) = terminal::Phase::try_from(&span) {
            Some(Self::Phase(token))
        } else if let Ok(token) = terminal::PlaneswalkerSubtype::try_from(&span) {
            Some(Self::PlaneswalkerSubtype(token))
        } else if let Ok(token) = intermediate::TensedPlayerAction::try_from(&span) {
            Some(Self::PlayerAction(token))
        } else if let Ok(token) = intermediate::PlayerDesignation::try_from(&span) {
            Some(Self::PlayerDesignation(token))
        } else if let Ok(token) = intermediate::PlayerProperties::try_from(&span) {
            Some(Self::PlayerProperties(token))
        } else if let Ok(token) = intermediate::PlayerSpecifier::try_from(&span) {
            Some(Self::PlayerSpecifier(token))
        } else if let Ok(token) = terminal::SagaChapterNumber::try_from(&span) {
            Some(Self::SagaChapterNumber(token))
        } else if let Ok(token) = terminal::Step::try_from(&span) {
            Some(Self::Step(token))
        } else if let Ok(token) = terminal::Supertype::try_from(&span) {
            Some(Self::Supertype(token))
        } else if let Ok(token) = intermediate::Symbol::try_from(&span) {
            Some(Self::Symbol(token))
        } else if let Ok(token) = intermediate::TapUntapCost::try_from(&span) {
            Some(Self::TapUntapCost(token))
        } else if let Ok(token) = intermediate::UncardSpecialTerm::try_from(&span) {
            Some(Self::UncardSpecialTerm(token))
        } else if let Ok(token) = intermediate::WinLoseClause::try_from(&span) {
            Some(Self::WinLoseClause(token))
        } else {
            None
        }
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for Token {
    fn span(&self) -> boseiju_span::Span {
        match self {
            Self::AbilityKind(child) => child.span(),
            Self::AbilityProperty(child) => child.span(),
            Self::AbilityWord(child) => child.span(),
            Self::ActionKeyword(child) => child.span(),
            Self::AdverbialAdditive(child) => child.span(),
            Self::AdverbialManner(child) => child.span(),
            Self::AdverbialPositional(child) => child.span(),
            Self::AdverbialRestrictive(child) => child.span(),
            Self::AmbiguousToken(child) => child.span(),
            Self::ArtifactSubtype(child) => child.span(),
            Self::AttachedObject(child) => child.span(),
            Self::BackwardDuration(child) => child.span(),
            Self::BattleSubtype(child) => child.span(),
            Self::Bid(child) => child.span(),
            Self::CardActions(child) => child.span(),
            Self::CardFace(child) => child.span(),
            Self::CardOwnName(child) => child.span(),
            Self::CardProperty(child) => child.span(),
            Self::CardState(child) => child.span(),
            Self::CardType(child) => child.span(),
            Self::CoinFlip(child) => child.span(),
            Self::Color(child) => child.span(),
            Self::ControlFlow(child) => child.span(),
            Self::CountSpecifier(child) => child.span(),
            Self::Counter(child) => child.span(),
            Self::CreatureGrouping(child) => child.span(),
            Self::CreatureSubtype(child) => child.span(),
            Self::DamageKind(child) => child.span(),
            Self::DayNight(child) => child.span(),
            Self::Die(child) => child.span(),
            Self::Direction(child) => child.span(),
            Self::EnchantmentSubtype(child) => child.span(),
            Self::EnglishAdjective(child) => child.span(),
            Self::EnglishAdverb(child) => child.span(),
            Self::EnglishArticle(child) => child.span(),
            Self::EnglishComparison(child) => child.span(),
            Self::EnglishConditional(child) => child.span(),
            Self::EnglishConjunction(child) => child.span(),
            Self::EnglishDemonstrative(child) => child.span(),
            Self::EnglishDeterminer(child) => child.span(),
            Self::EnglishModalAuxiliary(child) => child.span(),
            Self::EnglishNegation(child) => child.span(),
            Self::EnglishPossessive(child) => child.span(),
            Self::EnglishPreposition(child) => child.span(),
            Self::EnglishPronoun(child) => child.span(),
            Self::EnglishTemporal(child) => child.span(),
            Self::EnglishVerb(child) => child.span(),
            Self::EnglishWh(child) => child.span(),
            Self::FlavorWord(child) => child.span(),
            Self::FormatSpecific(child) => child.span(),
            Self::ForwardDuration(child) => child.span(),
            Self::GameTerm(child) => child.span(),
            Self::GlobalZone(child) => child.span(),
            Self::InstantSorcerySubtype(child) => child.span(),
            Self::KeywordAbility(child) => child.span(),
            Self::KeywordAction(child) => child.span(),
            Self::LandSubtype(child) => child.span(),
            Self::Legality(child) => child.span(),
            Self::ManaSymbol(child) => child.span(),
            Self::MiscObjectName(child) => child.span(),
            Self::Mode(child) => child.span(),
            Self::NamedCard(child) => child.span(),
            Self::NamedChoice(child) => child.span(),
            Self::NamedDungeon(child) => child.span(),
            Self::NamedExpansion(child) => child.span(),
            Self::NamedMeld(child) => child.span(),
            Self::NamedPartner(child) => child.span(),
            Self::NamedToken(child) => child.span(),
            Self::NamedTransformation(child) => child.span(),
            Self::NamedVote(child) => child.span(),
            Self::NonKind(child) => child.span(),
            Self::Number(child) => child.span(),
            Self::NumberOperation(child) => child.span(),
            Self::Order(child) => child.span(),
            Self::OwnableZone(child) => child.span(),
            Self::OwnerSpecifier(child) => child.span(),
            Self::PartnerKind(child) => child.span(),
            Self::Phase(child) => child.span(),
            Self::PlaneswalkerSubtype(child) => child.span(),
            Self::PlayerAction(child) => child.span(),
            Self::PlayerDesignation(child) => child.span(),
            Self::PlayerProperties(child) => child.span(),
            Self::PlayerSpecifier(child) => child.span(),
            Self::SagaChapterNumber(child) => child.span(),
            Self::Step(child) => child.span(),
            Self::Supertype(child) => child.span(),
            Self::Symbol(child) => child.span(),
            Self::TapUntapCost(child) => child.span(),
            Self::UncardSpecialTerm(child) => child.span(),
            Self::WinLoseClause(child) => child.span(),
        }
    }
}
