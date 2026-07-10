pub mod intermediate;
pub mod tensed;
pub mod terminal;

#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    AbilityKind(intermediate::AbilityKind),
    AbilityWord(intermediate::AbilityWord),
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
    EnglishKeyword(intermediate::EnglishKeyword),
    FlavorWord(terminal::FlavorWord),
    FormatSpecific(intermediate::FormatSpecific),
    ForwardDuration(terminal::ForwardDuration),
    GameTerm(intermediate::GameTerm),
    GlobalZone(intermediate::GlobalZone),
    InstantSorcerySubtype(terminal::InstantSorcerySubtype),
    KeywordAbility(intermediate::KeywordAbility),
    LandSubtype(terminal::LandSubtype),
    ManaSymbol(terminal::ManaSymbol),
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
    PlayerDesignation(intermediate::PlayerDesignation),
    PlayerProperties(intermediate::PlayerProperties),
    PlayerSpecifier(intermediate::PlayerSpecifier),
    SagaChapterNumber(terminal::SagaChapterNumber),
    SpecialCost(intermediate::SpecialCost),
    Step(terminal::Step),
    Supertype(terminal::Supertype),
    TapUntapCost(intermediate::TapUntapCost),
    TensedActionKeyword(intermediate::TensedActionKeyword),
    TensedKeywordAction(intermediate::TensedKeywordAction),
    TensedPlayerAction(intermediate::TensedPlayerAction),
    VhyToSortLater(intermediate::VhyToSortLater),
    WinLoseClause(intermediate::WinLoseClause),
}

impl Token {
    pub fn try_from_span(span: crate::span::LexerSpan) -> Option<Token> {
        if let Ok(token) = intermediate::AbilityKind::try_from(&span) {
            Some(Self::AbilityKind(token))
        } else if let Ok(token) = intermediate::AbilityWord::try_from(&span) {
            Some(Self::AbilityWord(token))
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
        } else if let Ok(token) = intermediate::EnglishKeyword::try_from(&span) {
            Some(Self::EnglishKeyword(token))
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
        } else if let Ok(token) = terminal::LandSubtype::try_from(&span) {
            Some(Self::LandSubtype(token))
        } else if let Ok(token) = terminal::ManaSymbol::try_from(&span) {
            Some(Self::ManaSymbol(token))
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
        } else if let Ok(token) = intermediate::PlayerDesignation::try_from(&span) {
            Some(Self::PlayerDesignation(token))
        } else if let Ok(token) = intermediate::PlayerProperties::try_from(&span) {
            Some(Self::PlayerProperties(token))
        } else if let Ok(token) = intermediate::PlayerSpecifier::try_from(&span) {
            Some(Self::PlayerSpecifier(token))
        } else if let Ok(token) = terminal::SagaChapterNumber::try_from(&span) {
            Some(Self::SagaChapterNumber(token))
        } else if let Ok(token) = intermediate::SpecialCost::try_from(&span) {
            Some(Self::SpecialCost(token))
        } else if let Ok(token) = terminal::Step::try_from(&span) {
            Some(Self::Step(token))
        } else if let Ok(token) = terminal::Supertype::try_from(&span) {
            Some(Self::Supertype(token))
        } else if let Ok(token) = intermediate::TapUntapCost::try_from(&span) {
            Some(Self::TapUntapCost(token))
        } else if let Ok(token) = intermediate::TensedActionKeyword::try_from(&span) {
            Some(Self::TensedActionKeyword(token))
        } else if let Ok(token) = intermediate::TensedKeywordAction::try_from(&span) {
            Some(Self::TensedKeywordAction(token))
        } else if let Ok(token) = intermediate::TensedPlayerAction::try_from(&span) {
            Some(Self::TensedPlayerAction(token))
        } else if let Ok(token) = intermediate::VhyToSortLater::try_from(&span) {
            Some(Self::VhyToSortLater(token))
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
            Self::AbilityWord(child) => child.span(),
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
            Self::EnglishKeyword(child) => child.span(),
            Self::FlavorWord(child) => child.span(),
            Self::FormatSpecific(child) => child.span(),
            Self::ForwardDuration(child) => child.span(),
            Self::GameTerm(child) => child.span(),
            Self::GlobalZone(child) => child.span(),
            Self::InstantSorcerySubtype(child) => child.span(),
            Self::KeywordAbility(child) => child.span(),
            Self::LandSubtype(child) => child.span(),
            Self::ManaSymbol(child) => child.span(),
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
            Self::PlayerDesignation(child) => child.span(),
            Self::PlayerProperties(child) => child.span(),
            Self::PlayerSpecifier(child) => child.span(),
            Self::SagaChapterNumber(child) => child.span(),
            Self::SpecialCost(child) => child.span(),
            Self::Step(child) => child.span(),
            Self::Supertype(child) => child.span(),
            Self::TapUntapCost(child) => child.span(),
            Self::TensedActionKeyword(child) => child.span(),
            Self::TensedKeywordAction(child) => child.span(),
            Self::TensedPlayerAction(child) => child.span(),
            Self::VhyToSortLater(child) => child.span(),
            Self::WinLoseClause(child) => child.span(),
        }
    }
}
