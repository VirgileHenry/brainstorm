use crate::ability_tree::AbilityTree;
use crate::ability_tree::ability::ability_word::ExpandedAbilityWord;
use crate::ability_tree::ability::spell::SpellAbility;
use crate::ability_tree::ability::statik::StaticAbilityKind;
use crate::ability_tree::ability::statik::continuous_effect::{ContinuousEffect, PowerToughnessModifiers};
use crate::ability_tree::ability::statik::cost_modification_effect::{CostModification, CostModificationEffect};
use crate::ability_tree::ability::triggered::TriggerCondition;
use crate::ability_tree::ability::{Ability, KeywordAbility, WrittenAbility};
use crate::ability_tree::action::CreatureAction;
use crate::ability_tree::card_layout::TokenLayout;
use crate::ability_tree::colors::Colors;
use crate::ability_tree::conditional::Condition;
use crate::ability_tree::cost::Cost;
use crate::ability_tree::event::Event;
use crate::ability_tree::imperative::{CreatedTokenKind, Imperative, ImperativeKind, ManaToAdd};
use crate::ability_tree::imperative_list::ImperativeList;
use crate::ability_tree::number::{GameStateNumber, Number, XDefinition};
use crate::ability_tree::object::kind::*;
use crate::ability_tree::object::specified_object::*;
use crate::ability_tree::object::*;
use crate::ability_tree::player::PlayerSpecifier;
use crate::ability_tree::statement::Statement;
use crate::ability_tree::terminals::{CounterKind, CreatureSubtype, ManaCost};
use crate::ability_tree::time::{ForwardDuration, IncomingInstant, RecurrentInstant};
use crate::ability_tree::type_line::TypeLine;
use crate::ability_tree::zone::ZoneReference;

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
    CountSpecifier { count: CountSpecifier },
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
    ImperativeChoices { choices: ImperativeChoices },
    ImperativeKind { imperative: ImperativeKind },
    ImperativeList { imperatives: ImperativeList },
    IncomingInstant { instant: IncomingInstant },
    KeywordAbility { keyword_ability: KeywordAbility },
    Land { land: Land },
    LandKind { land: LandKind },
    LandSpecifier { specifier: LandSpecifier },
    LandSpecifiers { specifiers: Specifiers<LandSpecifier> },
    LexerToken(crate::lexer::tokens::Token),
    ManaCost { mana_cost: ManaCost },
    ManaToAdd { mana: ManaToAdd },
    MultipleKeywordAbilities { abilities: MultipleKeywordAbilities },
    Number { number: Number },
    Permanent { permanent: Permanent },
    PermanentKind { permanent: PermanentKind },
    PermanentSpecifier { specifier: PermanentSpecifier },
    PermanentSpecifiers { specifiers: Specifiers<PermanentSpecifier> },
    PlaneswalkerKind { planeswalker: PlaneswalkerKind },
    Player { player: PlayerSpecifier },
    PowerToughnessModifiers { modifiers: PowerToughnessModifiers },
    PutCounterKind { kind: CounterKind },
    RecurrentInstant { instant: RecurrentInstant },
    SpecifiedArtifact { artifact: SpecifiedArtifact },
    SpecifiedCard { card: SpecifiedCard },
    SpecifiedCreature { creature: SpecifiedCreature },
    SpecifiedEnchantment { enchantment: SpecifiedEnchantment },
    SpecifiedLand { land: SpecifiedLand },
    SpecifiedPermanent { permanent: SpecifiedPermanent },
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

impl From<crate::lexer::tokens::Token> for ParserNode {
    fn from(token: crate::lexer::tokens::Token) -> Self {
        ParserNode::LexerToken(token)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImperativeChoices {
    pub choices: crate::utils::HeapArrayVec<SpellAbility, 11 /* Fixme */>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::utils::DummyInit for ImperativeChoices {
    fn dummy_init() -> Self {
        Self {
            choices: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultipleKeywordAbilities {
    pub abilities: crate::utils::HeapArrayVec<KeywordAbility, 12 /* Fixme */>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

impl crate::utils::DummyInit for MultipleKeywordAbilities {
    fn dummy_init() -> Self {
        Self {
            abilities: crate::utils::dummy(),
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
