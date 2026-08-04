use boseiju_lexer::terminal::CounterKind;
use boseiju_lexer::terminal::CreatureSubtype;
use boseiju_lexer::terminal::ForwardDuration;
use boseiju_tree::AbilityTree;
use boseiju_tree::ability_tree::ability::ability_word::ExpandedAbilityWord;
use boseiju_tree::ability_tree::ability::spell::SpellAbility;
use boseiju_tree::ability_tree::ability::statik::StaticAbilityKind;
use boseiju_tree::ability_tree::ability::statik::continuous_effect::ContinuousEffect;
use boseiju_tree::ability_tree::ability::statik::continuous_effect::continuous_effect_kind::PowerToughnessModifiers;
use boseiju_tree::ability_tree::ability::statik::cost_modification_effect::{CostModification, CostModificationEffect};
use boseiju_tree::ability_tree::ability::triggered::TriggerCondition;
use boseiju_tree::ability_tree::ability::{Ability, KeywordAbility, WrittenAbility};
use boseiju_tree::ability_tree::action::CreatureAction;
use boseiju_tree::ability_tree::colors::Colors;
use boseiju_tree::ability_tree::conditional::Condition;
use boseiju_tree::ability_tree::cost::Cost;
use boseiju_tree::ability_tree::event::Event;
use boseiju_tree::ability_tree::imperative::{CreatedTokenKind, Imperative, ImperativeKind, ManaToAdd};
use boseiju_tree::ability_tree::imperative_list::ImperativeList;
use boseiju_tree::ability_tree::mana_cost::ManaCost;
use boseiju_tree::ability_tree::number::{GameStateNumber, Number, XDefinition};
use boseiju_tree::ability_tree::object::kind::*;
use boseiju_tree::ability_tree::object::specified_object::*;
use boseiju_tree::ability_tree::object::*;
use boseiju_tree::ability_tree::player::PlayerReference;
use boseiju_tree::ability_tree::power_toughness::PowerToughness;
use boseiju_tree::ability_tree::quantifier::Quantifier;
use boseiju_tree::ability_tree::statement::Statement;
use boseiju_tree::ability_tree::time::{IncomingInstant, RecurrentInstant};
use boseiju_tree::ability_tree::type_line::TypeLine;
use boseiju_tree::ability_tree::zone::ZoneReference;
use boseiju_tree::card::layout::TokenLayout;

/// Since this can carry entire ability trees, we need to box the biggest variants.
/// Otherwise, this can easily blow up the stack when attempting to store multiple of them.
/// Current size is 112 bytes, let's try to keep it around here ?
#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserNode {
    Ability { ability: Ability },
    AbilityTree { tree: AbilityTree },
    AbilityWord { ability_word: ExpandedAbilityWord },
    AnotherSpecifier { specifier: AnotherObjectSpecifier },
    Artifact { artifact: Artifact },
    ArtifactKind { artifact: ArtifactKind },
    ArtifactSpecifier { specifier: ArtifactSpecifier },
    ArtifactSpecifiers { specifiers: Specifiers<ArtifactSpecifier> },
    Card { card: Card },
    CardKind { card: CardKind },
    CardSpecifier { specifier: CardSpecifier },
    CardSpecifiers { specifiers: Specifiers<CardSpecifier> },
    Colors { colors: Colors },
    ColorSpecifier { specifier: ColorSpecifier },
    Condition { condition: Condition },
    ContinuousEffect { effect: ContinuousEffect },
    ControlSpecifier { specifier: ControlSpecifier },
    Cost { cost: Cost },
    CostModification { cost_modification: CostModification },
    CostModificationEffect { cost_modification: CostModificationEffect },
    CreatedTokenKind { kind: CreatedTokenKind },
    CreatureAction { action: CreatureAction },
    Creature { creature: Creature },
    CreatureKind { creature: CreatureKind },
    CreatureSpecifier { specifier: CreatureSpecifier },
    CreatureSpecifiers { specifiers: Specifiers<CreatureSpecifier> },
    CreatureSubtype { subtype: CreatureSubtype },
    CreatureTokenTypeLine { type_line: TypeLine },
    DamageReceiver { receiver: DamageReceiver },
    DamageReceiverKind { receiver: DamageReceiverKind },
    Enchantment { enchantment: Enchantment },
    EnchantmentKind { enchantment: EnchantmentKind },
    EnchantmentSpecifier { specifier: EnchantmentSpecifier },
    EnchantmentSpecifiers { specifiers: Specifiers<EnchantmentSpecifier> },
    Event { event: Event },
    ForwardDuration { duration: ForwardDuration },
    GameStateNumber { number: GameStateNumber },
    Imperative { imperative: Imperative },
    ImperativeAsCost { cost: Imperative },
    ImperativeChoices { choices: ImperativeChoices },
    ImperativeKind { imperative: ImperativeKind },
    ImperativeList { imperatives: ImperativeList },
    IncomingInstant { instant: IncomingInstant },
    KeywordAbility { keyword_ability: KeywordAbility },
    Land { land: Land },
    LandKind { land: LandKind },
    LandSpecifier { specifier: LandSpecifier },
    LandSpecifiers { specifiers: Specifiers<LandSpecifier> },
    LexerToken(boseiju_lexer::Token),
    ManaCost { mana_cost: ManaCost },
    ManaToAdd { mana: ManaToAdd },
    MultipleKeywordAbilities { abilities: MultipleKeywordAbilities },
    Number { number: Number },
    Permanent { permanent: Permanent },
    PermanentKind { permanent: PermanentKind },
    PermanentSpecifier { specifier: PermanentSpecifier },
    PermanentSpecifiers { specifiers: Specifiers<PermanentSpecifier> },
    PlaneswalkerKind { planeswalker: PlaneswalkerKind },
    Player { player: PlayerReference },
    PowerToughness { power_toughness: PowerToughness },
    PowerToughnessModifiers { modifiers: PowerToughnessModifiers },
    PutCounterKind { kind: CounterKind },
    Quantifier { count: Quantifier },
    RecurrentInstant { instant: RecurrentInstant },
    SpecifiedArtifact { artifact: SpecifiedArtifact },
    SpecifiedCard { card: SpecifiedCard },
    SpecifiedCreature { creature: SpecifiedCreature },
    SpecifiedEnchantment { enchantment: SpecifiedEnchantment },
    SpecifiedLand { land: SpecifiedLand },
    SpecifiedPermanent { permanent: SpecifiedPermanent },
    SpecifiedPlaneswalker { planeswalker: SpecifiedPlaneswalker },
    SpecifiedSpell { spell: SpecifiedSpell },
    SpellAbility { ability: SpellAbility },
    Spell { spell: Spell },
    SpellKind { spell: SpellKind },
    SpellSpecifier { specifier: SpellSpecifier },
    SpellSpecifiers { specifiers: Specifiers<SpellSpecifier> },
    Statement { statement: Statement },
    StaticAbilityKind { kind: StaticAbilityKind },
    TokenDefinition { token: TokenLayout },
    TypeLine { type_line: TypeLine },
    TriggerCondition { condition: TriggerCondition },
    WrittenAbility { ability: WrittenAbility },
    XDefinition { definition: XDefinition },
    ZoneReference { zone: ZoneReference },
}

impl From<boseiju_lexer::Token> for ParserNode {
    fn from(token: boseiju_lexer::Token) -> Self {
        ParserNode::LexerToken(token)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImperativeChoices {
    pub choices: boseiju_tree::HeapArrayVec<SpellAbility, 11 /* Fixme */>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Default for ImperativeChoices {
    fn default() -> Self {
        Self {
            choices: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultipleKeywordAbilities {
    pub abilities: boseiju_tree::HeapArrayVec<KeywordAbility, 12 /* Fixme */>,
    #[cfg(feature = "spanned_tree")]
    pub span: boseiju_span::Span,
}

impl Default for MultipleKeywordAbilities {
    fn default() -> Self {
        Self {
            abilities: Default::default(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
