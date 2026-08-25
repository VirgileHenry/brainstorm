use boseiju_lexer::terminal::CounterKind;
use boseiju_lexer::terminal::CreatureSubtype;
use boseiju_lexer::terminal::ForwardDuration;
use boseiju_tree::AbilityTree;
use boseiju_tree::ability_tree::ability::ability_word::ExpandedAbilityWord;
use boseiju_tree::ability_tree::ability::spell::SpellAbility;
use boseiju_tree::ability_tree::ability::triggered::TriggerCondition;
use boseiju_tree::ability_tree::ability::{Ability, KeywordAbility, WrittenAbility};
use boseiju_tree::ability_tree::action::CreatureAction;
use boseiju_tree::ability_tree::colors::Colors;
use boseiju_tree::ability_tree::conditional::Condition;
use boseiju_tree::ability_tree::continuous_effect::ContinuousEffect;
use boseiju_tree::ability_tree::continuous_effect::ContinuousEffectKind;
use boseiju_tree::ability_tree::cost::AtomicCost;
use boseiju_tree::ability_tree::cost::Cost;
use boseiju_tree::ability_tree::deed;
use boseiju_tree::ability_tree::event::Event;
use boseiju_tree::ability_tree::imperative;
use boseiju_tree::ability_tree::imperative::Imperative;
use boseiju_tree::ability_tree::imperative_list::ImperativeList;
use boseiju_tree::ability_tree::mana_cost::ManaCost;
use boseiju_tree::ability_tree::number::{GameStateNumber, Number, XDefinition};
use boseiju_tree::ability_tree::object::kind::*;
use boseiju_tree::ability_tree::object::specified_object::*;
use boseiju_tree::ability_tree::object::*;
use boseiju_tree::ability_tree::object_mods::ObjectModsEffect;
use boseiju_tree::ability_tree::player::ActivePlayerReference;
use boseiju_tree::ability_tree::player::PassivePlayerReference;
use boseiju_tree::ability_tree::power_toughness::PowerToughness;
use boseiju_tree::ability_tree::power_toughness::PowerToughnessModifiers;
use boseiju_tree::ability_tree::quantifier::ActiveQuantifier;
use boseiju_tree::ability_tree::quantifier::PassiveQuantifier;
use boseiju_tree::ability_tree::replacement_effect::ReplacementEffect;
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
    ArtifactActive { artifact: ActiveArtifact },
    ArtifactKind { artifact: ArtifactKind },
    ArtifactPassive { artifact: PassiveArtifact },
    ArtifactSpecifier { specifier: ArtifactSpecifier },
    ArtifactSpecifiers { specifiers: Specifiers<ArtifactSpecifier> },
    AtomicCost { cost: AtomicCost },
    CardActive { card: ActiveCard },
    CardKind { card: CardKind },
    CardPassive { card: PassiveCard },
    CardSpecifier { specifier: CardSpecifier },
    CardSpecifiers { specifiers: Specifiers<CardSpecifier> },
    Colors { colors: Colors },
    ColorSpecifier { specifier: ColorSpecifier },
    Condition { condition: Condition },
    ContinuousEffect { effect: ContinuousEffect },
    ContinuousEffectKind { kind: ContinuousEffectKind },
    ControlSpecifier { specifier: ControlSpecifier },
    Cost { cost: Cost },
    CreatureAction { action: CreatureAction },
    CreatureActive { creature: ActiveCreature },
    CreatureKind { creature: CreatureKind },
    CreaturePassive { creature: PassiveCreature },
    CreatureSpecifier { specifier: CreatureSpecifier },
    CreatureSpecifiers { specifiers: Specifiers<CreatureSpecifier> },
    CreatureSubtype { subtype: CreatureSubtype },
    CreatureTokenTypeLine { type_line: TypeLine },
    DamageReceiver { receiver: DamageReceiver },
    DamageReceiverKind { receiver: DamageReceiverKind },
    DeedActiveForm { deed: deed::ActiveFormDeed },
    DeedAddManaActive { deed: deed::add_mana::AddManaActive },
    DeedAddManaPassive { deed: deed::add_mana::AddManaPassive },
    DeedAttackActive { deed: deed::attack::AttackActive },
    DeedAttackPassive { deed: deed::attack::AttackPassive },
    DeedCastActive { deed: deed::cast::CastActive },
    DeedCastPassive { deed: deed::cast::CastPassive },
    DeedPassiveForm { deed: deed::PassiveFormDeed },
    DeedDealDamages { deed: deed::deal_damages::DealDamages },
    DeedDestroyActive { deed: deed::destroy::DestroyActive },
    DeedDestroyPassive { deed: deed::destroy::DestroyPassive },
    DeedDrawActive { deed: deed::draw::DrawActive },
    DeedDrawPassive { deed: deed::draw::DrawPassive },
    DeedEtb { deed: deed::etb::EntersTheBattlefield },
    DeedPutCountersActive { deed: deed::put_counters::PutCountersActive },
    DeedPutCountersPassive { deed: deed::put_counters::PutCountersPassive },
    DeedSacrificeActive { deed: deed::sacrifice::SacrificeActive },
    DeedSacrificePassive { deed: deed::sacrifice::SacrificePassive },
    EnchantmentActive { enchantment: ActiveEnchantment },
    EnchantmentKind { enchantment: EnchantmentKind },
    EnchantmentPassive { enchantment: PassiveEnchantment },
    EnchantmentSpecifier { specifier: EnchantmentSpecifier },
    EnchantmentSpecifiers { specifiers: Specifiers<EnchantmentSpecifier> },
    Event { event: Event },
    ForwardDuration { duration: ForwardDuration },
    GameStateNumber { number: GameStateNumber },
    Imperative { imperative: Imperative },
    ImperativeList { imperatives: ImperativeList },
    IncomingInstant { instant: IncomingInstant },
    KeywordAbility { keyword_ability: KeywordAbility },
    LandActive { land: ActiveLand },
    LandKind { land: LandKind },
    LandPassive { land: PassiveLand },
    LandSpecifier { specifier: LandSpecifier },
    LandSpecifiers { specifiers: Specifiers<LandSpecifier> },
    LexerToken(boseiju_lexer::Token),
    ManaCost { mana_cost: ManaCost },
    ManaToAdd { mana: deed::add_mana::ManaToAdd },
    MultipleKeywordAbilities { abilities: MultipleKeywordAbilities },
    Number { number: Number },
    ObjectModsEffect { effect: ObjectModsEffect },
    PayLife { deed: deed::pay_life::PayLife },
    PayMana { deed: deed::pay_mana::PayMana },
    PermanentActive { permanent: ActivePermanent },
    PermanentKind { permanent: PermanentKind },
    PermanentPassive { permanent: PassivePermanent },
    PermanentSpecifier { specifier: PermanentSpecifier },
    PermanentSpecifiers { specifiers: Specifiers<PermanentSpecifier> },
    PlaneswalkerKind { planeswalker: PlaneswalkerKind },
    PlaneswalkerSpecifiers { specifiers: Specifiers<PlaneswalkerSpecifier> },
    PlayerAction { action: imperative::PlayerAction },
    PlayerActionInModal { mode: SpellAbility },
    PlayerActive { player: ActivePlayerReference },
    PlayerPassive { player: PassivePlayerReference },
    PowerToughness { power_toughness: PowerToughness },
    PowerToughnessModifiers { modifiers: PowerToughnessModifiers },
    PutCounterKind { kind: CounterKind },
    ReplacementEffect { effect: ReplacementEffect },
    QuantifierActive { count: ActiveQuantifier },
    QuantifierPassive { count: PassiveQuantifier },
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
    SpellActive { spell: ActiveSpell },
    SpellPassive { spell: PassiveSpell },
    SpellKind { spell: SpellKind },
    SpellSpecifier { specifier: SpellSpecifier },
    SpellSpecifiers { specifiers: Specifiers<SpellSpecifier> },
    Statement { statement: Statement },
    TapActive { deed: deed::tap::TapActive },
    TapPassive { deed: deed::tap::TapPassive },
    TokenDefinition { token: TokenLayout },
    TypeLine { type_line: TypeLine },
    TriggerCondition { condition: TriggerCondition },
    UntapActive { deed: deed::untap::UntapActive },
    UntapPassive { deed: deed::untap::UntapPassive },
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
