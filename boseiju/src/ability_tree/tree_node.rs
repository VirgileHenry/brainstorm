/// This enum lists all the possible nodes of the ability tree.
///
/// The goal of this enum is to distribute an unique identifier to all node kinds.
/// The [`idris_derive::Idris`] implements the [`idris::Idris`] trait for this enum,
/// providing a unique id for each variant, allowing to get that variant through this enum.
///
/// The id is distributed recursively to all variants that are unnamed with a single field.
///
/// Sometimes, it is desired to have an id for an enum node, but we also want to distribute ids
/// to all variants of this node, since its the kind of node that carries enum only.
///
/// To do this, we create two variants, an IdMarker variant to give the node an id, and a variant
/// that use the idris derive to recusrively add the nodes to all child variants.
#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeKind {
    /// A special "no node" kind.
    ///
    /// It's sometimes usefull to put "blank spaces" nodes when providing children.
    /// Check the [`AbilityTreeNode::children()`] implementation of the [`SpecifierOrOfAndList`] for an example.
    ///
    /// The node is placed first so that idris will give him the symbolic id 0.
    _EmptyNode,
    /// The none node is another speical case of node, that does not mean that there is no node,
    /// but rather that there could have been something, but instead the none variant is present.
    _NoneNode,
    Ability,
    AbilityTree,
    ActivatedAbility,
    CharacteristicDefiningAbility,
    CreateTokensEvent,
    ChooseImperative,
    ContinuousEffect,
    ContinuousEffectKind,
    ContinuousEffectObjectGainsAbilies,
    ContinuousEffectReplacementEvent,
    Cost,
    CostModification,
    CostModificationCostMore,
    CostModificationCostLess,
    CostModificationCostSet,
    CostModificationEffect,
    CounterOnPermanent, /* Fixme: better name for this one */
    CounterOnPermanentReplacement,
    DealsDamageImperative,
    DestroyImperative,
    EffectEventSource,
    EntersTheBattlefieldEvent,
    Event,
    EventReplacement,
    EventSource,
    EventSourceReferenceIdMarker,
    EventSourceReference(crate::ability_tree::event::replacement::EventSourceReference),
    ExileImperative,
    ExileFollowUp,
    ExileFollowUpReturn,
    ExpandedKeywordAbilityIdMarker,
    ExpandedKeywordAbility(crate::ability_tree::ability::keyword::ExpandedKeywordAbility),
    IfCondition,
    IfConditionEventOccured,
    Imperative,
    ImperativeChoices,
    KeywordAbility,
    LifeGainedEvent,
    ManaCost,
    MayAbility,
    MtgData(MtgDataNodeKind),
    NumberIdMarker,
    Number(crate::ability_tree::number::Number),
    ObjectReference,
    ObjectSpecifier,
    ObjectSpecifiers,
    OwnedZone,
    PlayerCastsSpellEvent,
    PreviouslyMentionnedCounter,
    PreviouslyMentionnedObject,
    PreviouslyMentionnedToken,
    PutCounterOnPermanentEvent,
    PutCountersImperative,
    ReplacedCounterKind,
    ReplacedTokenKind,
    ReturnImperative,
    SacrificeImperative,
    SpecifiedObject,
    SpecifierAndList,
    SpecifierOrList,
    SpecifierOrOfAndList,
    SpellAbility,
    Statement,
    StaticAbility,
    Terminal(TerminalNodeKind),
    TokenCreation,
    TokenCreationReplacement,
    TokenLayout,
    TriggeredAbility,
    TypeLineIdMarker,
    TypeLine(TypeLineNodeKind),
    WardKeywordAbility,
    WrittenOrKeywordAbilty,
    ZoneReferenceIdMarker,
    ZoneReference(crate::ability_tree::zone::ZoneReference),
}

#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MtgDataNodeKind {
    Color,
    ManaIdMarker,
    Mana(mtg_data::Mana),
    ObjectKindIdMarker,
    ObjectKind(crate::ability_tree::object::ObjectKind),
    CardIdMarker,
    PermanentIdMarker,
    SpellIdMarker,
    ArtifactTypeIdMarker,
    BattleTypeIdMarker,
    CreatureTypeIdMarker,
    EnchantmentTypeIdMarker,
    SpellTypeIdMarker,
    LandTypeIdMarker,
    PlaneswalkerTypeIdMarker,
    SupertypeIdMarker,
    CardTypeIdMarker,
}

#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TerminalNodeKind {
    AnotherObjectSpecifier,
    CastSpecifierIdMarker,
    BackwardDurationIdMarker,
    BackwardDuration(crate::ability_tree::time::BackwardDuration),
    CastSpecifier(crate::ability_tree::terminals::CastSpecifier),
    ControlSpecifierIdMarker,
    ControlSpecifier(crate::ability_tree::terminals::ControlSpecifier),
    CounterIdMarker,
    Counter(crate::ability_tree::terminals::Counter),
    CountSpecifierIdMarker,
    CountSpecifier(crate::ability_tree::terminals::CountSpecifier),
    ForwardDurationIdMarker,
    ForwardDuration(crate::ability_tree::time::ForwardDuration),
    InstantIdMarker,
    Instant(crate::ability_tree::time::Instant),
    NamedTokenIdMarker,
    NamedToken(crate::ability_tree::terminals::NamedToken),
    ObjectAttachedTo,
    OrderIdMarker,
    Order(crate::ability_tree::terminals::Order),
    OwnerSpecifierIdMarker,
    OwnerSpecifier(crate::ability_tree::terminals::OwnerSpecifier),
    PlayerSpecifierIdMarker,
    PlayerSpecifier(crate::ability_tree::terminals::PlayerSpecifier),
    PowerToughnessModifierIdMarker,
    PowerToughnessModifier(crate::ability_tree::terminals::PowerToughnessModifier),
    SelfReferencing,
    OwnableZoneIdMarker,
    OwnableZone(crate::ability_tree::zone::OwnableZone),
}

#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeLineNodeKind {
    ArtifactSubtype,
    BattleSubtype,
    ConspiracySubtype,
    CreatureSubtype,
    DungeonSubtype,
    EmblemSubtype,
    EnchantmentSubtype,
    HeroSubtype,
    InstantSubtype,
    KindredSubtype,
    LandSubtype,
    PhenomenonSubtype,
    PlaneSubtype,
    PlaneswalkerSubtype,
    SchemeSubtype,
    SorcerySubtype,
    VanguardSubtype,
}
