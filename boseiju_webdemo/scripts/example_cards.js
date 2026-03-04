const EXAMPLE_CARDS = [
  {
    name: "Greenwarden of Murasa",
    oracle: "when this creature enters, you may return target card from your graveyard to your hand.\nwhen this creature dies, you may exile it. if you do, return target card from your graveyard to your hand.",
  },
  {
    name: "High-Society Hunter",
    oracle: "flying\nwhenever this creature attacks, you may sacrifice another creature. if you do, put a +1/+1 counter on this creature.\nwhenever another nontoken creature dies, draw a card.",
  },
  {
    name: "Bounty of Might",
    oracle: "target creature gets +3/+3 until end of turn.\ntarget creature gets +3/+3 until end of turn.\ntarget creature gets +3/+3 until end of turn.",
  },
  {
    name: "Seeds of Strength",
    oracle: "target creature gets +1/+1 until end of turn.\ntarget creature gets +1/+1 until end of turn.\ntarget creature gets +1/+1 until end of turn.",
  },
  {
    name: "Patron of the Vein",
    oracle: "flying\nwhen this creature enters, destroy target creature an opponent controls.\nwhenever a creature an opponent controls dies, exile it and put a +1/+1 counter on each vampire you control.",
  },
  {
    name: "Earth King's Lieutenant",
    oracle: "trample\nwhen this creature enters, put a +1/+1 counter on each other ally creature you control.\nwhenever another ally you control enters, put a +1/+1 counter on this creature.",
  },
  {
    name: "Sanguine Indulgence",
    oracle: "this spell costs {3} less to cast if you've gained 3 or more life this turn.\nreturn up to two target creature cards from your graveyard to your hand.",
  },
  {
    name: "Beetle-Headed Merchants",
    oracle: "whenever this creature attacks, you may sacrifice another creature or artifact. if you do, draw a card and put a +1/+1 counter on this creature.",
  },
  {
    name: "Honest Rutstein",
    oracle: "when ~ enters, return target creature card from your graveyard to your hand.\ncreature spells you cast cost {1} less to cast.",
  },
  {
    name: "Tombstone, Career Criminal",
    oracle: "when ~ enters, return target villain card from your graveyard to your hand.\nvillain spells you cast cost {1} less to cast.",
  },
  {
    name: "Wedding Security",
    oracle: "whenever this creature attacks, you may sacrifice a blood token. if you do, put a +1/+1 counter on this creature and draw a card.",
  },
  {
    name: "Thalia's Lieutenant",
    oracle: "when this creature enters, put a +1/+1 counter on each other human you control.\nwhenever another human you control enters, put a +1/+1 counter on this creature.",
  },
  {
    name: "Iname, Life Aspect",
    oracle: "when ~ dies, you may exile it. if you do, return any number of target spirit cards from your graveyard to your hand.",
  },
  {
    name: "Shipwreck Looter",
    oracle: "raid — when this creature enters, if you attacked this turn, you may draw a card. if you do, discard a card.",
  },
  {
    name: "Courier Bat",
    oracle: "flying\nwhen this creature enters, if you gained life this turn, return up to one target creature card from your graveyard to your hand.",
  },
  {
    name: "Drakuseth, Maw of Flames",
    oracle: "flying\nwhenever ~ attacks, it deals 4 damage to any target and 3 damage to each of up to two other targets.",
  },
  {
    name: "Oath of Ajani",
    oracle: "when ~ enters, put a +1/+1 counter on each creature you control.\nplaneswalker spells you cast cost {1} less to cast.",
  },
  {
    name: "Incinerating Blast",
    oracle: "~ deals 6 damage to target creature.\nyou may discard a card. if you do, draw a card.",
  },
  {
    name: "Wasp of the Bitter End",
    oracle: "flying\nwhenever you cast a bolas planeswalker spell, you may sacrifice this creature. if you do, destroy target creature.",
  },
  {
    name: "Aunt May",
    oracle: "whenever another creature you control enters, you gain 1 life. if it's a spider, put a +1/+1 counter on it.",
  },
  {
    name: "Glissa, the Traitor",
    oracle: "first strike, deathtouch\nwhenever a creature an opponent controls dies, you may return target artifact card from your graveyard to your hand.",
  },
  {
    name: "Cemetery Recruitment",
    oracle: "return target creature card from your graveyard to your hand. if it's a zombie card, draw a card.",
  },
  {
    name: "Harvester Troll",
    oracle: "when this creature enters, you may sacrifice a creature or land. if you do, put two +1/+1 counters on this creature.",
  },
  {
    name: "Skyswimmer Koi",
    oracle: "flying\nwhenever an artifact you control enters, you may draw a card. if you do, discard a card.",
  },
  {
    name: "Dreamcatcher",
    oracle: "whenever you cast a spirit or arcane spell, you may sacrifice this creature. if you do, draw a card.",
  },
  {
    name: "Tweeze",
    oracle: "~ deals 3 damage to any target. you may discard a card. if you do, draw a card.",
  },
  {
    name: "Salvage Drone",
    oracle: "devoid \ningest \nwhen this creature dies, you may draw a card. if you do, discard a card.",
  },
  {
    name: "Shoal Kraken",
    oracle: "constellation — whenever an enchantment you control enters, you may draw a card. if you do, discard a card.",
  },
  {
    name: "Rook Turret",
    oracle: "flying\nwhenever another artifact you control enters, you may draw a card. if you do, discard a card.",
  },
  {
    name: "Murder of Crows",
    oracle: "flying\nwhenever another creature dies, you may draw a card. if you do, discard a card.",
  },
  {
    name: "Martial Glory",
    oracle: "target creature gets +3/+0 until end of turn.\ntarget creature gets +0/+3 until end of turn.",
  },
  {
    name: "Mnemonic Wall",
    oracle: "defender\nwhen this creature enters, you may return target instant or sorcery card from your graveyard to your hand.",
  },
  {
    name: "Burning-Tree Vandal",
    oracle: "riot \nwhenever this creature attacks, you may discard a card. if you do, draw a card.",
  },
  {
    name: "Yuyan Archers",
    oracle: "reach\nwhen this creature enters, you may discard a card. if you do, draw a card.",
  },
  {
    name: "Quicksmith Genius",
    oracle: "whenever an artifact you control enters, you may discard a card. if you do, draw a card.",
  },
  {
    name: "Thrilling Discovery",
    oracle: "you gain 2 life. then you may discard two cards. if you do, draw three cards.",
  },
  {
    name: "Pelakka Wurm",
    oracle: "trample \nwhen this creature enters, you gain 7 life.\nwhen this creature dies, draw a card.",
  },
  {
    name: "Plundering Predator",
    oracle: "flying\nwhen this creature enters, you may discard a card. if you do, draw a card.",
  },
  {
    name: "Pharika's Mender",
    oracle: "when this creature enters, you may return target creature or enchantment card from your graveyard to your hand.",
  },
  {
    name: "Graveshifter",
    oracle: "changeling \nwhen this creature enters, you may return target creature card from your graveyard to your hand.",
  },
  {
    name: "Gorehorn Raider",
    oracle: "raid — when this creature enters, if you attacked this turn, this creature deals 2 damage to any target.",
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
    name: "Storm Fleet Pyromancer",
    oracle: "raid — when this creature enters, if you attacked this turn, this creature deals 2 damage to any target.",
  },
  {
    name: "Ironclad Slayer",
    oracle: "when this creature enters, you may return target aura or equipment card from your graveyard to your hand.",
  },
  {
    name: "Shipwreck Dowser",
    oracle: "prowess \nwhen this creature enters, return target instant or sorcery card from your graveyard to your hand.",
  },
  {
    name: "Mardu Heart-Piercer",
    oracle: "raid — when this creature enters, if you attacked this turn, this creature deals 2 damage to any target.",
  },
  {
    name: "Keldon Raider",
    oracle: "when this creature enters, you may discard a card. if you do, draw a card.",
  },
  {
    name: "Junk Diver",
    oracle: "flying\nwhen this creature dies, return another target artifact card from your graveyard to your hand.",
  },
  {
    name: "Viashino Racketeer",
    oracle: "when this creature enters, you may discard a card. if you do, draw a card.",
  },
  {
    name: "Setessan Champion",
    oracle: "constellation — whenever an enchantment you control enters, put a +1/+1 counter on this creature and draw a card.",
  },
  {
    name: "Danitha Capashen, Paragon",
    oracle: "first strike, vigilance, lifelink\naura and equipment spells you cast cost {1} less to cast.",
  },
  {
    name: "Mortality Spear",
    oracle: "this spell costs {2} less to cast if you gained life this turn.\ndestroy target nonland permanent.",
  },
  {
    name: "Common Iguana",
    oracle: "when this creature enters, you may discard a card. if you do, draw a card.",
  },
  {
    name: "Tilling Treefolk",
    oracle: "when this creature enters, you may return up to two target land cards from your graveyard to your hand.",
  },
  {
    name: "Discerning Peddler",
    oracle: "when this creature enters, you may discard a card. if you do, draw a card.",
  },
  {
    name: "Filigree Familiar",
    oracle: "when this creature enters, you gain 2 life.\nwhen this creature dies, draw a card.",
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
    name: "Moriok Scavenger",
    oracle: "when this creature enters, you may return target artifact creature card from your graveyard to your hand.",
  },
  {
    name: "Flaming Sword",
    oracle: "flash\nenchant creature\nenchanted creature gets +1/+0 and has first strike.",
  },
  {
    name: "Wings of Aesthir",
    oracle: "enchant creature\nenchanted creature gets +1/+0 and has flying and first strike.",
  },
  {
    name: "Tatyova, Benthic Druid",
    oracle: "landfall — whenever a land you control enters, you gain 1 life and draw a card.",
  },
  {
    name: "Unflinching Courage",
    oracle: "enchant creature\nenchanted creature gets +2/+2 and has trample and lifelink. ",
  },
  {
    name: "Soul of Magma",
    oracle: "whenever you cast a spirit or arcane spell, this creature deals 1 damage to target creature.",
  },
  {
    name: "Gravedigger",
    oracle: "when this creature enters, you may return target creature card from your graveyard to your hand.",
  },
  {
    name: "Izzet Chronarch",
    oracle: "when this creature enters, return target instant or sorcery card from your graveyard to your hand.",
  },
  {
    name: "Ardent Elementalist",
    oracle: "when this creature enters, return target instant or sorcery card from your graveyard to your hand.",
  },
  {
    name: "Living Lightning",
    oracle: "when this creature dies, return target instant or sorcery card from your graveyard to your hand.",
  },
  {
    name: "Scrivener",
    oracle: "when this creature enters, you may return target instant card from your graveyard to your hand.",
  },
  {
    name: "Drake Umbra",
    oracle: "enchant creature\nenchanted creature gets +3/+3 and has flying.\numbra armor ",
  },
  {
    name: "Tiger Claws",
    oracle: "flash\nenchant creature\nenchanted creature gets +1/+1 and has trample.",
  },
  {
    name: "Zephid's Embrace",
    oracle: "enchant creature\nenchanted creature gets +2/+2 and has flying and shroud. ",
  },
  {
    name: "Minwu, White Mage",
    oracle: "vigilance, lifelink\nwhenever you gain life, put a +1/+1 counter on each cleric you control.",
  },
  {
    name: "Hyena Umbra",
    oracle: "enchant creature\nenchanted creature gets +1/+1 and has first strike.\numbra armor ",
  },
  {
    name: "Salvager of Secrets",
    oracle: "when this creature enters, return target instant or sorcery card from your graveyard to your hand.",
  },
  {
    name: "Sibsig Muckdraggers",
    oracle: "delve \nwhen this creature enters, return target creature card from your graveyard to your hand.",
  },
  {
    name: "Restoration Gearsmith",
    oracle: "when this creature enters, return target artifact or creature card from your graveyard to your hand.",
  },
  {
    name: "Archangel of Thune",
    oracle: "flying\nlifelink \nwhenever you gain life, put a +1/+1 counter on each creature you control.",
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
    name: "Frantic Strength",
    oracle: "flash\nenchant creature\nenchanted creature gets +2/+2 and has trample.",
  },
  {
    name: "Krosan Adaptation",
    oracle: "enchant creature\nstorm \nenchanted creature gets +1/+0 and has vigilance.",
  },
  {
    name: "Sire of the Storm",
    oracle: "flying\nwhenever you cast a spirit or arcane spell, you may draw a card.",
  },
  {
    name: "Serra's Embrace",
    oracle: "enchant creature\nenchanted creature gets +2/+2 and has flying and vigilance. ",
  },
  {
    name: "Treasure Hunter",
    oracle: "when this creature enters, you may return target artifact card from your graveyard to your hand.",
  },
  {
    name: "Anarchist",
    oracle: "when this creature enters, you may return target sorcery card from your graveyard to your hand.",
  },
  {
    name: "Trick Shot",
    oracle: "~ deals 6 damage to target creature and 2 damage to up to one other target creature token.",
  },
  {
    name: "Urborg Uprising",
    oracle: "return up to two target creature cards from your graveyard to your hand.\ndraw a card.",
  },
  {
    name: "Ovalchase Daredevil",
    oracle: "whenever an artifact you control enters, you may return this card from your graveyard to your hand.",
  },
  {
    name: "Strongarm Thug",
    oracle: "when this creature enters, you may return target mercenary card from your graveyard to your hand.",
  },
  {
    name: "Spiteful Motives",
    oracle: "flash \nenchant creature\nenchanted creature gets +3/+0 and has first strike.",
  },
  {
    name: "Archaeomancer",
    oracle: "when this creature enters, return target instant or sorcery card from your graveyard to your hand.",
  },
  {
    name: "Sprite Dragon",
    oracle: "flying, haste\nwhenever you cast a noncreature spell, put a +1/+1 counter on this creature.",
  },
  {
    name: "Epic Proportions",
    oracle: "flash\nenchant creature\nenchanted creature gets +5/+5 and has trample.",
  },
  {
    name: "Auramancer",
    oracle: "when this creature enters, you may return target enchantment card from your graveyard to your hand.",
  },
  {
    name: "Griffin Dreamfinder",
    oracle: "flying\nwhen this creature enters, return target enchantment card from your graveyard to your hand.",
  },
  {
    name: "Stoic Builder",
    oracle: "when this creature enters, you may return target land card from your graveyard to your hand.",
  },
  {
    name: "Mammoth Umbra",
    oracle: "enchant creature\nenchanted creature gets +3/+3 and has vigilance.\numbra armor ",
  },
  {
    name: "Gift of Orzhova",
    oracle: "enchant creature\nenchanted creature gets +1/+1 and has flying and lifelink.",
  },
  {
    name: "Lotus-Eye Mystics",
    oracle: "prowess \nwhen this creature enters, return target enchantment card from your graveyard to your hand.",
  },
];
