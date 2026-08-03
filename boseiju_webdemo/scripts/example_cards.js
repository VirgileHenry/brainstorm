const EXAMPLE_CARDS = [
  {
    name: "Spike Rogue",
    oracle: "this creature enters with two +1/+1 counters on it.\n{2}, remove a +1/+1 counter from this creature: put a +1/+1 counter on target creature.\n{2}, remove a +1/+1 counter from a creature you control: put a +1/+1 counter on this creature.",
  },
  {
    name: "Staff of Domination",
    oracle: "{1}: untap this artifact.\n{2}, {t}: you gain 1 life.\n{3}, {t}: untap target creature.\n{4}, {t}: tap target creature.\n{5}, {t}: draw a card.",
  },
  {
    name: "Greenwarden of Murasa",
    oracle: "when this creature enters, you may return target card from your graveyard to your hand.\nwhen this creature dies, you may exile it. if you do, return target card from your graveyard to your hand.",
  },
  {
    name: "Trigon of Corruption",
    oracle: "this artifact enters with three charge counters on it.\n{b}{b}, {t}: put a charge counter on this artifact.\n{2}, {t}, remove a charge counter from this artifact: put a -1/-1 counter on target creature.",
  },
  {
    name: "Spike Hatcher",
    oracle: "this creature enters with six +1/+1 counters on it.\n{2}, remove a +1/+1 counter from this creature: put a +1/+1 counter on target creature.\n{1}, remove a +1/+1 counter from this creature: regenerate this creature.",
  },
  {
    name: "Golden Egg",
    oracle: "when this artifact enters, draw a card.\n{1}, {t}, sacrifice this artifact: add one mana of any color.\n{2}, {t}, sacrifice this artifact: you gain 3 life.",
  },
  {
    name: "Spike Feeder",
    oracle: "this creature enters with two +1/+1 counters on it.\n{2}, remove a +1/+1 counter from this creature: put a +1/+1 counter on target creature.\nremove a +1/+1 counter from this creature: you gain 2 life.",
  },
  {
    name: "Omni-Cheese Pizza",
    oracle: "when this artifact enters, draw a card.\n{1}, {t}, sacrifice this artifact: add one mana of any color.\n{2}, {t}, sacrifice this artifact: you gain 3 life.",
  },
  {
    name: "Trigon of Thought",
    oracle: "this artifact enters with three charge counters on it.\n{u}{u}, {t}: put a charge counter on this artifact.\n{2}, {t}, remove a charge counter from this artifact: draw a card.",
  },
  {
    name: "Dragon's Hoard",
    oracle: "whenever a dragon you control enters, put a gold counter on this artifact.\n{t}, remove a gold counter from this artifact: draw a card.\n{t}: add one mana of any color.",
  },
  {
    name: "Namazu Trader",
    oracle: "when this creature enters, you lose 1 life and create a treasure token.\nwhenever this creature attacks, you may sacrifice another creature or artifact. if you do, surveil 2.",
  },
  {
    name: "Mikaeus, the Lunarch",
    oracle: "~ enters with x +1/+1 counters on it.\n{t}: put a +1/+1 counter on ~.\n{t}, remove a +1/+1 counter from ~: put a +1/+1 counter on each other creature you control.",
  },
  {
    name: "Clattering Augur",
    oracle: "this creature can't block.\nwhen this creature enters, you draw a card and you lose 1 life.\n{2}{b}{b}: return this card from your graveyard to your hand.",
  },
  {
    name: "Laser Screwdriver",
    oracle: "{t}: add one mana of any color.\n{1}, {t}: tap target artifact.\n{2}, {t}: surveil 1. \n{3}, {t}: goad target creature.",
  },
  {
    name: "Mindless Automaton",
    oracle: "this creature enters with two +1/+1 counters on it.\n{1}, discard a card: put a +1/+1 counter on this creature.\nremove two +1/+1 counters from this creature: draw a card.",
  },
  {
    name: "Merfolk Pupil",
    oracle: "when this creature enters, draw a card, then discard a card.\n{1}{u}, exile this card from your graveyard: draw a card, then discard a card.",
  },
  {
    name: "Bitter Reunion",
    oracle: "when this enchantment enters, you may discard a card. if you do, draw two cards.\n{1}, sacrifice this enchantment: creatures you control gain haste until end of turn.",
  },
  {
    name: "Norn's Wellspring",
    oracle: "whenever a creature you control dies, scry 1 and put an oil counter on this artifact.\n{1}, {t}, remove two oil counters from this artifact: draw a card.",
  },
  {
    name: "Sunset Pyramid",
    oracle: "this artifact enters with three brick counters on it.\n{2}, {t}, remove a brick counter from this artifact: draw a card.\n{2}, {t}: scry 1.",
  },
  {
    name: "Giant's Boulder",
    oracle: "when this artifact enters, scry 2. \n{1}, {t}: add one mana of any color.\n{7}, {t}, sacrifice this artifact: destroy target permanent.",
  },
  {
    name: "Earth King's Lieutenant",
    oracle: "trample\nwhen this creature enters, put a +1/+1 counter on each other ally creature you control.\nwhenever another ally you control enters, put a +1/+1 counter on this creature.",
  },
  {
    name: "Metalspinner's Puzzleknot",
    oracle: "when this artifact enters, you draw a card and you lose 1 life.\n{2}{b}, sacrifice this artifact: you draw a card and you lose 1 life.",
  },
  {
    name: "Memorial to Folly",
    oracle: "this land enters tapped.\n{t}: add {b}.\n{2}{b}, {t}, sacrifice this land: return target creature card from your graveyard to your hand.",
  },
  {
    name: "Master Skald",
    oracle: "when this creature enters, you may exile a creature card from your graveyard. if you do, return target artifact or enchantment card from your graveyard to your hand.",
  },
  {
    name: "Aegis Sculptor",
    oracle: "flying\nward {2} \nat the beginning of your upkeep, you may exile two cards from your graveyard. if you do, put a +1/+1 counter on this creature.",
  },
  {
    name: "Gates of Istfell",
    oracle: "this land enters tapped.\n{t}: add {w}.\n{2}{w}{u}{u}, {t}, sacrifice this land: you gain 2 life and draw two cards.",
  },
  {
    name: "Nyx Weaver",
    oracle: "reach\nat the beginning of your upkeep, mill two cards. \n{1}{b}{g}, exile this creature: return target card from your graveyard to your hand.",
  },
  {
    name: "Nephalia Moondrakes",
    oracle: "flying\nwhen this creature enters, target creature gains flying until end of turn.\n{4}{u}{u}, exile this card from your graveyard: creatures you control gain flying until end of turn.",
  },
  {
    name: "Tendo Ice Bridge",
    oracle: "this land enters with a charge counter on it.\n{t}: add {c}.\n{t}, remove a charge counter from this land: add one mana of any color.",
  },
  {
    name: "Obsessive Stitcher",
    oracle: "{t}: draw a card, then discard a card.\n{2}{u}{b}, {t}, sacrifice this creature: return target creature card from your graveyard to the battlefield.",
  },
  {
    name: "Mirrodin's Core",
    oracle: "{t}: add {c}.\n{t}: put a charge counter on this land.\n{t}, remove a charge counter from this land: add one mana of any color.",
  },
  {
    name: "Elixir of Vitality",
    oracle: "this artifact enters tapped.\n{t}, sacrifice this artifact: you gain 4 life.\n{8}, {t}, sacrifice this artifact: you gain 8 life.",
  },
  {
    name: "Channeler Initiate",
    oracle: "when this creature enters, put three -1/-1 counters on target creature you control.\n{t}, remove a -1/-1 counter from this creature: add one mana of any color.",
  },
  {
    name: "Arcane Spyglass",
    oracle: "{2}, {t}, sacrifice a land: draw a card and put a charge counter on this artifact.\nremove three charge counters from this artifact: draw a card.",
  },
  {
    name: "Faerie Dreamthief",
    oracle: "flying\nwhen this creature enters, surveil 1. \n{2}{b}, exile this card from your graveyard: you draw a card and you lose 1 life.",
  },
  {
    name: "Thalia's Lieutenant",
    oracle: "when this creature enters, put a +1/+1 counter on each other human you control.\nwhenever another human you control enters, put a +1/+1 counter on this creature.",
  },
  {
    name: "Iceberg",
    oracle: "this enchantment enters with x ice counters on it.\n{3}: put an ice counter on this enchantment.\nremove an ice counter from this enchantment: add {c}.",
  },
  {
    name: "Bramble Wurm",
    oracle: "reach, trample\nwhen this creature enters, you gain 5 life.\n{2}{g}, exile this card from your graveyard: you gain 5 life.",
  },
  {
    name: "Omen of the Dead",
    oracle: "flash\nwhen this enchantment enters, return target creature card from your graveyard to your hand.\n{2}{b}, sacrifice this enchantment: scry 2.",
  },
  {
    name: "Seraph Sanctuary",
    oracle: "when this land enters, you gain 1 life.\nwhenever an angel you control enters, you gain 1 life.\n{t}: add {c}.",
  },
  {
    name: "Auriok Survivors",
    oracle: "when this creature enters, you may return target equipment card from your graveyard to the battlefield. if you do, you may attach it to this creature.",
  },
  {
    name: "Golgari Guildmage",
    oracle: "{4}{b}, sacrifice a creature: return target creature card from your graveyard to your hand.\n{4}{g}: put a +1/+1 counter on target creature.",
  },
  {
    name: "Carnifex Demon",
    oracle: "flying\nthis creature enters with two -1/-1 counters on it.\n{b}, remove a -1/-1 counter from this creature: put a -1/-1 counter on each other creature.",
  },
  {
    name: "Druids' Repository",
    oracle: "whenever a creature you control attacks, put a charge counter on this enchantment.\nremove a charge counter from this enchantment: add one mana of any color.",
  },
  {
    name: "Crypt of the Eternals",
    oracle: "when this land enters, you gain 1 life.\n{t}: add {c}.\n{1}, {t}: add {u}, {b}, or {r}.",
  },
  {
    name: "Alquist Proft, Master Sleuth",
    oracle: "vigilance\nwhen ~ enters, investigate. \n{x}{w}{u}{u}, {t}, sacrifice a clue: you draw x cards and gain x life.",
  },
  {
    name: "Spare Supplies",
    oracle: "this artifact enters tapped.\nwhen this artifact enters, draw a card.\n{2}, {t}, sacrifice this artifact: draw a card.",
  },
  {
    name: "Beetle-Headed Merchants",
    oracle: "whenever this creature attacks, you may sacrifice another creature or artifact. if you do, draw a card and put a +1/+1 counter on this creature.",
  },
  {
    name: "Waterfront District",
    oracle: "this land enters tapped.\n{t}: add {u} or {b}.\n{2}{u}{b}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Botanical Plaza",
    oracle: "this land enters tapped.\n{t}: add {g} or {w}.\n{2}{g}{w}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Surge Node",
    oracle: "this artifact enters with six charge counters on it.\n{1}, {t}, remove a charge counter from this artifact: put a charge counter on target artifact.",
  },
  {
    name: "Marauding Looter",
    oracle: "raid — at the beginning of your end step, if you attacked this turn, you may draw a card. if you do, discard a card.",
  },
  {
    name: "Skybridge Towers",
    oracle: "this land enters tapped.\n{t}: add {w} or {u}.\n{2}{w}{u}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Trostani, Three Whispers",
    oracle: "{1}{g}: target creature gains deathtouch until end of turn.\n{g/w}: target creature gains vigilance until end of turn.\n{2}{w}: target creature gains double strike until end of turn.",
  },
  {
    name: "Racers' Ring",
    oracle: "this land enters tapped.\n{t}: add {r} or {g}.\n{2}{r}{g}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Rumble Arena",
    oracle: "vigilance\nwhen this land enters, scry 1. \n{t}: add {c}.\n{1}, {t}: add one mana of any color.",
  },
  {
    name: "Forgotten Harvest",
    oracle: "at the beginning of your upkeep, you may exile a land card from your graveyard. if you do, put a +1/+1 counter on target creature.",
  },
  {
    name: "Elder Cathar",
    oracle: "when this creature dies, put a +1/+1 counter on target creature you control. if that creature is a human, put two +1/+1 counters on it instead.",
  },
  {
    name: "Heirloom Auntie",
    oracle: "this creature enters with two -1/-1 counters on it.\nwhenever another creature you control dies, surveil 1, then remove a -1/-1 counter from this creature.",
  },
  {
    name: "Tramway Station",
    oracle: "this land enters tapped.\n{t}: add {b} or {r}.\n{2}{b}{r}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Reaper of the Wilds",
    oracle: "whenever another creature dies, scry 1.\n{b}: this creature gains deathtouch until end of turn.\n{1}{g}: this creature gains hexproof until end of turn.",
  },
  {
    name: "Shapers of Nature",
    oracle: "{3}{g}: put a +1/+1 counter on target creature.\n{2}{u}, remove a +1/+1 counter from a creature you control: draw a card.",
  },
  {
    name: "Blade of the Bloodchief",
    oracle: "whenever a creature dies, put a +1/+1 counter on equipped creature. if equipped creature is a vampire, put two +1/+1 counters on it instead.\nequip {1}",
  },
  {
    name: "Salt Road Quartermasters",
    oracle: "this creature enters with two +1/+1 counters on it.\n{2}{g}, remove a +1/+1 counter from this creature: put a +1/+1 counter on target creature.",
  },
  {
    name: "Axiom Engraver",
    oracle: "this creature enters with two oil counters on it.\n{t}, remove an oil counter from this creature, discard a card: draw a card.",
  },
  {
    name: "Despoiler of Souls",
    oracle: "this creature can't block.\n{b}{b}, exile two other creature cards from your graveyard: return this card from your graveyard to the battlefield.",
  },
  {
    name: "Airship Engine Room",
    oracle: "this land enters tapped.\n{t}: add {u} or {r}.\n{4}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Spike Colony",
    oracle: "this creature enters with four +1/+1 counters on it.\n{2}, remove a +1/+1 counter from this creature: put a +1/+1 counter on target creature.",
  },
  {
    name: "Conduit Pylons",
    oracle: "when this land enters, surveil 1. \n{t}: add {c}.\n{1}, {t}: add one mana of any color.",
  },
  {
    name: "Hidden Grotto",
    oracle: "when this land enters, surveil 1. \n{t}: add {c}.\n{1}, {t}: add one mana of any color.",
  },
  {
    name: "Feral Thallid",
    oracle: "at the beginning of your upkeep, put a spore counter on this creature.\nremove three spore counters from this creature: regenerate this creature.",
  },
  {
    name: "North Pole Gates",
    oracle: "this land enters tapped.\n{t}: add {w} or {u}.\n{4}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Omen of the Sea",
    oracle: "flash \nwhen this enchantment enters, scry 2, then draw a card.\n{2}{u}, sacrifice this enchantment: scry 2.",
  },
  {
    name: "Undercity Scavenger",
    oracle: "when this creature enters, you may sacrifice another creature. if you do, put two +1/+1 counters on this creature, then scry 2.",
  },
  {
    name: "Meditation Pools",
    oracle: "this land enters tapped.\n{t}: add {g} or {u}.\n{4}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Boiling Rock Prison",
    oracle: "this land enters tapped.\n{t}: add {b} or {r}.\n{4}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Omashu City",
    oracle: "this land enters tapped.\n{t}: add {r} or {g}.\n{4}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Candy Trail",
    oracle: "when this artifact enters, scry 2.\n{2}, {t}, sacrifice this artifact: you gain 3 life and draw a card.",
  },
  {
    name: "Shinewend",
    oracle: "flying\nthis creature enters with a +1/+1 counter on it.\n{1}{w}, remove a +1/+1 counter from this creature: destroy target enchantment.",
  },
  {
    name: "Carrion Cruiser",
    oracle: "when this vehicle enters, mill two cards. then return a creature or vehicle card from your graveyard to your hand. \ncrew 1",
  },
  {
    name: "Vashta Nerada",
    oracle: "indestructible\nshadow \nmorbid — at the beginning of each end step, if a creature died this turn, put a +1/+1 counter on this creature.",
  },
  {
    name: "Infernal Idol",
    oracle: "{t}: add {b}.\n{1}{b}{b}, {t}, sacrifice this artifact: you draw two cards and you lose 2 life.",
  },
  {
    name: "Surveillance Room",
    oracle: "when this land enters, surveil 1. \n{t}: add {c}.\n{1}, {t}: add one mana of any color.",
  },
  {
    name: "Foggy Bottom Swamp",
    oracle: "this land enters tapped.\n{t}: add {b} or {g}.\n{4}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Sun-Blessed Peak",
    oracle: "this land enters tapped.\n{t}: add {r} or {w}.\n{4}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Scrapheap Scrounger",
    oracle: "this creature can't block.\n{1}{b}, exile another creature card from your graveyard: return this card from your graveyard to the battlefield.",
  },
  {
    name: "Spike Worker",
    oracle: "this creature enters with two +1/+1 counters on it.\n{2}, remove a +1/+1 counter from this creature: put a +1/+1 counter on target creature.",
  },
  {
    name: "Kyoshi Village",
    oracle: "this land enters tapped.\n{t}: add {g} or {w}.\n{4}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Spike Drone",
    oracle: "this creature enters with a +1/+1 counter on it.\n{2}, remove a +1/+1 counter from this creature: put a +1/+1 counter on target creature.",
  },
  {
    name: "Misty Palms Oasis",
    oracle: "this land enters tapped.\n{t}: add {w} or {b}.\n{4}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Steelbane Hydra",
    oracle: "this creature enters with x +1/+1 counters on it.\n{2}{g}, remove a +1/+1 counter from this creature: destroy target artifact or enchantment.",
  },
  {
    name: "Stingmoggie",
    oracle: "this creature enters with two +1/+1 counters on it.\n{3}{r}, remove a +1/+1 counter from this creature: destroy target artifact or land.",
  },
  {
    name: "Shipwreck Looter",
    oracle: "raid — when this creature enters, if you attacked this turn, you may draw a card. if you do, discard a card.",
  },
  {
    name: "Crystal Grotto",
    oracle: "when this land enters, scry 1.\n{t}: add {c}.\n{1}, {t}: add one mana of any color.",
  },
  {
    name: "Serpent's Pass",
    oracle: "this land enters tapped.\n{t}: add {u} or {b}.\n{4}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Tumble Magnet",
    oracle: "this artifact enters with three charge counters on it.\n{t}, remove a charge counter from this artifact: tap target artifact or creature.",
  },
  {
    name: "Instant Ramen",
    oracle: "flash\nwhen this artifact enters, draw a card.\n{2}, {t}, sacrifice this artifact: you gain 3 life.",
  },
  {
    name: "Timberland Ruins",
    oracle: "this land enters tapped.\n{t}: add {g}.\n{t}, sacrifice this land: add one mana of any color.",
  },
  {
    name: "Eater of Hope",
    oracle: "flying\n{b}, sacrifice another creature: regenerate this creature.\n{2}{b}, sacrifice two other creatures: destroy target creature.",
  },
  {
    name: "Bladed Ambassador",
    oracle: "this creature enters with an oil counter on it.\n{1}, remove an oil counter from this creature: this creature gains indestructible until end of turn.",
  },
  {
    name: "Buried Ruin",
    oracle: "{t}: add {c}.\n{2}, {t}, sacrifice this land: return target artifact card from your graveyard to your hand.",
  },
  {
    name: "Wickerbough Elder",
    oracle: "this creature enters with a -1/-1 counter on it.\n{g}, remove a -1/-1 counter from this creature: destroy target artifact or enchantment.",
  },
  {
    name: "Abzan Banner",
    oracle: "{t}: add {w}, {b}, or {g}.\n{w}{b}{g}, {t}, sacrifice this artifact: draw a card.",
  },
  {
    name: "Kaleidostone",
    oracle: "when this artifact enters, draw a card.\n{5}, {t}, sacrifice this artifact: add {w}{u}{b}{r}{g}.",
  },
  {
    name: "Abandoned Outpost",
    oracle: "this land enters tapped.\n{t}: add {w}.\n{t}, sacrifice this land: add one mana of any color.",
  },
  {
    name: "Ravaged Highlands",
    oracle: "this land enters tapped.\n{t}: add {r}.\n{t}, sacrifice this land: add one mana of any color.",
  },
  {
    name: "The Dross Pits",
    oracle: "this land enters tapped.\n{t}: add {b}.\n{1}{b}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Memorial to Genius",
    oracle: "this land enters tapped.\n{t}: add {u}.\n{4}{u}, {t}, sacrifice this land: draw two cards.",
  },
  {
    name: "Veilborn Ghoul",
    oracle: "this creature can't block.\nwhenever a swamp you control enters, you may return this card from your graveyard to your hand.",
  },
  {
    name: "Furnace Strider",
    oracle: "this creature enters with two oil counters on it.\nremove an oil counter from this creature: target creature you control gains haste until end of turn.",
  },
  {
    name: "The Fair Basilica",
    oracle: "this land enters tapped.\n{t}: add {w}.\n{1}{w}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Cackling Prowler",
    oracle: "ward {2} \nmorbid — at the beginning of your end step, if a creature died this turn, put a +1/+1 counter on this creature.",
  },
  {
    name: "Temur Banner",
    oracle: "{t}: add {g}, {u}, or {r}.\n{g}{u}{r}, {t}, sacrifice this artifact: draw a card.",
  },
  {
    name: "Sultai Banner",
    oracle: "{t}: add {b}, {g}, or {u}.\n{b}{g}{u}, {t}, sacrifice this artifact: draw a card.",
  },
  {
    name: "Fretwork Colony",
    oracle: "this creature can't block.\nat the beginning of your upkeep, put a +1/+1 counter on this creature and you lose 1 life.",
  },
  {
    name: "Mardu Banner",
    oracle: "{t}: add {r}, {w}, or {b}.\n{r}{w}{b}, {t}, sacrifice this artifact: draw a card.",
  },
  {
    name: "Wintermoon Mesa",
    oracle: "this land enters tapped.\n{t}: add {c}.\n{2}, {t}, sacrifice this land: tap two target lands.",
  },
  {
    name: "Lamplighter of Selhoff",
    oracle: "when this creature enters, if you control another zombie, you may draw a card. if you do, discard a card.",
  },
  {
    name: "Atzocan Seer",
    oracle: "{t}: add one mana of any color.\nsacrifice this creature: return target dinosaur card from your graveyard to your hand.",
  },
  {
    name: "The Hunter Maze",
    oracle: "this land enters tapped.\n{t}: add {g}.\n{1}{g}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Barkhide Troll",
    oracle: "this creature enters with a +1/+1 counter on it.\n{1}, remove a +1/+1 counter from this creature: this creature gains hexproof until end of turn.",
  },
  {
    name: "Bog Wreckage",
    oracle: "this land enters tapped.\n{t}: add {b}.\n{t}, sacrifice this land: add one mana of any color.",
  },
  {
    name: "Deity of Scars",
    oracle: "trample\nthis creature enters with two -1/-1 counters on it.\n{b/g}, remove a -1/-1 counter from this creature: regenerate this creature.",
  },
  {
    name: "Tamiyo's Immobilizer",
    oracle: "this artifact enters with four oil counters on it.\n{t}, remove an oil counter from this artifact: tap target artifact or creature.",
  },
  {
    name: "The Surgical Bay",
    oracle: "this land enters tapped.\n{t}: add {u}.\n{1}{u}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Skywarp Skaab",
    oracle: "flying\nwhen this creature enters, you may exile two creature cards from your graveyard. if you do, draw a card.",
  },
  {
    name: "The Autonomous Furnace",
    oracle: "this land enters tapped.\n{t}: add {r}.\n{1}{r}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Seafloor Debris",
    oracle: "this land enters tapped.\n{t}: add {u}.\n{t}, sacrifice this land: add one mana of any color.",
  },
  {
    name: "Vein Ripper",
    oracle: "flying\nward—sacrifice a creature.\nwhenever a creature dies, target opponent loses 2 life and you gain 2 life.",
  },
  {
    name: "Memorial to War",
    oracle: "this land enters tapped.\n{t}: add {r}.\n{4}{r}, {t}, sacrifice this land: destroy target land.",
  },
  {
    name: "Jeskai Banner",
    oracle: "{t}: add {u}, {r}, or {w}.\n{u}{r}{w}, {t}, sacrifice this artifact: draw a card.",
  },
  {
    name: "Illegitimate Business",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {b} or {g}.",
  },
  {
    name: "Fiery Islet",
    oracle: "{t}, pay 1 life: add {u} or {r}.\n{1}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Kazandu Refuge",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {r} or {g}.",
  },
  {
    name: "Foot Headquarters",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {w} or {b}.",
  },
  {
    name: "Avengers Hangar",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {w} or {u}.",
  },
  {
    name: "Hell's Kitchen",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {b} or {r}.",
  },
  {
    name: "Horizon Canopy",
    oracle: "{t}, pay 1 life: add {g} or {w}.\n{1}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Harvester Troll",
    oracle: "when this creature enters, you may sacrifice a creature or land. if you do, put two +1/+1 counters on this creature.",
  },
  {
    name: "Loxodon Hierarch",
    oracle: "when this creature enters, you gain 4 life.\n{g}{w}, sacrifice this creature: regenerate each creature you control.",
  },
  {
    name: "Swiftwater Cliffs",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {u} or {r}.",
  },
  {
    name: "Akoum Refuge",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {b} or {r}.",
  },
  {
    name: "Crumbling Vestige",
    oracle: "this land enters tapped.\nwhen this land enters, add one mana of any color.\n{t}: add {c}.",
  },
  {
    name: "TCRI Building",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {u} or {r}.",
  },
  {
    name: "Urn of Godfire",
    oracle: "{2}: add one mana of any color.\n{6}, {t}, sacrifice this artifact: destroy target creature or enchantment.",
  },
  {
    name: "Blossoming Sands",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {g} or {w}.",
  },
  {
    name: "Los Diablos Missile Base",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {r} or {g}.",
  },
  {
    name: "Stark Industries",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {u} or {r}.",
  },
  {
    name: "Mnemonic Sphere",
    oracle: "{1}{u}, sacrifice this artifact: draw two cards.\nchannel — {u}, discard this card: draw a card.",
  },
  {
    name: "Cabal Surgeon",
    oracle: "{2}{b}{b}, {t}, exile two cards from your graveyard: return target creature card from your graveyard to your hand.",
  },
  {
    name: "Fisk Tower",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {w} or {b}.",
  },
  {
    name: "Pym Technologies",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {g} or {u}.",
  },
  {
    name: "Tranquil Cove",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {w} or {u}.",
  },
  {
    name: "Graypelt Refuge",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {g} or {w}.",
  },
  {
    name: "Bloodfell Caves",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {b} or {r}.",
  },
  {
    name: "Rugged Highlands",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {r} or {g}.",
  },
  {
    name: "Jungle Hollow",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {b} or {g}.",
  },
  {
    name: "Waterlogged Grove",
    oracle: "{t}, pay 1 life: add {g} or {u}.\n{1}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Sejiri Refuge",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {w} or {u}.",
  },
  {
    name: "Swarm of Bloodflies",
    oracle: "flying\nthis creature enters with two +1/+1 counters on it.\nwhenever another creature dies, put a +1/+1 counter on this creature.",
  },
  {
    name: "A.I.M. Labs",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {u} or {b}.",
  },
  {
    name: "Dismal Backwater",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {u} or {b}.",
  },
  {
    name: "Dimension X",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {r} or {w}.",
  },
  {
    name: "Jwar Isle Refuge",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {u} or {b}.",
  },
  {
    name: "Nurturing Peatland",
    oracle: "{t}, pay 1 life: add {b} or {g}.\n{1}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Devouring Strossus",
    oracle: "flying, trample\nat the beginning of your upkeep, sacrifice a creature.\nsacrifice a creature: regenerate this creature.",
  },
  {
    name: "Morselhoarder",
    oracle: "this creature enters with two -1/-1 counters on it.\nremove a -1/-1 counter from this creature: add one mana of any color.",
  },
  {
    name: "Gemstone Array",
    oracle: "{2}: put a charge counter on this artifact.\nremove a charge counter from this artifact: add one mana of any color.",
  },
  {
    name: "Prognostic Sphinx",
    oracle: "flying\ndiscard a card: this creature gains hexproof until end of turn. tap it.\nwhenever this creature attacks, scry 3.",
  },
  {
    name: "Needletooth Pack",
    oracle: "morbid — at the beginning of your end step, if a creature died this turn, put two +1/+1 counters on target creature you control.",
  },
  {
    name: "Big Wheel",
    oracle: "trample\nwhen this vehicle enters, you may discard a card. if you do, draw a card.\ncrew 2",
  },
  {
    name: "Wind-Scarred Crag",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {r} or {w}.",
  },
  {
    name: "Ghost-Lit Redeemer",
    oracle: "{w}, {t}: you gain 2 life.\nchannel — {1}{w}, discard this card: you gain 4 life.",
  },
  {
    name: "Vampire Soulcaller",
    oracle: "flying\nthis creature can't block.\nwhen this creature enters, return target creature card from your graveyard to your hand.",
  },
  {
    name: "Birnin Zana Plaza",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {g} or {w}.",
  },
  {
    name: "Scoured Barrens",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {w} or {b}.",
  },
  {
    name: "Subterranean Cavern",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {b} or {g}.",
  },
  {
    name: "Asgardian Citadel",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {r} or {w}.",
  },
  {
    name: "Harrier Strix",
    oracle: "flying\nwhen this creature enters, tap target permanent.\n{2}{u}: draw a card, then discard a card.",
  },
  {
    name: "Thornwood Falls",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {g} or {u}.",
  },
  {
    name: "Mutant Town",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 1 life.\n{t}: add {g} or {u}.",
  },
  {
    name: "Sunbaked Canyon",
    oracle: "{t}, pay 1 life: add {r} or {w}.\n{1}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Karametra's Favor",
    oracle: "enchant creature\nwhen this aura enters, draw a card.\nenchanted creature has \"{t}: add one mana of any color.\"",
  },
  {
    name: "Silent Clearing",
    oracle: "{t}, pay 1 life: add {w} or {b}.\n{1}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Ragefire Hellkite",
    oracle: "flying\nwhenever this creature attacks, you may sacrifice another creature. if you do, this creature gains double strike until end of turn.",
  },
  {
    name: "Potion of Healing",
    oracle: "when this artifact enters, draw a card.\n{w}, {t}, sacrifice this artifact: you gain 3 life.",
  },
  {
    name: "Koskun Keep",
    oracle: "{t}: add {c}.\n{1}, {t}: add {r}.\n{2}, {t}: add {b} or {g}.",
  },
  {
    name: "Vaultbreaker",
    oracle: "whenever this creature attacks, you may discard a card. if you do, draw a card.\ndash {2}{r}",
  },
  {
    name: "Spectacle Summit",
    oracle: "this land enters tapped.\n{t}: add {u} or {r}.\n{2}{u}{r}, {t}: surveil 1.",
  },
  {
    name: "Forum of Amity",
    oracle: "this land enters tapped.\n{t}: add {w} or {b}.\n{2}{w}{b}, {t}: surveil 1.",
  },
  {
    name: "Dimir Locket",
    oracle: "{t}: add {u} or {b}.\n{u/b}{u/b}{u/b}{u/b}, {t}, sacrifice this artifact: draw two cards.",
  },
  {
    name: "Visions of Villainy",
    oracle: "this spell costs {1} less to cast if you control a villain.\nyou draw two cards and lose 2 life.",
  },
  {
    name: "Compulsion",
    oracle: "{1}{u}, discard a card: draw a card.\n{1}{u}, sacrifice this enchantment: draw a card.",
  },
  {
    name: "An-Havva Township",
    oracle: "{t}: add {c}.\n{1}, {t}: add {g}.\n{2}, {t}: add {r} or {w}.",
  },
  {
    name: "Wizards' School",
    oracle: "{t}: add {c}.\n{1}, {t}: add {u}.\n{2}, {t}: add {w} or {b}.",
  },
  {
    name: "Skyswimmer Koi",
    oracle: "flying\nwhenever an artifact you control enters, you may draw a card. if you do, discard a card.",
  },
  {
    name: "Titan's Grave",
    oracle: "this land enters tapped.\n{t}: add {b} or {g}.\n{2}{b}{g}, {t}: surveil 1.",
  },
  {
    name: "Merfolk Skydiver",
    oracle: "flying\nwhen this creature enters, put a +1/+1 counter on target creature you control.\n{3}{g}{u}: proliferate.",
  },
  {
    name: "Anchovy & Banana Pizza",
    oracle: "when this artifact enters, destroy target creature.\n{2}, {t}, sacrifice this artifact: you gain 3 life.",
  },
  {
    name: "Paradox Gardens",
    oracle: "this land enters tapped.\n{t}: add {g} or {u}.\n{2}{g}{u}, {t}: surveil 1.",
  },
  {
    name: "Aysen Abbey",
    oracle: "{t}: add {c}.\n{1}, {t}: add {w}.\n{2}, {t}: add {g} or {u}.",
  },
  {
    name: "Gruul Locket",
    oracle: "{t}: add {r} or {g}.\n{r/g}{r/g}{r/g}{r/g}, {t}, sacrifice this artifact: draw two cards.",
  },
  {
    name: "Senate Guildmage",
    oracle: "{w}, {t}: you gain 2 life.\n{u}, {t}: draw a card, then discard a card.",
  },
  {
    name: "Rustvine Cultivator",
    oracle: "{t}: put an oil counter on this creature.\n{t}, remove an oil counter from this creature: untap target land.",
  },
  {
    name: "Kill-Zone Acrobat",
    oracle: "whenever this creature attacks, you may sacrifice another creature or artifact. if you do, this creature gains flying until end of turn.",
  },
  {
    name: "Izzet Locket",
    oracle: "{t}: add {u} or {r}.\n{u/r}{u/r}{u/r}{u/r}, {t}, sacrifice this artifact: draw two cards.",
  },
  {
    name: "Venom's Hunger",
    oracle: "this spell costs {2} less to cast if you control a villain.\ndestroy target creature. you gain 2 life.",
  },
  {
    name: "Orzhov Locket",
    oracle: "{t}: add {w} or {b}.\n{w/b}{w/b}{w/b}{w/b}, {t}, sacrifice this artifact: draw two cards.",
  },
  {
    name: "Blessing of Leeches",
    oracle: "flash\nenchant creature\nat the beginning of your upkeep, you lose 1 life.\n{0}: regenerate enchanted creature.",
  },
  {
    name: "Pendulum of Patterns",
    oracle: "when this artifact enters, you gain 3 life.\n{5}, {t}, sacrifice this artifact: draw a card.",
  },
  {
    name: "Molt Tender",
    oracle: "{t}: mill a card. \n{t}, exile a card from your graveyard: add one mana of any color.",
  },
  {
    name: "Mistmeadow Council",
    oracle: "this spell costs {1} less to cast if you control a kithkin.\nwhen this creature enters, draw a card.",
  },
  {
    name: "Salvage Drone",
    oracle: "devoid \ningest \nwhen this creature dies, you may draw a card. if you do, discard a card.",
  },
  {
    name: "Selesnya Locket",
    oracle: "{t}: add {g} or {w}.\n{g/w}{g/w}{g/w}{g/w}, {t}, sacrifice this artifact: draw two cards.",
  },
  {
    name: "Fields of Strife",
    oracle: "this land enters tapped.\n{t}: add {r} or {w}.\n{2}{r}{w}, {t}: surveil 1.",
  },
  {
    name: "Soaring Seacliff",
    oracle: "this land enters tapped.\nwhen this land enters, target creature gains flying until end of turn.\n{t}: add {u}.",
  },
  {
    name: "Action News Crew",
    oracle: "vigilance\nchannel — {6}, discard this card: put a +1/+1 counter on each creature you control. draw a card.",
  },
  {
    name: "Rakdos Locket",
    oracle: "{t}: add {b} or {r}.\n{b/r}{b/r}{b/r}{b/r}, {t}, sacrifice this artifact: draw two cards.",
  },
  {
    name: "Boros Locket",
    oracle: "{t}: add {r} or {w}.\n{r/w}{r/w}{r/w}{r/w}, {t}, sacrifice this artifact: draw two cards.",
  },
  {
    name: "Shoal Kraken",
    oracle: "constellation — whenever an enchantment you control enters, you may draw a card. if you do, discard a card.",
  },
  {
    name: "Petrified Field",
    oracle: "{t}: add {c}.\n{t}, sacrifice this land: return target land card from your graveyard to your hand.",
  },
  {
    name: "Simic Locket",
    oracle: "{t}: add {g} or {u}.\n{g/u}{g/u}{g/u}{g/u}, {t}, sacrifice this artifact: draw two cards.",
  },
  {
    name: "Rook Turret",
    oracle: "flying\nwhenever another artifact you control enters, you may draw a card. if you do, discard a card.",
  },
  {
    name: "Angel of Glory's Rise",
    oracle: "flying\nwhen this creature enters, exile all zombies, then return all human creature cards from your graveyard to the battlefield.",
  },
  {
    name: "Castle Sengir",
    oracle: "{t}: add {c}.\n{1}, {t}: add {b}.\n{2}, {t}: add {u} or {r}.",
  },
  {
    name: "Operations Officer",
    oracle: "lifelink \nwhen this creature enters, draw a card.\nwhenever this creature attacks, put a +1/+1 counter on it.",
  },
  {
    name: "Azorius Locket",
    oracle: "{t}: add {w} or {u}.\n{w/u}{w/u}{w/u}{w/u}, {t}, sacrifice this artifact: draw two cards.",
  },
  {
    name: "Sabertooth Mauler",
    oracle: "at the beginning of your end step, if a creature died this turn, put a +1/+1 counter on this creature and untap it.",
  },
  {
    name: "Prosperous Innkeeper",
    oracle: "when this creature enters, create a treasure token. \nwhenever another creature you control enters, you gain 1 life.",
  },
  {
    name: "Golgari Locket",
    oracle: "{t}: add {b} or {g}.\n{b/g}{b/g}{b/g}{b/g}, {t}, sacrifice this artifact: draw two cards.",
  },
  {
    name: "Fountain of Renewal",
    oracle: "at the beginning of your upkeep, you gain 1 life.\n{3}, sacrifice this artifact: draw a card.",
  },
  {
    name: "Lux Cannon",
    oracle: "{t}: put a charge counter on this artifact.\n{t}, remove three charge counters from this artifact: destroy target permanent.",
  },
  {
    name: "Veloheart Bike",
    oracle: "when this vehicle enters, you gain 2 life.\n{t}: add one mana of any color.\ncrew 2",
  },
  {
    name: "Sunbeam Spellbomb",
    oracle: "{w}, sacrifice this artifact: you gain 5 life.\n{1}, sacrifice this artifact: draw a card.",
  },
  {
    name: "Havenwood Battleground",
    oracle: "this land enters tapped.\n{t}: add {g}.\n{t}, sacrifice this land: add {g}{g}.",
  },
  {
    name: "Blight Keeper",
    oracle: "flying\n{7}{b}, {t}, sacrifice this creature: target opponent loses 4 life and you gain 4 life.",
  },
  {
    name: "Ancient Spring",
    oracle: "this land enters tapped.\n{t}: add {u}.\n{t}, sacrifice this land: add {w}{b}.",
  },
  {
    name: "Felidar Umbra",
    oracle: "enchant creature\nenchanted creature has lifelink.\n{1}{w}: attach this aura to target creature you control.\numbra armor",
  },
  {
    name: "Temple of Deceit",
    oracle: "this land enters tapped.\nwhen this land enters, scry 1. \n{t}: add {u} or {b}.",
  },
  {
    name: "Temple of Abandon",
    oracle: "this land enters tapped.\nwhen this land enters, scry 1. \n{t}: add {r} or {g}.",
  },
  {
    name: "Unwilling Ingredient",
    oracle: "menace \n{2}{b}, exile this card from your graveyard: you draw a card and you lose 1 life.",
  },
  {
    name: "Seraph of the Scales",
    oracle: "flying\n{w}: this creature gains vigilance until end of turn.\n{b}: this creature gains deathtouch until end of turn.\nafterlife 2",
  },
  {
    name: "Ebon Stronghold",
    oracle: "this land enters tapped.\n{t}: add {b}.\n{t}, sacrifice this land: add {b}{b}.",
  },
  {
    name: "Hierophant's Chalice",
    oracle: "when this artifact enters, target opponent loses 1 life and you gain 1 life.\n{t}: add {c}.",
  },
  {
    name: "Svyelunite Temple",
    oracle: "this land enters tapped.\n{t}: add {u}.\n{t}, sacrifice this land: add {u}{u}.",
  },
  {
    name: "Karstoderm",
    oracle: "this creature enters with five +1/+1 counters on it.\nwhenever an artifact enters, remove a +1/+1 counter from this creature.",
  },
  {
    name: "Temple of Triumph",
    oracle: "this land enters tapped.\nwhen this land enters, scry 1. \n{t}: add {r} or {w}.",
  },
  {
    name: "Hope Tender",
    oracle: "{1}, {t}: untap target land.\n{1}, {t}, exert this creature: untap two target lands.",
  },
  {
    name: "Wort, Boggart Auntie",
    oracle: "fear \nat the beginning of your upkeep, you may return target goblin card from your graveyard to your hand.",
  },
  {
    name: "Temple of Epiphany",
    oracle: "this land enters tapped.\nwhen this land enters, scry 1. \n{t}: add {u} or {r}.",
  },
  {
    name: "Temple of Malice",
    oracle: "this land enters tapped.\nwhen this land enters, scry 1. \n{t}: add {b} or {r}.",
  },
  {
    name: "Temple of Mystery",
    oracle: "this land enters tapped.\nwhen this land enters, scry 1. \n{t}: add {g} or {u}.",
  },
  {
    name: "Geothermal Crevice",
    oracle: "this land enters tapped.\n{t}: add {r}.\n{t}, sacrifice this land: add {b}{g}.",
  },
  {
    name: "Temple of Enlightenment",
    oracle: "this land enters tapped.\nwhen this land enters, scry 1. \n{t}: add {w} or {u}.",
  },
  {
    name: "Ruins of Trokair",
    oracle: "this land enters tapped.\n{t}: add {w}.\n{t}, sacrifice this land: add {w}{w}.",
  },
  {
    name: "Tinder Farm",
    oracle: "this land enters tapped.\n{t}: add {g}.\n{t}, sacrifice this land: add {r}{w}.",
  },
  {
    name: "Kabira Crossroads",
    oracle: "this land enters tapped.\nwhen this land enters, you gain 2 life.\n{t}: add {w}.",
  },
  {
    name: "Sulfur Vent",
    oracle: "this land enters tapped.\n{t}: add {b}.\n{t}, sacrifice this land: add {u}{r}.",
  },
  {
    name: "Glistener Seer",
    oracle: "this creature enters with three oil counters on it.\n{t}, remove an oil counter from this creature: scry 1.",
  },
  {
    name: "Ethersworn Adjudicator",
    oracle: "flying\n{1}{w}{b}, {t}: destroy target creature or enchantment.\n{2}{u}: untap this creature.",
  },
  {
    name: "Dwarven Ruins",
    oracle: "this land enters tapped.\n{t}: add {r}.\n{t}, sacrifice this land: add {r}{r}.",
  },
  {
    name: "Pterafractyl",
    oracle: "flying\nthis creature enters with x +1/+1 counters on it.\nwhen this creature enters, you gain 2 life.",
  },
  {
    name: "Pelakka Wurm",
    oracle: "trample \nwhen this creature enters, you gain 7 life.\nwhen this creature dies, draw a card.",
  },
  {
    name: "Temple of Malady",
    oracle: "this land enters tapped.\nwhen this land enters, scry 1. \n{t}: add {b} or {g}.",
  },
  {
    name: "Irrigation Ditch",
    oracle: "this land enters tapped.\n{t}: add {w}.\n{t}, sacrifice this land: add {g}{u}.",
  },
  {
    name: "Letter of Acceptance",
    oracle: "{t}: add one mana of any color.\n{2}, {t}, sacrifice this artifact: draw a card.",
  },
  {
    name: "Temple of Plenty",
    oracle: "this land enters tapped.\nwhen this land enters, scry 1. \n{t}: add {g} or {w}.",
  },
  {
    name: "Temple of Silence",
    oracle: "this land enters tapped.\nwhen this land enters, scry 1. \n{t}: add {w} or {b}.",
  },
  {
    name: "Bloodline Necromancer",
    oracle: "lifelink\nwhen this creature enters, you may return target vampire or wizard creature card from your graveyard to the battlefield.",
  },
  {
    name: "Sinister Concoction",
    oracle: "{b}, pay 1 life, mill a card, discard a card, sacrifice this enchantment: destroy target creature.",
  },
  {
    name: "Ruthless Knave",
    oracle: "{2}{b}, sacrifice a creature: create two treasure tokens. \nsacrifice three treasures: draw a card.",
  },
  {
    name: "Chainbreaker",
    oracle: "this creature enters with two -1/-1 counters on it.\n{3}, {t}: remove a -1/-1 counter from target creature.",
  },
  {
    name: "Reya Dawnbringer",
    oracle: "flying\nat the beginning of your upkeep, you may return target creature card from your graveyard to the battlefield.",
  },
  {
    name: "Orzhov Cluestone",
    oracle: "{t}: add {w} or {b}.\n{w}{b}, {t}, sacrifice this artifact: draw a card.",
  },
  {
    name: "Simic Cluestone",
    oracle: "{t}: add {g} or {u}.\n{g}{u}, {t}, sacrifice this artifact: draw a card.",
  },
  {
    name: "Futurist Forge",
    oracle: "when this artifact enters, draw a card.\n{3}{u}, sacrifice this artifact: draw two cards.",
  },
  {
    name: "Strands of Night",
    oracle: "{b}{b}, pay 2 life, sacrifice a swamp: return target creature card from your graveyard to the battlefield.",
  },
  {
    name: "Prophetic Prism",
    oracle: "when this artifact enters, draw a card.\n{1}, {t}: add one mana of any color.",
  },
  {
    name: "Quandrix Campus",
    oracle: "this land enters tapped.\n{t}: add {g} or {u}.\n{4}, {t}: scry 1.",
  },
  {
    name: "Savage Mansion",
    oracle: "this land enters tapped.\n{t}: add {r} or {g}.\n{4}, {t}: surveil 1.",
  },
  {
    name: "Azorius Cluestone",
    oracle: "{t}: add {w} or {u}.\n{w}{u}, {t}, sacrifice this artifact: draw a card.",
  },
  {
    name: "Silverquill Campus",
    oracle: "this land enters tapped.\n{t}: add {w} or {b}.\n{4}, {t}: scry 1.",
  },
  {
    name: "Villainous Ogre",
    oracle: "this creature can't block.\nas long as you control a demon, this creature has \"{b}: regenerate this creature.\"",
  },
  {
    name: "Prismari Campus",
    oracle: "this land enters tapped.\n{t}: add {u} or {r}.\n{4}, {t}: scry 1.",
  },
  {
    name: "Izzet Cluestone",
    oracle: "{t}: add {u} or {r}.\n{u}{r}, {t}, sacrifice this artifact: draw a card.",
  },
  {
    name: "Fetid Heath",
    oracle: "{t}: add {c}.\n{w/b}, {t}: add {w}{w}, {w}{b}, or {b}{b}.",
  },
  {
    name: "Lorehold Campus",
    oracle: "this land enters tapped.\n{t}: add {r} or {w}.\n{4}, {t}: scry 1.",
  },
  {
    name: "Vengeful Bloodwitch",
    oracle: "whenever this creature or another creature you control dies, target opponent loses 1 life and you gain 1 life.",
  },
  {
    name: "Graven Cairns",
    oracle: "{t}: add {c}.\n{b/r}, {t}: add {b}{b}, {b}{r}, or {r}{r}.",
  },
  {
    name: "Burning-Tree Vandal",
    oracle: "riot \nwhenever this creature attacks, you may discard a card. if you do, draw a card.",
  },
  {
    name: "Lotleth Troll",
    oracle: "trample\ndiscard a creature card: put a +1/+1 counter on this creature.\n{b}: regenerate this creature.",
  },
  {
    name: "Wooded Bastion",
    oracle: "{t}: add {c}.\n{g/w}, {t}: add {g}{g}, {g}{w}, or {w}{w}.",
  },
  {
    name: "Glorifier of Dusk",
    oracle: "pay 2 life: this creature gains flying until end of turn.\npay 2 life: this creature gains vigilance until end of turn.",
  },
  {
    name: "Dimir Cluestone",
    oracle: "{t}: add {u} or {b}.\n{u}{b}, {t}, sacrifice this artifact: draw a card.",
  },
  {
    name: "Yuyan Archers",
    oracle: "reach\nwhen this creature enters, you may discard a card. if you do, draw a card.",
  },
  {
    name: "Coretapper",
    oracle: "{t}: put a charge counter on target artifact.\nsacrifice this creature: put two charge counters on target artifact.",
  },
  {
    name: "Ominous Asylum",
    oracle: "this land enters tapped.\n{t}: add {b} or {r}.\n{4}, {t}: surveil 1.",
  },
  {
    name: "Cliffhaven Kitesail",
    oracle: "when this equipment enters, attach it to target creature you control.\nequipped creature has flying.\nequip {2}",
  },
  {
    name: "Witherbloom Campus",
    oracle: "this land enters tapped.\n{t}: add {b} or {g}.\n{4}, {t}: scry 1.",
  },
  {
    name: "Suburban Sanctuary",
    oracle: "this land enters tapped.\n{t}: add {g} or {w}.\n{4}, {t}: surveil 1.",
  },
  {
    name: "Fearless Fledgling",
    oracle: "landfall — whenever a land you control enters, put a +1/+1 counter on this creature. it gains flying until end of turn.",
  },
  {
    name: "Selesnya Cluestone",
    oracle: "{t}: add {g} or {w}.\n{g}{w}, {t}, sacrifice this artifact: draw a card.",
  },
  {
    name: "Gruul Cluestone",
    oracle: "{t}: add {r} or {g}.\n{r}{g}, {t}, sacrifice this artifact: draw a card.",
  },
  {
    name: "Sinister Hideout",
    oracle: "this land enters tapped.\n{t}: add {u} or {b}.\n{4}, {t}: surveil 1.",
  },
  {
    name: "Gavony Township",
    oracle: "{t}: add {c}.\n{2}{g}{w}, {t}: put a +1/+1 counter on each creature you control.",
  },
  {
    name: "Rugged Prairie",
    oracle: "{t}: add {c}.\n{r/w}, {t}: add {r}{r}, {r}{w}, or {w}{w}.",
  },
  {
    name: "Bant Battlemage",
    oracle: "{g}, {t}: target creature gains trample until end of turn.\n{u}, {t}: target creature gains flying until end of turn.",
  },
  {
    name: "University Campus",
    oracle: "this land enters tapped.\n{t}: add {w} or {u}.\n{4}, {t}: surveil 1.",
  },
  {
    name: "Desolate Lighthouse",
    oracle: "{t}: add {c}.\n{1}{u}{r}, {t}: draw a card, then discard a card.",
  },
  {
    name: "Deathreap Ritual",
    oracle: "morbid — at the beginning of each end step, if a creature died this turn, you may draw a card.",
  },
  {
    name: "Rakdos Cluestone",
    oracle: "{t}: add {b} or {r}.\n{b}{r}, {t}, sacrifice this artifact: draw a card.",
  },
  {
    name: "Murder of Crows",
    oracle: "flying\nwhenever another creature dies, you may draw a card. if you do, discard a card.",
  },
  {
    name: "Quicksmith Genius",
    oracle: "whenever an artifact you control enters, you may discard a card. if you do, draw a card.",
  },
  {
    name: "Mechanical Glider",
    oracle: "when this equipment enters, attach it to target creature you control.\nequipped creature has flying. \nequip {2}",
  },
  {
    name: "Fire-Lit Thicket",
    oracle: "{t}: add {c}.\n{r/g}, {t}: add {r}{r}, {r}{g}, or {g}{g}.",
  },
  {
    name: "Twilight Mire",
    oracle: "{t}: add {c}.\n{b/g}, {t}: add {b}{b}, {b}{g}, or {g}{g}.",
  },
  {
    name: "Flooded Grove",
    oracle: "{t}: add {c}.\n{g/u}, {t}: add {g}{g}, {g}{u}, or {u}{u}.",
  },
  {
    name: "Plundering Predator",
    oracle: "flying\nwhen this creature enters, you may discard a card. if you do, draw a card.",
  },
  {
    name: "Sunken Ruins",
    oracle: "{t}: add {c}.\n{u/b}, {t}: add {u}{u}, {u}{b}, or {b}{b}.",
  },
  {
    name: "Golgari Cluestone",
    oracle: "{t}: add {b} or {g}.\n{b}{g}, {t}, sacrifice this artifact: draw a card.",
  },
  {
    name: "Mystic Gate",
    oracle: "{t}: add {c}.\n{w/u}, {t}: add {w}{w}, {w}{u}, or {u}{u}.",
  },
  {
    name: "Arcum's Astrolabe",
    oracle: "when this artifact enters, draw a card.\n{1}, {t}: add one mana of any color.",
  },
  {
    name: "Cascade Bluffs",
    oracle: "{t}: add {c}.\n{u/r}, {t}: add {u}{u}, {u}{r}, or {r}{r}.",
  },
  {
    name: "Boros Cluestone",
    oracle: "{t}: add {r} or {w}.\n{r}{w}, {t}, sacrifice this artifact: draw a card.",
  },
  {
    name: "Pharika's Mender",
    oracle: "when this creature enters, you may return target creature or enchantment card from your graveyard to your hand.",
  },
  {
    name: "Vito's Inquisitor",
    oracle: "{b}, sacrifice another creature or artifact: put a +1/+1 counter on this creature. it gains menace until end of turn.",
  },
  {
    name: "Dining Room",
    oracle: "this land enters tapped.\n{t}: add {r} or {g}.\n{4}, {t}: investigate.",
  },
  {
    name: "Dreamstone Hedron",
    oracle: "{t}: add {c}{c}{c}.\n{3}, {t}, sacrifice this artifact: draw three cards.",
  },
  {
    name: "Graveshifter",
    oracle: "changeling \nwhen this creature enters, you may return target creature card from your graveyard to your hand.",
  },
  {
    name: "Oasis Gardener",
    oracle: "when this creature enters, you gain 2 life.\n{t}: add one mana of any color.",
  },
  {
    name: "Eldrazi Ravager",
    oracle: "annihilator 1 \nsacrifice two eldrazi: return this card from your graveyard to your hand.\ncycling {2}",
  },
  {
    name: "Ulvenwald Bear",
    oracle: "morbid — when this creature enters, if a creature died this turn, put two +1/+1 counters on target creature.",
  },
  {
    name: "Capital City",
    oracle: "{t}: add {c}.\n{1}, {t}: add one mana of any color.\ncycling {2}",
  },
  {
    name: "Resolute Rider",
    oracle: "{w/b}{w/b}: this creature gains lifelink until end of turn.\n{w/b}{w/b}{w/b}: this creature gains indestructible until end of turn.",
  },
  {
    name: "Hopeful Initiate",
    oracle: "training \n{2}{w}, remove two +1/+1 counters from among creatures you control: destroy target artifact or enchantment.",
  },
  {
    name: "Library",
    oracle: "this land enters tapped.\n{t}: add {u} or {r}.\n{4}, {t}: investigate.",
  },
  {
    name: "Setessan Champion",
    oracle: "constellation — whenever an enchantment you control enters, put a +1/+1 counter on this creature and draw a card.",
  },
  {
    name: "Treasure Vault",
    oracle: "{t}: add {c}.\n{x}{x}, {t}, sacrifice this land: create x treasure tokens.",
  },
  {
    name: "Kitchen",
    oracle: "this land enters tapped.\n{t}: add {g} or {u}.\n{4}, {t}: investigate.",
  },
  {
    name: "Bulette",
    oracle: "at the beginning of your end step, if a creature died this turn, put a +1/+1 counter on this creature.",
  },
  {
    name: "Dreadmobile",
    oracle: "menace\n{1}, sacrifice another artifact or creature: put a +1/+1 counter on this vehicle.\ncrew 1",
  },
  {
    name: "Shadewing Laureate",
    oracle: "flying\nwhenever another creature you control with flying dies, put a +1/+1 counter on target creature you control.",
  },
  {
    name: "Frog Butler",
    oracle: "deathtouch\n{t}: add one mana of any color.\n{2}: this creature gains reach until end of turn.",
  },
  {
    name: "Grim Backwoods",
    oracle: "{t}: add {c}.\n{2}{b}{g}, {t}, sacrifice a creature: draw a card.",
  },
  {
    name: "Ghen, Arcanum Weaver",
    oracle: "{r}{w}{b}, {t}, sacrifice an enchantment: return target enchantment card from your graveyard to the battlefield.",
  },
  {
    name: "New Benalia",
    oracle: "this land enters tapped.\nwhen this land enters, scry 1. \n{t}: add {w}.",
  },
  {
    name: "Sanctum Gargoyle",
    oracle: "flying\nwhen this creature enters, you may return target artifact card from your graveyard to your hand.",
  },
  {
    name: "Cadaver Imp",
    oracle: "flying\nwhen this creature enters, you may return target creature card from your graveyard to your hand.",
  },
  {
    name: "Reckless Lackey",
    oracle: "first strike, haste\n{2}{r}, sacrifice this creature: draw a card and create a treasure token.",
  },
  {
    name: "Study",
    oracle: "this land enters tapped.\n{t}: add {w} or {u}.\n{4}, {t}: investigate.",
  },
  {
    name: "Sparring Collar",
    oracle: "equipped creature has first strike.\n{r}{r}: attach this equipment to target creature you control.\nequip {1}",
  },
  {
    name: "Conservatory",
    oracle: "this land enters tapped.\n{t}: add {g} or {w}.\n{4}, {t}: investigate.",
  },
  {
    name: "Hall",
    oracle: "this land enters tapped.\n{t}: add {r} or {w}.\n{4}, {t}: investigate.",
  },
  {
    name: "Ironclad Slayer",
    oracle: "when this creature enters, you may return target aura or equipment card from your graveyard to your hand.",
  },
  {
    name: "Lounge",
    oracle: "this land enters tapped.\n{t}: add {b} or {g}.\n{4}, {t}: investigate.",
  },
  {
    name: "Angel of Flight Alabaster",
    oracle: "flying\nat the beginning of your upkeep, return target spirit card from your graveyard to your hand.",
  },
  {
    name: "Billiard Room",
    oracle: "this land enters tapped.\n{t}: add {b} or {r}.\n{4}, {t}: investigate.",
  },
  {
    name: "Kraul Swarm",
    oracle: "flying\n{2}{b}, discard a creature card: return this card from your graveyard to your hand.",
  },
  {
    name: "Filigree Familiar",
    oracle: "when this creature enters, you gain 2 life.\nwhen this creature dies, draw a card.",
  },
  {
    name: "Witching Well",
    oracle: "when this artifact enters, scry 2. \n{3}{u}, sacrifice this artifact: draw two cards.",
  },
  {
    name: "Workhorse",
    oracle: "this creature enters with four +1/+1 counters on it.\nremove a +1/+1 counter from this creature: add {c}.",
  },
  {
    name: "Expedition Diviner",
    oracle: "flying\nas long as you control another wizard, this creature has \"when this creature dies, draw a card.\"",
  },
  {
    name: "Undead Augur",
    oracle: "whenever this creature or another zombie you control dies, you draw a card and you lose 1 life.",
  },
  {
    name: "Winged Words",
    oracle: "this spell costs {1} less to cast if you control a creature with flying.\ndraw two cards.",
  },
  {
    name: "Secret Passage",
    oracle: "this land enters tapped.\n{t}: add {u} or {b}.\n{4}, {t}: investigate.",
  },
  {
    name: "Dawnhart Rejuvenator",
    oracle: "when this creature enters, you gain 3 life.\n{t}: add one mana of any color.",
  },
  {
    name: "Ballroom",
    oracle: "this land enters tapped.\n{t}: add {w} or {b}.\n{4}, {t}: investigate.",
  },
  {
    name: "Stone-Seeder Hierophant",
    oracle: "landfall — whenever a land you control enters, untap this creature.\n{t}: untap target land.",
  },
  {
    name: "Neurok Stealthsuit",
    oracle: "equipped creature has shroud. \n{u}{u}: attach this equipment to target creature you control.\nequip {1}",
  },
  {
    name: "Aron, Benalia's Ruin",
    oracle: "menace \n{w}{b}, {t}, sacrifice another creature: put a +1/+1 counter on each creature you control.",
  },
  {
    name: "Poison Dart Frog",
    oracle: "reach\n{t}: add one mana of any color.\n{2}: this creature gains deathtouch until end of turn.",
  },
  {
    name: "Centaur Nurturer",
    oracle: "when this creature enters, you gain 3 life.\n{t}: add one mana of any color.",
  },
  {
    name: "Keldon Raider",
    oracle: "when this creature enters, you may discard a card. if you do, draw a card.",
  },
  {
    name: "Squee, Goblin Nabob",
    oracle: "at the beginning of your upkeep, you may return this card from your graveyard to your hand.",
  },
  {
    name: "Merciless Harlequin",
    oracle: "freerunning {1}{b} \nwhen this creature enters, you draw a card and you lose 1 life.",
  },
  {
    name: "Viashino Racketeer",
    oracle: "when this creature enters, you may discard a card. if you do, draw a card.",
  },
  {
    name: "Protomatter Powder",
    oracle: "{4}{w}, {t}, sacrifice this artifact: return target artifact card from your graveyard to the battlefield.",
  },
  {
    name: "Silent Sentinel",
    oracle: "flying\nwhenever this creature attacks, you may return target enchantment card from your graveyard to the battlefield.",
  },
  {
    name: "Boros Guildmage",
    oracle: "{1}{r}: target creature gains haste until end of turn.\n{1}{w}: target creature gains first strike until end of turn.",
  },
  {
    name: "Chromatic Sphere",
    oracle: "{1}, {t}, sacrifice this artifact: add one mana of any color. draw a card.",
  },
  {
    name: "Hedron Archive",
    oracle: "{t}: add {c}{c}.\n{2}, {t}, sacrifice this artifact: draw two cards.",
  },
  {
    name: "Archon of Falling Stars",
    oracle: "flying\nwhen this creature dies, you may return target enchantment card from your graveyard to the battlefield.",
  },
  {
    name: "Hanna, Ship's Navigator",
    oracle: "{1}{w}{u}, {t}: return target artifact or enchantment card from your graveyard to your hand.",
  },
  {
    name: "Tender Wildguide",
    oracle: "{t}: add one mana of any color.\n{t}: put a +1/+1 counter on this creature.",
  },
  {
    name: "Contagion Clasp",
    oracle: "when this artifact enters, put a -1/-1 counter on target creature.\n{4}, {t}: proliferate.",
  },
  {
    name: "Seaside Haven",
    oracle: "{t}: add {c}.\n{w}{u}, {t}, sacrifice a bird: draw a card.",
  },
  {
    name: "Blighted Cataract",
    oracle: "{t}: add {c}.\n{5}{u}, {t}, sacrifice this land: draw two cards.",
  },
  {
    name: "Redrock Sentinel",
    oracle: "defender\n{2}, {t}, sacrifice a land: draw a card and create a treasure token.",
  },
  {
    name: "Malevolent Awakening",
    oracle: "{1}{b}{b}, sacrifice a creature: return target creature card from your graveyard to your hand.",
  },
  {
    name: "Treetop Sentries",
    oracle: "reach\nwhen this creature enters, you may forage. if you do, draw a card.",
  },
  {
    name: "Predator, Flagship",
    oracle: "{2}: target creature gains flying until end of turn.\n{5}, {t}: destroy target creature with flying.",
  },
  {
    name: "Bamboo Grove Archer",
    oracle: "defender, reach\nchannel — {4}{g}, discard this card: destroy target creature with flying.",
  },
  {
    name: "Common Iguana",
    oracle: "when this creature enters, you may discard a card. if you do, draw a card.",
  },
  {
    name: "Gilded Assault Cart",
    oracle: "trample\ncrew 2 \nsacrifice two treasures: return this card from your graveyard to your hand.",
  },
  {
    name: "Geralf's Messenger",
    oracle: "this creature enters tapped.\nwhen this creature enters, target opponent loses 2 life.\nundying",
  },
  {
    name: "Death-Hood Cobra",
    oracle: "{1}{g}: this creature gains reach until end of turn.\n{1}{g}: this creature gains deathtouch until end of turn.",
  },
  {
    name: "Light of the Legion",
    oracle: "flying\nmentor \nwhen this creature dies, put a +1/+1 counter on each white creature you control.",
  },
  {
    name: "Bushy Bodyguard",
    oracle: "when this creature enters, you may forage. if you do, put two +1/+1 counters on it.",
  },
  {
    name: "Maestros Initiate",
    oracle: "{4}{u/r}, exile this card from your graveyard: draw two cards, then discard a card.",
  },
  {
    name: "Ovalchase Daredevil",
    oracle: "whenever an artifact you control enters, you may return this card from your graveyard to your hand.",
  },
  {
    name: "Odric's Outrider",
    oracle: "whenever this creature or another creature you control dies, put a +1/+1 counter on target creature you control.",
  },
  {
    name: "Discerning Peddler",
    oracle: "when this creature enters, you may discard a card. if you do, draw a card.",
  },
  {
    name: "Undertaker",
    oracle: "{b}, {t}, discard a card: return target creature card from your graveyard to your hand.",
  },
  {
    name: "Blood Host",
    oracle: "{1}{b}, sacrifice another creature: put a +1/+1 counter on this creature and you gain 2 life.",
  },
  {
    name: "Skullmead Cauldron",
    oracle: "{t}: you gain 1 life.\n{t}, discard a card: you gain 3 life.",
  },
  {
    name: "Restless Bones",
    oracle: "{3}{b}, {t}: target creature gains swampwalk until end of turn. \n{1}{b}: regenerate this creature.",
  },
  {
    name: "Phyrexia's Core",
    oracle: "{t}: add {c}.\n{1}, {t}, sacrifice an artifact: you gain 1 life.",
  },
  {
    name: "Gift of Compleation",
    oracle: "when this enchantment enters, incubate 3. \nwhenever a phyrexian you control dies, surveil 1.",
  },
  {
    name: "Clay Revenant",
    oracle: "this creature enters tapped.\n{2}{b}: return this card from your graveyard to your hand.",
  },
  {
    name: "Archaeological Dig",
    oracle: "{t}: add {c}.\n{t}, sacrifice this land: add one mana of any color.",
  },
  {
    name: "She-Hulk, Jennifer Walters",
    oracle: "trample \n{2}{r}, sacrifice a land: draw a card and put a +1/+1 counter on ~.",
  },
  {
    name: "Immersturm Raider",
    oracle: "when this creature enters, you may discard a card. if you do, draw a card.",
  },
  {
    name: "Fissure Wizard",
    oracle: "when this creature enters, you may discard a card. if you do, draw a card.",
  },
  {
    name: "Dread Rider",
    oracle: "{1}{b}, {t}, exile a creature card from your graveyard: target opponent loses 3 life.",
  },
  {
    name: "Ocular Halo",
    oracle: "enchant creature\nenchanted creature has \"{t}: draw a card.\"\n{w}: enchanted creature gains vigilance until end of turn.",
  },
  {
    name: "Sarcomite Myr",
    oracle: "{2}: this creature gains flying until end of turn.\n{2}, sacrifice this creature: draw a card.",
  },
  {
    name: "Thornscape Apprentice",
    oracle: "{r}, {t}: target creature gains first strike until end of turn.\n{w}, {t}: tap target creature.",
  },
  {
    name: "Dutiful Griffin",
    oracle: "flying\n{2}{w}, sacrifice two enchantments: return this card from your graveyard to your hand.",
  },
  {
    name: "Crystal Chimes",
    oracle: "{3}, {t}, sacrifice this artifact: return all enchantment cards from your graveyard to your hand.",
  },
  {
    name: "Energy Refractor",
    oracle: "when this artifact enters, draw a card.\n{2}: add one mana of any color.",
  },
  {
    name: "Indebted Samurai",
    oracle: "bushido 1 \nwhenever a samurai you control dies, you may put a +1/+1 counter on this creature.",
  },
  {
    name: "Gravetiller Wurm",
    oracle: "trample\nmorbid — this creature enters with four +1/+1 counters on it if a creature died this turn.",
  },
  {
    name: "Sharuum the Hegemon",
    oracle: "flying\nwhen ~ enters, you may return target artifact card from your graveyard to the battlefield.",
  },
  {
    name: "Unstable Obelisk",
    oracle: "{t}: add {c}.\n{7}, {t}, sacrifice this artifact: destroy target permanent.",
  },
  {
    name: "Tatyova, Benthic Druid",
    oracle: "landfall — whenever a land you control enters, you gain 1 life and draw a card.",
  },
  {
    name: "Mana Geode",
    oracle: "when this artifact enters, scry 1.\n{t}: add one mana of any color.",
  },
  {
    name: "Tin-Wing Chimera",
    oracle: "flying\nsacrifice this creature: put a +2/+2 counter on target chimera creature. it gains flying.",
  },
  {
    name: "Doomed Necromancer",
    oracle: "{b}, {t}, sacrifice this creature: return target creature card from your graveyard to the battlefield.",
  },
  {
    name: "Gravedigger",
    oracle: "when this creature enters, you may return target creature card from your graveyard to your hand.",
  },
  {
    name: "Erinis, Gloom Stalker",
    oracle: "deathtouch\nwhenever ~ attacks, return target land card from your graveyard to the battlefield.\nchoose a background",
  },
  {
    name: "Brass-Talon Chimera",
    oracle: "first strike\nsacrifice this creature: put a +2/+2 counter on target chimera creature. it gains first strike.",
  },
  {
    name: "Syndicate Trafficker",
    oracle: "{1}, sacrifice an artifact: put a +1/+1 counter on this creature. it gains indestructible until end of turn.",
  },
  {
    name: "Exalted Angel",
    oracle: "flying\nwhenever this creature deals damage, you gain that much life.\nmorph {2}{w}{w}",
  },
  {
    name: "Apothecary Geist",
    oracle: "flying\nwhen this creature enters, if you control another spirit, you gain 3 life.",
  },
  {
    name: "Bloodborn Scoundrels",
    oracle: "assist \nwhen this creature enters, target opponent loses 2 life and you gain 2 life.",
  },
  {
    name: "Fallen Angel Avatar",
    oracle: "whenever a creature you control dies, target opponent loses 1 life and you gain 1 life.",
  },
  {
    name: "Bronze Walrus",
    oracle: "when this creature enters, scry 2. \n{t}: add one mana of any color.",
  },
  {
    name: "Skymarch Bloodletter",
    oracle: "flying\nwhen this creature enters, target opponent loses 1 life and you gain 1 life.",
  },
  {
    name: "Sibsig Muckdraggers",
    oracle: "delve \nwhen this creature enters, return target creature card from your graveyard to your hand.",
  },
  {
    name: "Draconic Lore",
    oracle: "this spell costs {2} less to cast if you control a dragon.\ndraw three cards.",
  },
  {
    name: "Restoration Gearsmith",
    oracle: "when this creature enters, return target artifact or creature card from your graveyard to your hand.",
  },
  {
    name: "Vampire Sovereign",
    oracle: "flying\nwhen this creature enters, target opponent loses 3 life and you gain 3 life.",
  },
  {
    name: "Iizuka the Ruthless",
    oracle: "bushido 2 \n{2}{r}, sacrifice a samurai: samurai creatures you control gain double strike until end of turn.",
  },
  {
    name: "Cartographer",
    oracle: "when this creature enters, you may return target land card from your graveyard to your hand.",
  },
  {
    name: "Skyship Buccaneer",
    oracle: "flying\nraid — when this creature enters, if you attacked this turn, draw a card.",
  },
  {
    name: "Strix Lookout",
    oracle: "flying, vigilance \n{1}{u}, {t}: draw a card, then discard a card.",
  },
  {
    name: "Hollowhenge Scavenger",
    oracle: "morbid — when this creature enters, if a creature died this turn, you gain 5 life.",
  },
  {
    name: "Phyrexian Reclamation",
    oracle: "{1}{b}, pay 2 life: return target creature card from your graveyard to your hand.",
  },
  {
    name: "Treasure Hunter",
    oracle: "when this creature enters, you may return target artifact card from your graveyard to your hand.",
  },
  {
    name: "Territorial Scythecat",
    oracle: "trample \nlandfall — whenever a land you control enters, put a +1/+1 counter on this creature.",
  },
  {
    name: "Illuminated Wings",
    oracle: "enchant creature\nenchanted creature has flying.\n{2}, sacrifice this aura: draw a card.",
  },
  {
    name: "Overgrown Arch",
    oracle: "defender\n{t}: you gain 1 life.\n{2}, sacrifice this creature: learn.",
  },
  {
    name: "Gilded Pinions",
    oracle: "when this equipment enters, create a treasure token. \nequipped creature has flying.\nequip {2}",
  },
  {
    name: "Kor Outfitter",
    oracle: "when this creature enters, you may attach target equipment you control to target creature you control.",
  },
  {
    name: "Somberwald Spider",
    oracle: "reach \nmorbid — this creature enters with two +1/+1 counters on it if a creature died this turn.",
  },
  {
    name: "Pearl of Wisdom",
    oracle: "this spell costs {1} less to cast if you control an otter.\ndraw two cards.",
  },
  {
    name: "Flayer Drone",
    oracle: "devoid \nfirst strike\nwhenever another colorless creature you control enters, target opponent loses 1 life.",
  },
  {
    name: "Sire of Seven Deaths",
    oracle: "first strike, vigilance\nmenace, trample\nreach, lifelink\nward—pay 7 life.",
  },
  {
    name: "Wren's Run Hydra",
    oracle: "reach\nthis creature enters with x +1/+1 counters on it.\nreinforce x—{x}{g}{g}",
  },
  {
    name: "Strongarm Thug",
    oracle: "when this creature enters, you may return target mercenary card from your graveyard to your hand.",
  },
  {
    name: "Sunhome, Fortress of the Legion",
    oracle: "{t}: add {c}.\n{2}{r}{w}, {t}: target creature gains double strike until end of turn.",
  },
  {
    name: "Heartmender",
    oracle: "at the beginning of your upkeep, remove a -1/-1 counter from each creature you control.\npersist",
  },
  {
    name: "Auramancer",
    oracle: "when this creature enters, you may return target enchantment card from your graveyard to your hand.",
  },
  {
    name: "Festerhide Boar",
    oracle: "trample\nmorbid — this creature enters with two +1/+1 counters on it if a creature died this turn.",
  },
  {
    name: "Gibbering Barricade",
    oracle: "defender\n{2}{b}, sacrifice a creature: you gain 1 life and draw a card.",
  },
  {
    name: "Mind Stone",
    oracle: "{t}: add {c}.\n{1}, {t}, sacrifice this artifact: draw a card.",
  },
  {
    name: "Griffin Dreamfinder",
    oracle: "flying\nwhen this creature enters, return target enchantment card from your graveyard to your hand.",
  },
  {
    name: "Witch's Cauldron",
    oracle: "{1}{b}, {t}, sacrifice a creature: you gain 1 life and draw a card.",
  },
  {
    name: "Stoic Builder",
    oracle: "when this creature enters, you may return target land card from your graveyard to your hand.",
  },
  {
    name: "Boosted Sloop",
    oracle: "menace\nwhenever you attack, draw a card, then discard a card.\ncrew 1",
  },
  {
    name: "Rushwood Elemental",
    oracle: "trample\nat the beginning of your upkeep, you may put a +1/+1 counter on this creature.",
  },
  {
    name: "Skithiryx, the Blight Dragon",
    oracle: "flying\ninfect \n{b}: ~ gains haste until end of turn.\n{b}{b}: regenerate ~.",
  },
  {
    name: "Spinning Wheel",
    oracle: "{t}: add one mana of any color.\n{5}, {t}: tap target creature.",
  },
  {
    name: "Snapping Voidcraw",
    oracle: "devoid \n{t}: add {c}{c}.\n{3}{c}, {t}: draw a card.",
  },
  {
    name: "Lead-Belly Chimera",
    oracle: "trample\nsacrifice this creature: put a +2/+2 counter on target chimera creature. it gains trample.",
  },
  {
    name: "Prowling Felidar",
    oracle: "vigilance\nlandfall — whenever a land you control enters, put a +1/+1 counter on this creature.",
  },
  {
    name: "Lotus-Eye Mystics",
    oracle: "prowess \nwhen this creature enters, return target enchantment card from your graveyard to your hand.",
  },
  {
    name: "Tortured Existence",
    oracle: "{b}, discard a creature card: return target creature card from your graveyard to your hand.",
  },
  {
    name: "Arcane Epiphany",
    oracle: "this spell costs {1} less to cast if you control a wizard.\ndraw three cards.",
  },
  {
    name: "Iron-Heart Chimera",
    oracle: "vigilance\nsacrifice this creature: put a +2/+2 counter on target chimera creature. it gains vigilance.",
  },
  {
    name: "Plumecreed Escort",
    oracle: "flash\nflying\nwhen this creature enters, target creature you control gains hexproof until end of turn.",
  },
  {
    name: "Soultether Golem",
    oracle: "vanishing 1 \nwhenever another creature you control enters, put a time counter on this creature.",
  },
  {
    name: "Perimeter Captain",
    oracle: "defender\nwhenever a creature you control with defender blocks, you may gain 2 life.",
  },
  {
    name: "Incarnation Technique",
    oracle: "demonstrate \nmill five cards, then return a creature card from your graveyard to the battlefield.",
  },
  {
    name: "Grisly Transformation",
    oracle: "enchant creature\nwhen this aura enters, draw a card.\nenchanted creature has intimidate.",
  },
  {
    name: "Advocate of the Beast",
    oracle: "at the beginning of your end step, put a +1/+1 counter on target beast creature you control.",
  },
  {
    name: "Life Goes On",
    oracle: "you gain 4 life. if a creature died this turn, you gain 8 life instead.",
  },
  {
    name: "Sphinx of Magosi",
    oracle: "flying\n{2}{u}: draw a card, then put a +1/+1 counter on this creature.",
  },
  {
    name: "Leatherhead, Iron Gator",
    oracle: "trample, haste\nwhenever ~ attacks, put two +1/+1 counters on each creature you control.",
  },
  {
    name: "Homestead Courage",
    oracle: "put a +1/+1 counter on target creature you control. it gains vigilance until end of turn.\nflashback {w}",
  },
  {
    name: "Bone Picker",
    oracle: "this spell costs {3} less to cast if a creature died this turn.\nflying, deathtouch",
  },
  {
    name: "Eternal Witness",
    oracle: "when this creature enters, you may return target card from your graveyard to your hand.",
  },
  {
    name: "Firewing Phoenix",
    oracle: "flying\n{1}{r}{r}{r}: return this card from your graveyard to your hand.",
  },
  {
    name: "Qarsi High Priest",
    oracle: "{1}{b}, {t}, sacrifice another creature: manifest the top card of your library.",
  },
  {
    name: "Dreaded Bat-Cloud",
    oracle: "this spell costs {3} less to cast if a creature died this turn.\nflying, deathtouch",
  },
  {
    name: "Grinning Demon",
    oracle: "at the beginning of your upkeep, you lose 2 life.\nmorph {2}{b}{b}",
  },
  {
    name: "Krakilin",
    oracle: "this creature enters with x +1/+1 counters on it.\n{1}{g}: regenerate this creature.",
  },
  {
    name: "Rootwater Diver",
    oracle: "{t}, sacrifice this creature: return target artifact card from your graveyard to your hand.",
  },
  {
    name: "Royal Assassin Avatar",
    oracle: "at the beginning of your upkeep, you draw a card and you lose 1 life.",
  },
  {
    name: "School of the Unseen",
    oracle: "{t}: add {c}.\n{2}, {t}: add one mana of any color.",
  },
  {
    name: "Salvage Scout",
    oracle: "{w}, sacrifice this creature: return target artifact card from your graveyard to your hand.",
  },
];
