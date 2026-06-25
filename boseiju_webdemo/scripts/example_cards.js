const EXAMPLE_CARDS = [
  {
    name: "Trading Post",
    oracle: "{1}, {t}, discard a card: you gain 4 life.\n{1}, {t}, pay 1 life: create a 0/1 white goat creature token.\n{1}, {t}, sacrifice a creature: return target artifact card from your graveyard to your hand.\n{1}, {t}, sacrifice an artifact: draw a card.",
  },
  {
    name: "Weather Maker",
    oracle: "landfall — whenever a land you control enters, put a charge counter on this artifact.\n{t}: add one mana of any color.\n{t}, remove two charge counters from this artifact: add {c}{c}.\n{t}, remove three charge counters from this artifact: it deals 3 damage to any target.",
  },
  {
    name: "Thornling",
    oracle: "{g}: this creature gains haste until end of turn.\n{g}: this creature gains trample until end of turn.\n{g}: this creature gains indestructible until end of turn.\n{1}: this creature gets +1/-1 until end of turn.\n{1}: this creature gets -1/+1 until end of turn.",
  },
  {
    name: "Morphling",
    oracle: "{u}: untap this creature.\n{u}: this creature gains flying until end of turn.\n{u}: this creature gains shroud until end of turn. \n{1}: this creature gets +1/-1 until end of turn.\n{1}: this creature gets -1/+1 until end of turn.",
  },
  {
    name: "Assassin Gauntlet",
    oracle: "when this equipment enters, attach it to up to one target creature you control. tap all creatures target opponent controls.\nequipped creature gets +1/+1 and has \"whenever this creature deals combat damage to a player, draw a card, then discard a card.\"\nequip {2}",
  },
  {
    name: "Super-Skrull",
    oracle: "flying\n{2}{w}: create a 0/4 colorless wall creature token with defender.\n{3}{g}: ~ gets +4/+4 until end of turn.\n{4}{r}: ~ deals 4 damage to target creature.\n{5}{u}: target player draws four cards.",
  },
  {
    name: "Sturdy Hatchling",
    oracle: "this creature enters with four -1/-1 counters on it.\n{g/u}: this creature gains shroud until end of turn. \nwhenever you cast a green spell, remove a -1/-1 counter from this creature.\nwhenever you cast a blue spell, remove a -1/-1 counter from this creature.",
  },
  {
    name: "Citizen's Crowbar",
    oracle: "when this equipment enters, create a 1/1 green and white citizen creature token, then attach this equipment to it.\nequipped creature gets +1/+1 and has \"{w}, {t}, sacrifice ~: destroy target artifact or enchantment.\"\nequip {2}",
  },
  {
    name: "Courier's Briefcase",
    oracle: "when this artifact enters, create a 1/1 green and white citizen creature token.\n{t}, sacrifice this artifact: add one mana of any color.\n{w}{u}{b}{r}{g}, {t}, sacrifice this artifact: draw three cards.",
  },
  {
    name: "Dubious Delicacy",
    oracle: "flash\nwhen this artifact enters, up to one target creature gets -3/-3 until end of turn.\n{2}, {t}, sacrifice this artifact: you gain 3 life.\n{2}, {t}, sacrifice this artifact: target opponent loses 3 life.",
  },
  {
    name: "Mindwrack Liege",
    oracle: "other blue creatures you control get +1/+1.\nother red creatures you control get +1/+1.\n{u/r}{u/r}{u/r}{u/r}: you may put a blue or red creature card from your hand onto the battlefield.",
  },
  {
    name: "Creakwood Liege",
    oracle: "other black creatures you control get +1/+1.\nother green creatures you control get +1/+1.\nat the beginning of your upkeep, you may create a 1/1 black and green worm creature token.",
  },
  {
    name: "Thallid Germinator",
    oracle: "at the beginning of your upkeep, put a spore counter on this creature.\nremove three spore counters from this creature: create a 1/1 green saproling creature token.\nsacrifice a saproling: target creature gets +1/+1 until end of turn.",
  },
  {
    name: "Deathspore Thallid",
    oracle: "at the beginning of your upkeep, put a spore counter on this creature.\nremove three spore counters from this creature: create a 1/1 green saproling creature token.\nsacrifice a saproling: target creature gets -1/-1 until end of turn.",
  },
  {
    name: "Skeletal Vampire",
    oracle: "flying\nwhen this creature enters, create two 1/1 black bat creature tokens with flying.\n{3}{b}{b}, sacrifice a bat: create two 1/1 black bat creature tokens with flying.\nsacrifice a bat: regenerate this creature.",
  },
  {
    name: "Spike Rogue",
    oracle: "this creature enters with two +1/+1 counters on it.\n{2}, remove a +1/+1 counter from this creature: put a +1/+1 counter on target creature.\n{2}, remove a +1/+1 counter from a creature you control: put a +1/+1 counter on this creature.",
  },
  {
    name: "Thallid Devourer",
    oracle: "at the beginning of your upkeep, put a spore counter on this creature.\nremove three spore counters from this creature: create a 1/1 green saproling creature token.\nsacrifice a saproling: this creature gets +1/+2 until end of turn.",
  },
  {
    name: "Trigon of Rage",
    oracle: "this artifact enters with three charge counters on it.\n{r}{r}, {t}: put a charge counter on this artifact.\n{2}, {t}, remove a charge counter from this artifact: target creature gets +3/+0 until end of turn.",
  },
  {
    name: "Spike Soldier",
    oracle: "this creature enters with three +1/+1 counters on it.\n{2}, remove a +1/+1 counter from this creature: put a +1/+1 counter on target creature.\nremove a +1/+1 counter from this creature: this creature gets +2/+2 until end of turn.",
  },
  {
    name: "Spike Breeder",
    oracle: "this creature enters with three +1/+1 counters on it.\n{2}, remove a +1/+1 counter from this creature: put a +1/+1 counter on target creature.\n{2}, remove a +1/+1 counter from this creature: create a 1/1 green spike creature token.",
  },
  {
    name: "Biogenic Ooze",
    oracle: "when this creature enters, create a 2/2 green ooze creature token.\nat the beginning of your end step, put a +1/+1 counter on each ooze you control.\n{1}{g}{g}{g}: create a 2/2 green ooze creature token.",
  },
  {
    name: "Wingmantle Chaplain",
    oracle: "defender\nwhen this creature enters, create a 1/1 white bird creature token with flying for each creature with defender you control.\nwhenever another creature you control with defender enters, create a 1/1 white bird creature token with flying.",
  },
  {
    name: "Boggart Mischief",
    oracle: "when this enchantment enters, you may blight 1. if you do, create two 1/1 black and red goblin creature tokens. \nwhenever a goblin creature you control dies, each opponent loses 1 life and you gain 1 life.",
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
    name: "Utopia Mycon",
    oracle: "at the beginning of your upkeep, put a spore counter on this creature.\nremove three spore counters from this creature: create a 1/1 green saproling creature token.\nsacrifice a saproling: add one mana of any color.",
  },
  {
    name: "Vibranium Strike Gauntlets",
    oracle: "flash\nwhen this equipment enters, attach it to target creature you control.\nequipped creature gets +3/+0 and has trample and \"whenever this creature deals combat damage to a player, draw a card.\"\nequip {3}",
  },
  {
    name: "Rank Officer",
    oracle: "when this creature enters, you may discard a card. if you do, create a 2/2 black zombie creature token.\n{1}{b}, {t}, exile a creature card from your graveyard: each opponent loses 2 life.",
  },
  {
    name: "Soul of Innistrad",
    oracle: "deathtouch\n{3}{b}{b}: return up to three target creature cards from your graveyard to your hand.\n{3}{b}{b}, exile this card from your graveyard: return up to three target creature cards from your graveyard to your hand.",
  },
  {
    name: "Ghave, Guru of Spores",
    oracle: "~ enters with five +1/+1 counters on it.\n{1}, remove a +1/+1 counter from a creature you control: create a 1/1 green saproling creature token.\n{1}, sacrifice a creature: put a +1/+1 counter on target creature.",
  },
  {
    name: "Quintessential Katana",
    oracle: "equipped creature gets +1/+1 and has \"whenever this creature deals combat damage, untap it and you gain 2 life.\"\nwhenever a ninja you control enters, you may attach this equipment to it.\nequip {2}",
  },
  {
    name: "Trigon of Corruption",
    oracle: "this artifact enters with three charge counters on it.\n{b}{b}, {t}: put a charge counter on this artifact.\n{2}, {t}, remove a charge counter from this artifact: put a -1/-1 counter on target creature.",
  },
  {
    name: "Noxious Hatchling",
    oracle: "this creature enters with four -1/-1 counters on it.\nwither \nwhenever you cast a black spell, remove a -1/-1 counter from this creature.\nwhenever you cast a green spell, remove a -1/-1 counter from this creature.",
  },
  {
    name: "Psychotrope Thallid",
    oracle: "at the beginning of your upkeep, put a spore counter on this creature.\nremove three spore counters from this creature: create a 1/1 green saproling creature token.\n{1}, sacrifice a saproling: draw a card.",
  },
  {
    name: "Belligerent Hatchling",
    oracle: "first strike\nthis creature enters with four -1/-1 counters on it.\nwhenever you cast a red spell, remove a -1/-1 counter from this creature.\nwhenever you cast a white spell, remove a -1/-1 counter from this creature.",
  },
  {
    name: "Vitaspore Thallid",
    oracle: "at the beginning of your upkeep, put a spore counter on this creature.\nremove three spore counters from this creature: create a 1/1 green saproling creature token.\nsacrifice a saproling: target creature gains haste until end of turn.",
  },
  {
    name: "Voracious Hatchling",
    oracle: "lifelink\nthis creature enters with four -1/-1 counters on it.\nwhenever you cast a white spell, remove a -1/-1 counter from this creature.\nwhenever you cast a black spell, remove a -1/-1 counter from this creature.",
  },
  {
    name: "The Meathook Massacre",
    oracle: "when ~ enters, each creature gets -x/-x until end of turn.\nwhenever a creature you control dies, each opponent loses 1 life.\nwhenever a creature an opponent controls dies, you gain 1 life.",
  },
  {
    name: "Garruk's Uprising",
    oracle: "when this enchantment enters, if you control a creature with power 4 or greater, draw a card.\ncreatures you control have trample. \nwhenever a creature you control with power 4 or greater enters, draw a card.",
  },
  {
    name: "Osai Vultures",
    oracle: "flying\nat the beginning of each end step, if a creature died this turn, put a carrion counter on this creature.\nremove two carrion counters from this creature: this creature gets +1/+1 until end of turn.",
  },
  {
    name: "Spike Hatcher",
    oracle: "this creature enters with six +1/+1 counters on it.\n{2}, remove a +1/+1 counter from this creature: put a +1/+1 counter on target creature.\n{1}, remove a +1/+1 counter from this creature: regenerate this creature.",
  },
  {
    name: "Elvish Farmer",
    oracle: "at the beginning of your upkeep, put a spore counter on this creature.\nremove three spore counters from this creature: create a 1/1 green saproling creature token.\nsacrifice a saproling: you gain 2 life.",
  },
  {
    name: "Mycologist",
    oracle: "at the beginning of your upkeep, put a spore counter on this creature.\nremove three spore counters from this creature: create a 1/1 green saproling creature token.\nsacrifice a saproling: you gain 2 life.",
  },
  {
    name: "Quilled Greatwurm",
    oracle: "trample\nwhenever a creature you control deals combat damage during your turn, put that many +1/+1 counters on it. \nyou may cast this card from your graveyard by removing six counters from among creatures you control in addition to paying its other costs.",
  },
  {
    name: "Trigon of Mending",
    oracle: "this artifact enters with three charge counters on it.\n{w}{w}, {t}: put a charge counter on this artifact.\n{2}, {t}, remove a charge counter from this artifact: target player gains 3 life.",
  },
  {
    name: "Golden Egg",
    oracle: "when this artifact enters, draw a card.\n{1}, {t}, sacrifice this artifact: add one mana of any color.\n{2}, {t}, sacrifice this artifact: you gain 3 life.",
  },
  {
    name: "Savage Thallid",
    oracle: "at the beginning of your upkeep, put a spore counter on this creature.\nremove three spore counters from this creature: create a 1/1 green saproling creature token.\nsacrifice a saproling: regenerate target fungus.",
  },
  {
    name: "Avenger of Zendikar",
    oracle: "when this creature enters, create a 0/1 green plant creature token for each land you control.\nlandfall — whenever a land you control enters, you may put a +1/+1 counter on each plant creature you control.",
  },
  {
    name: "Spike Feeder",
    oracle: "this creature enters with two +1/+1 counters on it.\n{2}, remove a +1/+1 counter from this creature: put a +1/+1 counter on target creature.\nremove a +1/+1 counter from this creature: you gain 2 life.",
  },
  {
    name: "Lattice-Blade Mantis",
    oracle: "this creature enters with two oil counters on it.\nwhenever this creature attacks, you may remove an oil counter from it. if you do, untap it and it gets +1/+1 until end of turn.",
  },
  {
    name: "Cabal Patriarch",
    oracle: "{2}{b}, sacrifice a creature: target creature gets -2/-2 until end of turn.\n{2}{b}, exile a creature card from your graveyard: target creature gets -2/-2 until end of turn.",
  },
  {
    name: "Omni-Cheese Pizza",
    oracle: "when this artifact enters, draw a card.\n{1}, {t}, sacrifice this artifact: add one mana of any color.\n{2}, {t}, sacrifice this artifact: you gain 3 life.",
  },
  {
    name: "Relic Vial",
    oracle: "{2}, {t}, sacrifice a creature: draw a card.\nas long as you control a cleric, this artifact has \"whenever a creature you control dies, each opponent loses 1 life and you gain 1 life.\"",
  },
  {
    name: "Sanctifier of Souls",
    oracle: "whenever another creature you control enters, this creature gets +1/+1 until end of turn.\n{2}{w}, exile a creature card from your graveyard: create a 1/1 white spirit creature token with flying.",
  },
  {
    name: "Alela, Artful Provocateur",
    oracle: "flying, deathtouch, lifelink\nother creatures you control with flying get +1/+0.\nwhenever you cast an artifact or enchantment spell, create a 1/1 blue faerie creature token with flying.",
  },
  {
    name: "Nurturing Presence",
    oracle: "enchant creature\nenchanted creature has \"whenever a creature you control enters, this creature gets +1/+1 until end of turn.\"\nwhen this aura enters, create a 1/1 white spirit creature token with flying.",
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
    name: "Mask of Immolation",
    oracle: "when this equipment enters, create a 1/1 red elemental creature token, then attach this equipment to it.\nequipped creature has \"sacrifice this creature: it deals 1 damage to any target.\"\nequip {2}",
  },
  {
    name: "Omnath, Locus of Rage",
    oracle: "landfall — whenever a land you control enters, create a 5/5 red and green elemental creature token.\nwhenever ~ or another elemental you control dies, ~ deals 3 damage to any target.",
  },
  {
    name: "Fable of Wolf and Owl",
    oracle: "whenever you cast a green spell, you may create a 2/2 green wolf creature token.\nwhenever you cast a blue spell, you may create a 1/1 blue bird creature token with flying.",
  },
  {
    name: "Knights' Charge",
    oracle: "whenever a knight you control attacks, each opponent loses 1 life and you gain 1 life.\n{6}{w}{b}, sacrifice this enchantment: return all knight creature cards from your graveyard to the battlefield.",
  },
  {
    name: "Emeria, the Sky Ruin",
    oracle: "this land enters tapped.\nat the beginning of your upkeep, if you control seven or more plains, you may return target creature card from your graveyard to the battlefield.\n{t}: add {w}.",
  },
  {
    name: "Midnight Angel Armor",
    oracle: "when this equipment enters, create a 1/1 white soldier creature token, then attach this equipment to it.\nequipped creature gets +3/+3 and has flying and vigilance.\nequip {3}",
  },
  {
    name: "Puppeteer Clique",
    oracle: "flying\nwhen this creature enters, put target creature card from an opponent's graveyard onto the battlefield under your control. it gains haste. at the beginning of your next end step, exile it.\npersist",
  },
  {
    name: "Namazu Trader",
    oracle: "when this creature enters, you lose 1 life and create a treasure token.\nwhenever this creature attacks, you may sacrifice another creature or artifact. if you do, surveil 2.",
  },
  {
    name: "Drake Hatcher",
    oracle: "vigilance, prowess \nwhenever this creature deals combat damage to a player, put that many incubation counters on it.\nremove three incubation counters from this creature: create a 2/2 blue drake creature token with flying.",
  },
  {
    name: "Shaman of the Great Hunt",
    oracle: "haste\nwhenever a creature you control deals combat damage to a player, put a +1/+1 counter on it.\nferocious — {2}{g/u}{g/u}: draw a card for each creature you control with power 4 or greater.",
  },
  {
    name: "Skyclave Pick-Axe",
    oracle: "when this equipment enters, attach it to target creature you control.\nlandfall — whenever a land you control enters, equipped creature gets +2/+2 until end of turn.\nequip {2}{g}",
  },
  {
    name: "Ravos, Soultender",
    oracle: "flying\nother creatures you control get +1/+1.\nat the beginning of your upkeep, you may return target creature card from your graveyard to your hand.\npartner",
  },
  {
    name: "Mikaeus, the Lunarch",
    oracle: "~ enters with x +1/+1 counters on it.\n{t}: put a +1/+1 counter on ~.\n{t}, remove a +1/+1 counter from ~: put a +1/+1 counter on each other creature you control.",
  },
  {
    name: "Infinity Formula",
    oracle: "when this equipment enters, attach it to target creature you control.\nequipped creature gets +1/+2 and has \"whenever this creature attacks, you gain 2 life.\"\nequip {2}",
  },
  {
    name: "Tragic Banshee",
    oracle: "morbid — when this creature enters, target creature an opponent controls gets -1/-1 until end of turn. if a creature died this turn, that creature gets -13/-13 until end of turn instead.",
  },
  {
    name: "Arashi, the Sky Asunder",
    oracle: "{x}{g}, {t}: ~ deals x damage to target creature with flying.\nchannel — {x}{g}{g}, discard this card: it deals x damage to each creature with flying.",
  },
  {
    name: "Cruel Sadist",
    oracle: "{b}, {t}, pay 1 life: put a +1/+1 counter on this creature.\n{2}{b}, {t}, remove x +1/+1 counters from this creature: it deals x damage to target creature.",
  },
  {
    name: "Jukai Preserver",
    oracle: "when this creature enters, put a +1/+1 counter on target creature you control.\nchannel — {2}{g}, discard this card: put a +1/+1 counter on each of up to two target creatures you control.",
  },
  {
    name: "Clattering Augur",
    oracle: "this creature can't block.\nwhen this creature enters, you draw a card and you lose 1 life.\n{2}{b}{b}: return this card from your graveyard to your hand.",
  },
  {
    name: "Lavamancer's Skill",
    oracle: "enchant creature\nenchanted creature has \"{t}: this creature deals 1 damage to target creature.\"\nas long as enchanted creature is a wizard, it has \"{t}: this creature deals 2 damage to target creature.\"",
  },
  {
    name: "Iceman and Firestar",
    oracle: "flying\nwhenever you cast a blue spell, tap up to one target creature.\nwhenever you cast a red spell, you may discard a card. if you do, draw a card.",
  },
  {
    name: "Walking Ballista",
    oracle: "this creature enters with x +1/+1 counters on it.\n{4}: put a +1/+1 counter on this creature.\nremove a +1/+1 counter from this creature: it deals 1 damage to any target.",
  },
  {
    name: "Avacyn's Collar",
    oracle: "equipped creature gets +1/+0 and has vigilance.\nwhenever equipped creature dies, if it was a human, create a 1/1 white spirit creature token with flying.\nequip {2}",
  },
  {
    name: "Laser Screwdriver",
    oracle: "{t}: add one mana of any color.\n{1}, {t}: tap target artifact.\n{2}, {t}: surveil 1. \n{3}, {t}: goad target creature.",
  },
  {
    name: "Battlewand Oak",
    oracle: "whenever a forest you control enters, this creature gets +2/+2 until end of turn.\nwhenever you cast a treefolk spell, this creature gets +2/+2 until end of turn.",
  },
  {
    name: "Baseball Bat",
    oracle: "when this equipment enters, attach it to target creature you control.\nequipped creature gets +1/+1.\nwhenever equipped creature attacks, tap up to one target creature.\nequip {3}",
  },
  {
    name: "Harvester of Misery",
    oracle: "menace\nwhen this creature enters, other creatures get -2/-2 until end of turn.\n{1}{b}, discard this card: target creature gets -2/-2 until end of turn.",
  },
  {
    name: "Ghost-Lit Nourisher",
    oracle: "{2}{g}, {t}: target creature gets +2/+2 until end of turn.\nchannel — {3}{g}, discard this card: target creature gets +4/+4 until end of turn.",
  },
  {
    name: "Throwing Knife",
    oracle: "equipped creature gets +2/+0.\nwhenever equipped creature attacks, you may sacrifice this equipment. if you do, this equipment deals 2 damage to any target.\nequip {2}",
  },
  {
    name: "Mindless Automaton",
    oracle: "this creature enters with two +1/+1 counters on it.\n{1}, discard a card: put a +1/+1 counter on this creature.\nremove two +1/+1 counters from this creature: draw a card.",
  },
  {
    name: "Duergar Hedge-Mage",
    oracle: "when this creature enters, if you control two or more mountains, you may destroy target artifact.\nwhen this creature enters, if you control two or more plains, you may destroy target enchantment.",
  },
  {
    name: "Deathrender",
    oracle: "equipped creature gets +2/+2.\nwhenever equipped creature dies, you may put a creature card from your hand onto the battlefield and attach this equipment to it.\nequip {2}",
  },
  {
    name: "Sporoloth Ancient",
    oracle: "at the beginning of your upkeep, put a spore counter on this creature.\ncreatures you control have \"remove two spore counters from this creature: create a 1/1 green saproling creature token.\"",
  },
  {
    name: "Rotwidow Pack",
    oracle: "reach\n{3}{b}{g}, exile a creature card from your graveyard: create a 1/2 green spider creature token with reach, then each opponent loses 1 life for each spider you control.",
  },
  {
    name: "Convenient Target",
    oracle: "enchant creature\nwhen this aura enters, suspect enchanted creature. \nenchanted creature gets +1/+1.\n{2}{r}: return this card from your graveyard to your hand.",
  },
  {
    name: "Dissection Practice",
    oracle: "target opponent loses 1 life and you gain 1 life.\nup to one target creature gets +1/+1 until end of turn.\nup to one target creature gets -1/-1 until end of turn.",
  },
  {
    name: "Headsplitter",
    oracle: "when this equipment enters, create a 1/1 black assassin creature token with menace, then attach this equipment to it.\nequipped creature gets +1/+0.\nequip {2}",
  },
  {
    name: "Soul of Zendikar",
    oracle: "reach\n{3}{g}{g}: create a 3/3 green beast creature token.\n{3}{g}{g}, exile this card from your graveyard: create a 3/3 green beast creature token.",
  },
  {
    name: "Herald of Anguish",
    oracle: "improvise \nflying\nat the beginning of your end step, each opponent discards a card.\n{1}{b}, sacrifice an artifact: target creature gets -2/-2 until end of turn.",
  },
  {
    name: "Merfolk Pupil",
    oracle: "when this creature enters, draw a card, then discard a card.\n{1}{u}, exile this card from your graveyard: draw a card, then discard a card.",
  },
  {
    name: "HYDRA Disintegrator",
    oracle: "when this equipment enters, create a 2/1 black villain creature token with menace, then attach this equipment to it. \nequipped creature gets +3/+3.\nequip {4}",
  },
  {
    name: "Guac & Marshmallow Pizza",
    oracle: "flash\nwhen this artifact enters, target creature gets +2/+2 until end of turn. untap it.\n{2}, {t}, sacrifice this artifact: you gain 3 life.",
  },
  {
    name: "Fungal Plots",
    oracle: "{1}{g}, exile a creature card from your graveyard: create a 1/1 green saproling creature token.\nsacrifice two saprolings: you gain 2 life and draw a card.",
  },
  {
    name: "Bitter Reunion",
    oracle: "when this enchantment enters, you may discard a card. if you do, draw two cards.\n{1}, sacrifice this enchantment: creatures you control gain haste until end of turn.",
  },
  {
    name: "Blighted Shaman",
    oracle: "{t}, sacrifice a swamp: target creature gets +1/+1 until end of turn.\n{t}, sacrifice a creature: target creature gets +2/+2 until end of turn.",
  },
  {
    name: "Disciple of Tevesh Szat",
    oracle: "{t}: target creature gets -1/-1 until end of turn.\n{4}{b}{b}, {t}, sacrifice this creature: target creature gets -6/-6 until end of turn.",
  },
  {
    name: "Squirrel Wrangler",
    oracle: "{1}{g}, sacrifice a land: create two 1/1 green squirrel creature tokens.\n{1}{g}, sacrifice a land: squirrel creatures get +1/+1 until end of turn.",
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
    name: "Narcissism",
    oracle: "{g}, discard a card: target creature gets +2/+2 until end of turn.\n{g}, sacrifice this enchantment: target creature gets +2/+2 until end of turn.",
  },
  {
    name: "Dire Undercurrents",
    oracle: "whenever a blue creature you control enters, you may have target player draw a card.\nwhenever a black creature you control enters, you may have target player discard a card.",
  },
  {
    name: "Tegwyll, Duke of Splendor",
    oracle: "flying, deathtouch\nother faeries you control get +1/+1.\nwhenever another faerie you control dies, you draw a card and you lose 1 life.",
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
    name: "Twinshot Sniper",
    oracle: "reach\nwhen this creature enters, it deals 2 damage to any target.\nchannel — {1}{r}, discard this card: it deals 2 damage to any target.",
  },
  {
    name: "Pashalik Mons",
    oracle: "whenever ~ or another goblin you control dies, ~ deals 1 damage to any target.\n{3}{r}, sacrifice a goblin: create two 1/1 red goblin creature tokens.",
  },
  {
    name: "Slimy Piper",
    oracle: "whenever this creature attacks, it gets +1/+1 until end of turn. if you control four or more creatures, it gets +2/+2 and gains indestructible until end of turn instead.",
  },
  {
    name: "Metalspinner's Puzzleknot",
    oracle: "when this artifact enters, you draw a card and you lose 1 life.\n{2}{b}, sacrifice this artifact: you draw a card and you lose 1 life.",
  },
  {
    name: "Gift of Growth",
    oracle: "kicker {2} \nuntap target creature. it gets +2/+2 until end of turn. if this spell was kicked, that creature gets +4/+4 until end of turn instead.",
  },
  {
    name: "Muster the Departed",
    oracle: "when this enchantment enters, create a 1/1 white spirit creature token with flying.\nmorbid — at the beginning of your end step, if a creature died this turn, populate.",
  },
  {
    name: "Wrench",
    oracle: "equipped creature gets +1/+1 and has vigilance and \"{3}, {t}: tap target creature.\"\n{2}, sacrifice this equipment: draw a card.\nequip {2}",
  },
  {
    name: "Midnight Entourage",
    oracle: "other aetherborn you control get +1/+1.\nwhenever this creature or another aetherborn you control dies, you draw a card and you lose 1 life.",
  },
  {
    name: "Slayer's Plate",
    oracle: "equipped creature gets +4/+2.\nwhenever equipped creature dies, if it was a human, create a 1/1 white spirit creature token with flying.\nequip {3}",
  },
  {
    name: "Maul of the Skyclaves",
    oracle: "when this equipment enters, attach it to target creature you control.\nequipped creature gets +2/+2 and has flying and first strike.\nequip {2}{w}{w}",
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
    name: "Ghost-Lit Raider",
    oracle: "{2}{r}, {t}: this creature deals 2 damage to target creature.\nchannel — {3}{r}, discard this card: it deals 4 damage to target creature.",
  },
  {
    name: "Bugenhagen, Wise Elder",
    oracle: "reach\nat the beginning of your upkeep, if you control a creature with power 7 or greater, draw a card.\n{t}: add one mana of any color.",
  },
  {
    name: "Thallid Shell-Dweller",
    oracle: "defender\nat the beginning of your upkeep, put a spore counter on this creature.\nremove three spore counters from this creature: create a 1/1 green saproling creature token.",
  },
  {
    name: "Bounty of Might",
    oracle: "target creature gets +3/+3 until end of turn.\ntarget creature gets +3/+3 until end of turn.\ntarget creature gets +3/+3 until end of turn.",
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
    name: "Kyoshi Battle Fan",
    oracle: "when this equipment enters, create a 1/1 white ally creature token, then attach this equipment to it.\nequipped creature gets +1/+0.\nequip {2}",
  },
  {
    name: "Nephalia Moondrakes",
    oracle: "flying\nwhen this creature enters, target creature gains flying until end of turn.\n{4}{u}{u}, exile this card from your graveyard: creatures you control gain flying until end of turn.",
  },
  {
    name: "Supply Drop",
    oracle: "flash\nwhen this artifact enters, target creature you control gets +2/+2 until end of turn.\n{4}, {t}, sacrifice this artifact: draw a card.",
  },
  {
    name: "Tendo Ice Bridge",
    oracle: "this land enters with a charge counter on it.\n{t}: add {c}.\n{t}, remove a charge counter from this land: add one mana of any color.",
  },
  {
    name: "Ancestral Blade",
    oracle: "when this equipment enters, create a 1/1 white soldier creature token, then attach this equipment to it.\nequipped creature gets +1/+1.\nequip {1}",
  },
  {
    name: "Valiant Veteran",
    oracle: "other soldiers you control get +1/+1.\n{3}{w}{w}, exile this card from your graveyard: put a +1/+1 counter on each soldier you control.",
  },
  {
    name: "Skyship Stalker",
    oracle: "flying\n{r}: this creature gets +1/+0 until end of turn.\n{r}: this creature gains first strike until end of turn.\n{r}: this creature gains haste until end of turn.",
  },
  {
    name: "Sprite Noble",
    oracle: "flying\nother creatures you control with flying get +0/+1.\n{t}: other creatures you control with flying get +1/+0 until end of turn.",
  },
  {
    name: "Scepter of Celebration",
    oracle: "equipped creature gets +2/+0 and has trample.\nwhenever equipped creature deals combat damage to a player, create that many 1/1 green and white citizen creature tokens.\nequip {3}",
  },
  {
    name: "Wooden Cane",
    oracle: "when this equipment enters, create a 2/2 red mutant creature token, then attach this equipment to it.\nequipped creature gets +2/+1.\nequip {3}",
  },
  {
    name: "Psychatog",
    oracle: "discard a card: this creature gets +1/+1 until end of turn.\nexile two cards from your graveyard: this creature gets +1/+1 until end of turn.",
  },
  {
    name: "Seeds of Strength",
    oracle: "target creature gets +1/+1 until end of turn.\ntarget creature gets +1/+1 until end of turn.\ntarget creature gets +1/+1 until end of turn.",
  },
  {
    name: "Sporesower Thallid",
    oracle: "at the beginning of your upkeep, put a spore counter on each fungus you control.\nremove three spore counters from this creature: create a 1/1 green saproling creature token.",
  },
  {
    name: "Thunderscape Master",
    oracle: "{b}{b}, {t}: target player loses 2 life and you gain 2 life.\n{g}{g}, {t}: creatures you control get +2/+2 until end of turn.",
  },
  {
    name: "Sarcatog",
    oracle: "exile two cards from your graveyard: this creature gets +1/+1 until end of turn.\nsacrifice an artifact: this creature gets +1/+1 until end of turn.",
  },
  {
    name: "Falcon's Wing Harness",
    oracle: "when this equipment enters, attach it to target creature you control.\nequipped creature gets +1/+1 and has flying and ward {1}. \nequip {2}{u}",
  },
  {
    name: "Obsessive Stitcher",
    oracle: "{t}: draw a card, then discard a card.\n{2}{u}{b}, {t}, sacrifice this creature: return target creature card from your graveyard to the battlefield.",
  },
  {
    name: "Havengul Runebinder",
    oracle: "{2}{u}, {t}, exile a creature card from your graveyard: create a 2/2 black zombie creature token, then put a +1/+1 counter on each zombie creature you control.",
  },
  {
    name: "Mirrodin's Core",
    oracle: "{t}: add {c}.\n{t}: put a charge counter on this land.\n{t}, remove a charge counter from this land: add one mana of any color.",
  },
  {
    name: "Lead Pipe",
    oracle: "equipped creature gets +2/+0.\nwhenever equipped creature dies, each opponent loses 1 life.\n{2}, sacrifice this equipment: draw a card.\nequip {2}",
  },
  {
    name: "Sunhome Guildmage",
    oracle: "{1}{r}{w}: creatures you control get +1/+0 until end of turn.\n{2}{r}{w}: create a 1/1 red and white soldier creature token with haste.",
  },
  {
    name: "Mighty Mutanimals",
    oracle: "when this creature enters, create a 2/2 red mutant creature token.\nalliance — whenever another creature you control enters, put a +1/+1 counter on target creature you control.",
  },
  {
    name: "Discourtesy Clerk",
    oracle: "when this creature enters, open an attraction. \nat the beginning of your end step, if you control three or more attractions, you draw a card and you lose 1 life.",
  },
  {
    name: "Festercreep",
    oracle: "this creature enters with a +1/+1 counter on it.\n{1}{b}, remove a +1/+1 counter from this creature: all other creatures get -1/-1 until end of turn.",
  },
  {
    name: "Gurmag Rakshasa",
    oracle: "menace \nwhen this creature enters, target creature an opponent controls gets -2/-2 until end of turn and target creature you control gets +2/+2 until end of turn.",
  },
  {
    name: "Candlestick",
    oracle: "equipped creature gets +1/+1 and has \"whenever this creature attacks, surveil 2.\" \n{2}, sacrifice this equipment: draw a card.\nequip {2}",
  },
  {
    name: "Herald of Torment",
    oracle: "bestow {3}{b}{b} \nflying\nat the beginning of your upkeep, you lose 1 life.\nenchanted creature gets +3/+3 and has flying.",
  },
  {
    name: "Emberstrike Duo",
    oracle: "whenever you cast a black spell, this creature gets +1/+1 until end of turn.\nwhenever you cast a red spell, this creature gains first strike until end of turn.",
  },
  {
    name: "Archon of Sun's Grace",
    oracle: "flying\nlifelink \npegasus creatures you control have lifelink.\nconstellation — whenever an enchantment you control enters, create a 2/2 white pegasus creature token with flying.",
  },
  {
    name: "Elixir of Vitality",
    oracle: "this artifact enters tapped.\n{t}, sacrifice this artifact: you gain 4 life.\n{8}, {t}, sacrifice this artifact: you gain 8 life.",
  },
  {
    name: "Ongoing Investigation",
    oracle: "whenever one or more creatures you control deal combat damage to a player, investigate. \n{1}{g}, exile a creature card from your graveyard: investigate. you gain 2 life.",
  },
  {
    name: "Sephiroth, Planet's Heir",
    oracle: "vigilance \nwhen ~ enters, creatures your opponents control get -2/-2 until end of turn.\nwhenever a creature an opponent controls dies, put a +1/+1 counter on ~.",
  },
  {
    name: "Drey Keeper",
    oracle: "when this creature enters, create two 1/1 green squirrel creature tokens.\n{3}{b}: squirrels you control get +1/+0 and gain menace until end of turn.",
  },
  {
    name: "Safehold Duo",
    oracle: "whenever you cast a green spell, this creature gets +1/+1 until end of turn.\nwhenever you cast a white spell, this creature gains vigilance until end of turn.",
  },
  {
    name: "Scrap Compactor",
    oracle: "{3}, {t}, sacrifice this artifact: it deals 3 damage to target creature.\n{6}, {t}, sacrifice this artifact: destroy target creature or vehicle.",
  },
  {
    name: "Aphemia, the Cacophony",
    oracle: "flying\nat the beginning of your end step, you may exile an enchantment card from your graveyard. if you do, create a 2/2 black zombie creature token.",
  },
  {
    name: "Might of Murasa",
    oracle: "kicker {2}{g} \ntarget creature gets +3/+3 until end of turn. if this spell was kicked, that creature gets +5/+5 until end of turn instead.",
  },
  {
    name: "Captain of the Watch",
    oracle: "vigilance \nother soldier creatures you control get +1/+1 and have vigilance.\nwhen this creature enters, create three 1/1 white soldier creature tokens.",
  },
  {
    name: "Tattermunge Duo",
    oracle: "whenever you cast a red spell, this creature gets +1/+1 until end of turn.\nwhenever you cast a green spell, this creature gains forestwalk until end of turn.",
  },
  {
    name: "Channeler Initiate",
    oracle: "when this creature enters, put three -1/-1 counters on target creature you control.\n{t}, remove a -1/-1 counter from this creature: add one mana of any color.",
  },
  {
    name: "Gravelgill Duo",
    oracle: "whenever you cast a blue spell, this creature gets +1/+1 until end of turn.\nwhenever you cast a black spell, this creature gains fear until end of turn.",
  },
  {
    name: "Phyrexian Plaguelord",
    oracle: "{t}, sacrifice this creature: target creature gets -4/-4 until end of turn.\nsacrifice a creature: target creature gets -1/-1 until end of turn.",
  },
  {
    name: "Five-Alarm Fire",
    oracle: "whenever a creature you control deals combat damage, put a blaze counter on this enchantment.\nremove five blaze counters from this enchantment: it deals 5 damage to any target.",
  },
  {
    name: "Wand of the Elements",
    oracle: "{t}, sacrifice an island: create a 2/2 blue elemental creature token with flying.\n{t}, sacrifice a mountain: create a 3/3 red elemental creature token.",
  },
  {
    name: "Hunted Bonebrute",
    oracle: "menace\nwhen this creature enters, target opponent creates two 1/1 white dog creature tokens.\nwhen this creature dies, each opponent loses 3 life.\ndisguise {1}{b}",
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
    name: "Thistledown Duo",
    oracle: "whenever you cast a white spell, this creature gets +1/+1 until end of turn.\nwhenever you cast a blue spell, this creature gains flying until end of turn.",
  },
  {
    name: "Sandstone Bridge",
    oracle: "this land enters tapped.\nwhen this land enters, target creature gets +1/+1 and gains vigilance until end of turn.\n{t}: add {w}.",
  },
  {
    name: "Shining Armor",
    oracle: "flash\nwhen this equipment enters, attach it to target knight you control.\nequipped creature gets +0/+2 and has vigilance.\nequip {3}",
  },
  {
    name: "Thorn Thallid",
    oracle: "at the beginning of your upkeep, put a spore counter on this creature.\nremove three spore counters from this creature: it deals 1 damage to any target.",
  },
  {
    name: "Swarm Guildmage",
    oracle: "{4}{b}, {t}: creatures you control get +1/+0 and gain menace until end of turn. \n{1}{g}, {t}: you gain 2 life.",
  },
  {
    name: "Lightning Spear",
    oracle: "equipped creature gets +1/+0 and has trample.\n{2}{r}, sacrifice this equipment: it deals 3 damage to any target.\nequip {1}",
  },
  {
    name: "Deathbringer Thoctar",
    oracle: "whenever another creature dies, you may put a +1/+1 counter on this creature.\nremove a +1/+1 counter from this creature: it deals 1 damage to any target.",
  },
  {
    name: "Abomination of Gudul",
    oracle: "flying\nwhenever this creature deals combat damage to a player, you may draw a card. if you do, discard a card.\nmorph {2}{b}{g}{u}",
  },
  {
    name: "Grim Bauble",
    oracle: "when this artifact enters, target creature an opponent controls gets -2/-2 until end of turn.\n{2}{b}, {t}, sacrifice this artifact: surveil 2.",
  },
  {
    name: "Thallid",
    oracle: "at the beginning of your upkeep, put a spore counter on this creature.\nremove three spore counters from this creature: create a 1/1 green saproling creature token.",
  },
  {
    name: "Squire's Devotion",
    oracle: "enchant creature\nenchanted creature gets +1/+1 and has lifelink.\nwhen this aura enters, create a 1/1 white vampire creature token with lifelink.",
  },
  {
    name: "Faerie Noble",
    oracle: "flying\nother faerie creatures you control get +0/+1.\n{t}: other faerie creatures you control get +1/+0 until end of turn.",
  },
  {
    name: "Ambling Stormshell",
    oracle: "ward {2}\nwhenever this creature attacks, put three stun counters on it and draw three cards. \nwhenever you cast a turtle spell, untap this creature.",
  },
  {
    name: "Unyaro Bees",
    oracle: "flying\n{g}: this creature gets +1/+1 until end of turn.\n{3}{g}, sacrifice this creature: it deals 2 damage to any target.",
  },
  {
    name: "Looming Spires",
    oracle: "this land enters tapped.\nwhen this land enters, target creature gets +1/+1 and gains first strike until end of turn.\n{t}: add {r}.",
  },
  {
    name: "Thalia's Lieutenant",
    oracle: "when this creature enters, put a +1/+1 counter on each other human you control.\nwhenever another human you control enters, put a +1/+1 counter on this creature.",
  },
  {
    name: "Creeping Trailblazer",
    oracle: "other elementals you control get +1/+0.\n{2}{r}{g}: this creature gets +1/+1 until end of turn for each elemental you control.",
  },
  {
    name: "Explosive Growth",
    oracle: "kicker {5} \ntarget creature gets +2/+2 until end of turn. if this spell was kicked, that creature gets +5/+5 until end of turn instead.",
  },
  {
    name: "Knightly Valor",
    oracle: "enchant creature\nwhen this aura enters, create a 2/2 white knight creature token with vigilance. \nenchanted creature gets +2/+2 and has vigilance.",
  },
  {
    name: "Regal Caracal",
    oracle: "other cats you control get +1/+1 and have lifelink. \nwhen this creature enters, create two 1/1 white cat creature tokens with lifelink.",
  },
  {
    name: "Bishop of Wings",
    oracle: "whenever an angel you control enters, you gain 4 life.\nwhenever an angel you control dies, create a 1/1 white spirit creature token with flying.",
  },
  {
    name: "Ipnu Rivulet",
    oracle: "{t}: add {c}.\n{t}, pay 1 life: add {u}.\n{1}{u}, {t}, sacrifice a desert: target player mills four cards.",
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
    name: "Siege-Gang Commander",
    oracle: "when this creature enters, create three 1/1 red goblin creature tokens.\n{1}{r}, sacrifice a goblin: this creature deals 2 damage to any target.",
  },
  {
    name: "Dismissive Pyromancer",
    oracle: "{r}, {t}, discard a card: draw a card.\n{2}{r}, {t}, sacrifice this creature: it deals 4 damage to target creature.",
  },
  {
    name: "Forebear's Blade",
    oracle: "equipped creature gets +3/+0 and has vigilance and trample.\nwhenever equipped creature dies, attach this equipment to target creature you control.\nequip {3}",
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
    name: "Comet Crawler",
    oracle: "lifelink\nwhenever this creature attacks, you may sacrifice another creature or artifact. if you do, this creature gets +2/+0 until end of turn.",
  },
  {
    name: "Fireforger's Puzzleknot",
    oracle: "when this artifact enters, it deals 1 damage to any target.\n{2}{r}, sacrifice this artifact: it deals 1 damage to any target.",
  },
  {
    name: "Auriok Survivors",
    oracle: "when this creature enters, you may return target equipment card from your graveyard to the battlefield. if you do, you may attach it to this creature.",
  },
  {
    name: "Choking Miasma",
    oracle: "kicker {g} \nif this spell was kicked, put a +1/+1 counter on a creature you control.\nall creatures get -2/-2 until end of turn.",
  },
  {
    name: "Masked Vandal",
    oracle: "changeling \nwhen this creature enters, you may exile a creature card from your graveyard. if you do, exile target artifact or enchantment an opponent controls.",
  },
  {
    name: "Veteran Beastrider",
    oracle: "at the beginning of your end step, untap each creature you control.\n{2}{g}{w}: creatures you control get +1/+1 until end of turn.",
  },
  {
    name: "Rousing Read",
    oracle: "enchant creature\nwhen this aura enters, draw two cards, then discard a card.\nenchanted creature gets +1/+1 and has flying.",
  },
  {
    name: "Skeleton Key",
    oracle: "equipped creature has skulk. \nwhenever equipped creature deals combat damage to a player, you may draw a card. if you do, discard a card.\nequip {2}",
  },
  {
    name: "Gnawing Vermin",
    oracle: "when this creature enters, target player mills two cards.\nwhen this creature dies, target creature you don't control gets -1/-1 until end of turn.",
  },
  {
    name: "Goblin Dynamo",
    oracle: "{t}: this creature deals 1 damage to any target.\n{x}{r}, {t}, sacrifice this creature: it deals x damage to any target.",
  },
  {
    name: "Golgari Guildmage",
    oracle: "{4}{b}, sacrifice a creature: return target creature card from your graveyard to your hand.\n{4}{g}: put a +1/+1 counter on target creature.",
  },
  {
    name: "Lithatog",
    oracle: "sacrifice an artifact: this creature gets +1/+1 until end of turn.\nsacrifice a land: this creature gets +1/+1 until end of turn.",
  },
  {
    name: "Carnifex Demon",
    oracle: "flying\nthis creature enters with two -1/-1 counters on it.\n{b}, remove a -1/-1 counter from this creature: put a -1/-1 counter on each other creature.",
  },
  {
    name: "Moorland Haunt",
    oracle: "{t}: add {c}.\n{w}{u}, {t}, exile a creature card from your graveyard: create a 1/1 white spirit creature token with flying.",
  },
  {
    name: "Clachan Festival",
    oracle: "when this enchantment enters, create two 1/1 green and white kithkin creature tokens.\n{4}{w}: create a 1/1 green and white kithkin creature token.",
  },
  {
    name: "Seshiro the Anointed",
    oracle: "other snake creatures you control get +2/+2.\nwhenever a snake you control deals combat damage to a player, you may draw a card.",
  },
  {
    name: "Kjeldoran Knight",
    oracle: "banding \n{1}{w}: this creature gets +1/+0 until end of turn.\n{w}{w}: this creature gets +0/+2 until end of turn.",
  },
  {
    name: "Gitaxian Raptor",
    oracle: "flying\nthis creature enters with three oil counters on it.\nremove an oil counter from this creature: this creature gets +1/-1 until end of turn.",
  },
  {
    name: "Hanged Executioner",
    oracle: "flying\nwhen this creature enters, create a 1/1 white spirit creature token with flying.\n{3}{w}, exile this creature: exile target creature.",
  },
  {
    name: "Cloak and Dagger",
    oracle: "equipped creature gets +2/+0 and has shroud. \nwhenever a rogue creature enters, you may attach this equipment to it.\nequip {3}",
  },
  {
    name: "Fortress Cyclops",
    oracle: "whenever this creature attacks, it gets +3/+0 until end of turn.\nwhenever this creature blocks, it gets +0/+3 until end of turn.",
  },
  {
    name: "Armorer Guildmage",
    oracle: "{b}, {t}: target creature gets +1/+0 until end of turn.\n{g}, {t}: target creature gets +0/+1 until end of turn.",
  },
  {
    name: "Thaumatog",
    oracle: "sacrifice a land: this creature gets +1/+1 until end of turn.\nsacrifice an enchantment: this creature gets +1/+1 until end of turn.",
  },
  {
    name: "Cauldron Familiar",
    oracle: "when this creature enters, each opponent loses 1 life and you gain 1 life.\nsacrifice a food: return this card from your graveyard to the battlefield.",
  },
  {
    name: "Slaughter Specialist",
    oracle: "when this creature enters, each opponent creates a 1/1 white human creature token.\nwhenever a creature an opponent controls dies, put a +1/+1 counter on this creature.",
  },
  {
    name: "Inspired Sprite",
    oracle: "flash\nflying\nwhenever you cast a wizard spell, you may untap this creature.\n{t}: draw a card, then discard a card.",
  },
  {
    name: "Griffin Guide",
    oracle: "enchant creature\nenchanted creature gets +2/+2 and has flying.\nwhen enchanted creature dies, create a 2/2 white griffin creature token with flying.",
  },
  {
    name: "Dega Disciple",
    oracle: "{b}, {t}: target creature gets -2/-0 until end of turn.\n{r}, {t}: target creature gets +2/+0 until end of turn.",
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
    name: "Glint-Eye Nephilim",
    oracle: "whenever this creature deals combat damage to a player, draw that many cards.\n{1}, discard a card: this creature gets +1/+1 until end of turn.",
  },
  {
    name: "Alquist Proft, Master Sleuth",
    oracle: "vigilance\nwhen ~ enters, investigate. \n{x}{w}{u}{u}, {t}, sacrifice a clue: you draw x cards and gain x life.",
  },
  {
    name: "Obsidian Battle-Axe",
    oracle: "equipped creature gets +2/+1 and has haste.\nwhenever a warrior creature enters, you may attach this equipment to it.\nequip {3}",
  },
  {
    name: "Memorial to Glory",
    oracle: "this land enters tapped.\n{t}: add {w}.\n{3}{w}, {t}, sacrifice this land: create two 1/1 white soldier creature tokens.",
  },
  {
    name: "Phantatog",
    oracle: "sacrifice an enchantment: this creature gets +1/+1 until end of turn.\ndiscard a card: this creature gets +1/+1 until end of turn.",
  },
  {
    name: "Beacon Hawk",
    oracle: "flying\nwhenever this creature deals combat damage to a player, you may untap target creature.\n{w}: this creature gets +0/+1 until end of turn.",
  },
  {
    name: "Plague Belcher",
    oracle: "menace\nwhen this creature enters, put two -1/-1 counters on target creature you control.\nwhenever another zombie you control dies, each opponent loses 1 life.",
  },
  {
    name: "Thrumming Hivepool",
    oracle: "affinity for slivers \nslivers you control have double strike and haste.\nat the beginning of your upkeep, create two 1/1 colorless sliver creature tokens.",
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
    name: "Keldon Mantle",
    oracle: "enchant creature\n{b}: regenerate enchanted creature.\n{r}: enchanted creature gets +1/+0 until end of turn.\n{g}: enchanted creature gains trample until end of turn.",
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
    name: "Meltstrider's Gear",
    oracle: "when this equipment enters, attach it to target creature you control.\nequipped creature gets +2/+1 and has reach.\nequip {5}",
  },
  {
    name: "Talons of Wildwood",
    oracle: "enchant creature\nenchanted creature gets +1/+1 and has trample. \n{2}{g}: return this card from your graveyard to your hand.",
  },
  {
    name: "Sunseed Nurturer",
    oracle: "at the beginning of your end step, if you control a creature with power 5 or greater, you may gain 2 life.\n{t}: add {c}.",
  },
  {
    name: "Heir of the Wilds",
    oracle: "deathtouch\nferocious — whenever this creature attacks, if you control a creature with power 4 or greater, this creature gets +1/+1 until end of turn.",
  },
  {
    name: "Drumhunter",
    oracle: "at the beginning of your end step, if you control a creature with power 5 or greater, you may draw a card.\n{t}: add {c}.",
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
    name: "Priest of the Blood Rite",
    oracle: "when this creature enters, create a 5/5 black demon creature token with flying.\nat the beginning of your upkeep, you lose 2 life.",
  },
  {
    name: "Skybridge Towers",
    oracle: "this land enters tapped.\n{t}: add {w} or {u}.\n{2}{w}{u}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Sling-Gang Lieutenant",
    oracle: "when this creature enters, create two 1/1 red goblin creature tokens.\nsacrifice a goblin: target player loses 1 life and you gain 1 life.",
  },
  {
    name: "Trostani, Three Whispers",
    oracle: "{1}{g}: target creature gains deathtouch until end of turn.\n{g/w}: target creature gains vigilance until end of turn.\n{2}{w}: target creature gains double strike until end of turn.",
  },
  {
    name: "Bitterbloom Bearer",
    oracle: "flash\nflying\nat the beginning of your upkeep, you lose 1 life and create a 1/1 blue and black faerie creature token with flying.",
  },
  {
    name: "Spitting Hydra",
    oracle: "this creature enters with four +1/+1 counters on it.\n{1}{r}, remove a +1/+1 counter from this creature: it deals 1 damage to target creature.",
  },
  {
    name: "Snake Umbra",
    oracle: "enchant creature\nenchanted creature gets +1/+1 and has \"whenever this creature deals damage to an opponent, you may draw a card.\"\numbra armor",
  },
  {
    name: "Draconic Disciple",
    oracle: "{t}: add one mana of any color.\n{7}, {t}, sacrifice this creature: create a 5/5 red dragon creature token with flying.",
  },
  {
    name: "Wild Onslaught",
    oracle: "kicker {4} \nput a +1/+1 counter on each creature you control. if this spell was kicked, put two +1/+1 counters on each creature you control instead.",
  },
  {
    name: "Racers' Ring",
    oracle: "this land enters tapped.\n{t}: add {r} or {g}.\n{2}{r}{g}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Teysa, Orzhov Scion",
    oracle: "sacrifice three white creatures: exile target creature.\nwhenever another black creature you control dies, create a 1/1 white spirit creature token with flying.",
  },
  {
    name: "Rumble Arena",
    oracle: "vigilance\nwhen this land enters, scry 1. \n{t}: add {c}.\n{1}, {t}: add one mana of any color.",
  },
  {
    name: "Slumbering Keepguard",
    oracle: "whenever an enchantment you control enters, scry 1.\n{2}{w}: this creature gets +1/+1 until end of turn for each enchantment you control.",
  },
  {
    name: "Forgotten Harvest",
    oracle: "at the beginning of your upkeep, you may exile a land card from your graveyard. if you do, put a +1/+1 counter on target creature.",
  },
  {
    name: "Drogskol Cavalry",
    oracle: "flying\nwhenever another spirit you control enters, you gain 2 life.\n{3}{w}: create a 1/1 white spirit creature token with flying.",
  },
  {
    name: "Deranged Hermit",
    oracle: "echo {3}{g}{g} \nwhen this creature enters, create four 1/1 green squirrel creature tokens.\nsquirrel creatures get +1/+1.",
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
    name: "Etched Monstrosity",
    oracle: "this creature enters with five -1/-1 counters on it.\n{w}{u}{b}{r}{g}, remove five -1/-1 counters from this creature: target player draws three cards.",
  },
  {
    name: "Tyrant of Kher Ridges",
    oracle: "flying\nwhen this creature enters, it deals 4 damage to any target.\n{r}: this creature gets +1/+0 until end of turn.",
  },
  {
    name: "Tramway Station",
    oracle: "this land enters tapped.\n{t}: add {b} or {r}.\n{2}{b}{r}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Brawl-Bash Ogre",
    oracle: "menace \nwhenever this creature attacks, you may sacrifice another creature. if you do, this creature gets +2/+2 until end of turn.",
  },
  {
    name: "Dragon Mantle",
    oracle: "enchant creature\nwhen this aura enters, draw a card.\nenchanted creature has \"{r}: this creature gets +1/+0 until end of turn.\"",
  },
  {
    name: "Tibalt's Rager",
    oracle: "when this creature dies, it deals 1 damage to any target.\n{1}{r}: this creature gets +2/+0 until end of turn.",
  },
  {
    name: "Reaper of the Wilds",
    oracle: "whenever another creature dies, scry 1.\n{b}: this creature gains deathtouch until end of turn.\n{1}{g}: this creature gains hexproof until end of turn.",
  },
  {
    name: "Horned Helm",
    oracle: "equipped creature gets +1/+1 and has trample.\n{g}{g}: attach this equipment to target creature you control.\nequip {1}",
  },
  {
    name: "Ob Nixilis, the Fallen",
    oracle: "landfall — whenever a land you control enters, you may have target player lose 3 life. if you do, put three +1/+1 counters on ~.",
  },
  {
    name: "Icatian Javelineers",
    oracle: "this creature enters with a javelin counter on it.\n{t}, remove a javelin counter from this creature: it deals 1 damage to any target.",
  },
  {
    name: "Soul-Strike Technique",
    oracle: "enchant creature\nenchanted creature gets +1/+1 and has vigilance.\nwhen enchanted creature dies, manifest the top card of your library.",
  },
  {
    name: "Ironclad Revolutionary",
    oracle: "when this creature enters, you may sacrifice an artifact. if you do, put two +1/+1 counters on this creature and each opponent loses 2 life.",
  },
  {
    name: "Blasting Station",
    oracle: "{t}, sacrifice a creature: this artifact deals 1 damage to any target.\nwhenever a creature enters, you may untap this artifact.",
  },
  {
    name: "Teetering Peaks",
    oracle: "this land enters tapped.\nwhen this land enters, target creature gets +2/+0 until end of turn.\n{t}: add {r}.",
  },
  {
    name: "Boartusk Liege",
    oracle: "trample\nother red creatures you control get +1/+1.\nother green creatures you control get +1/+1.",
  },
  {
    name: "Stormrider Rig",
    oracle: "equipped creature gets +1/+1.\nwhenever a creature you control enters, you may attach this equipment to it.\nequip {2}",
  },
  {
    name: "Havengul Vampire",
    oracle: "whenever this creature deals combat damage to a player, put a +1/+1 counter on it.\nwhenever another creature dies, put a +1/+1 counter on this creature.",
  },
  {
    name: "Turntimber Grove",
    oracle: "this land enters tapped.\nwhen this land enters, target creature gets +1/+1 until end of turn.\n{t}: add {g}.",
  },
  {
    name: "Cave People",
    oracle: "whenever this creature attacks, it gets +1/-2 until end of turn.\n{1}{r}{r}, {t}: target creature gains mountainwalk until end of turn.",
  },
  {
    name: "Diregraf Captain",
    oracle: "deathtouch\nother zombie creatures you control get +1/+1.\nwhenever another zombie you control dies, target opponent loses 1 life.",
  },
  {
    name: "Shapers of Nature",
    oracle: "{3}{g}: put a +1/+1 counter on target creature.\n{2}{u}, remove a +1/+1 counter from a creature you control: draw a card.",
  },
  {
    name: "Lord of the Undead",
    oracle: "other zombie creatures get +1/+1.\n{1}{b}, {t}: return target zombie card from your graveyard to your hand.",
  },
  {
    name: "Mortarpod",
    oracle: "living weapon \nequipped creature gets +0/+1 and has \"sacrifice this creature: this creature deals 1 damage to any target.\"\nequip {2}",
  },
  {
    name: "Thistledown Liege",
    oracle: "flash\nother white creatures you control get +1/+1.\nother blue creatures you control get +1/+1.",
  },
  {
    name: "Dueling Rapier",
    oracle: "flash\nwhen this equipment enters, attach it to target creature you control.\nequipped creature gets +2/+0.\nequip {4}",
  },
  {
    name: "Deep Forest Hermit",
    oracle: "vanishing 3 \nwhen this creature enters, create four 1/1 green squirrel creature tokens.\nsquirrels you control get +1/+1.",
  },
  {
    name: "Viashivan Dragon",
    oracle: "flying\n{r}: this creature gets +1/+0 until end of turn.\n{g}: this creature gets +0/+1 until end of turn.",
  },
  {
    name: "Super Speed",
    oracle: "flash\nenchant creature\nwhen this aura enters, enchanted creature gains first strike until end of turn.\nenchanted creature gets +1/+0 and has haste.",
  },
  {
    name: "Blade of the Bloodchief",
    oracle: "whenever a creature dies, put a +1/+1 counter on equipped creature. if equipped creature is a vampire, put two +1/+1 counters on it instead.\nequip {1}",
  },
  {
    name: "Niv-Mizzet, Dracogenius",
    oracle: "flying\nwhenever ~ deals damage to a player, you may draw a card.\n{u}{r}: ~ deals 1 damage to any target.",
  },
  {
    name: "Salt Road Quartermasters",
    oracle: "this creature enters with two +1/+1 counters on it.\n{2}{g}, remove a +1/+1 counter from this creature: put a +1/+1 counter on target creature.",
  },
  {
    name: "Glen Elendra Liege",
    oracle: "flying\nother blue creatures you control get +1/+1.\nother black creatures you control get +1/+1.",
  },
  {
    name: "Darling of the Masses",
    oracle: "other citizens you control get +1/+0.\nwhenever this creature attacks, create a 1/1 green and white citizen creature token.",
  },
  {
    name: "Malamet Scythe",
    oracle: "flash\nwhen this equipment enters, attach it to target creature you control.\nequipped creature gets +2/+2.\nequip {4}",
  },
  {
    name: "Elas il-Kor, Sadistic Pilgrim",
    oracle: "deathtouch\nwhenever another creature you control enters, you gain 1 life.\nwhenever another creature you control dies, each opponent loses 1 life.",
  },
  {
    name: "Axiom Engraver",
    oracle: "this creature enters with two oil counters on it.\n{t}, remove an oil counter from this creature, discard a card: draw a card.",
  },
  {
    name: "Mirran Banesplitter",
    oracle: "flash\nwhen this equipment enters, attach it to target creature you control.\nequipped creature gets +2/+0.\nequip {3}",
  },
  {
    name: "Firewake Sliver",
    oracle: "all sliver creatures have haste.\nall slivers have \"{1}, sacrifice this permanent: target sliver creature gets +2/+2 until end of turn.\"",
  },
  {
    name: "Monoskelion",
    oracle: "this creature enters with a +1/+1 counter on it.\n{1}, remove a +1/+1 counter from this creature: it deals 1 damage to any target.",
  },
  {
    name: "Despoiler of Souls",
    oracle: "this creature can't block.\n{b}{b}, exile two other creature cards from your graveyard: return this card from your graveyard to the battlefield.",
  },
  {
    name: "Fae Flight",
    oracle: "flash\nenchant creature\nwhen this aura enters, enchanted creature gains hexproof until end of turn.\nenchanted creature gets +1/+0 and has flying.",
  },
  {
    name: "Paladin's Shield",
    oracle: "flash\nwhen this equipment enters, attach it to target creature you control.\nequipped creature gets +0/+2.\nequip {3}",
  },
  {
    name: "Sai of the Shinobi",
    oracle: "equipped creature gets +1/+1.\nwhenever a creature you control enters, you may attach this equipment to it.\nequip {2}",
  },
  {
    name: "Inventor's Goggles",
    oracle: "equipped creature gets +1/+2.\nwhenever an artificer you control enters, you may attach this equipment to it.\nequip {2}",
  },
  {
    name: "Verdant Embrace",
    oracle: "enchant creature\nenchanted creature gets +3/+3 and has \"at the beginning of each upkeep, create a 1/1 green saproling creature token.\"",
  },
  {
    name: "For the Family",
    oracle: "target creature gets +2/+2 until end of turn. if you control four or more creatures, that creature gets +4/+4 until end of turn instead.",
  },
  {
    name: "Thassa's Emissary",
    oracle: "bestow {5}{u} \nwhenever this creature or enchanted creature deals combat damage to a player, draw a card.\nenchanted creature gets +3/+3.",
  },
  {
    name: "Airship Engine Room",
    oracle: "this land enters tapped.\n{t}: add {u} or {r}.\n{4}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Staggering Insight",
    oracle: "enchant creature\nenchanted creature gets +1/+1 and has lifelink and \"whenever this creature deals combat damage to a player, draw a card.\"",
  },
  {
    name: "Falcon and Redwing",
    oracle: "flying\nwhenever ~ deal combat damage to a player, create that many 1/1 white bird creature tokens with flying, then put a +1/+1 counter on ~.",
  },
  {
    name: "Hellkite Igniter",
    oracle: "flying, haste\n{1}{r}: this creature gets +x/+0 until end of turn, where x is the number of artifacts you control.",
  },
  {
    name: "Spike Colony",
    oracle: "this creature enters with four +1/+1 counters on it.\n{2}, remove a +1/+1 counter from this creature: put a +1/+1 counter on target creature.",
  },
  {
    name: "Consuming Fervor",
    oracle: "enchant creature\nenchanted creature gets +3/+3 and has \"at the beginning of your upkeep, put a -1/-1 counter on this creature.\"",
  },
  {
    name: "Heliod's Emissary",
    oracle: "bestow {6}{w} \nwhenever this creature or enchanted creature attacks, tap target creature an opponent controls.\nenchanted creature gets +3/+3.",
  },
  {
    name: "Hellkite Overlord",
    oracle: "flying, trample, haste\n{r}: this creature gets +1/+0 until end of turn.\n{b}{g}: regenerate this creature.",
  },
  {
    name: "Ceta Disciple",
    oracle: "{r}, {t}: target creature gets +2/+0 until end of turn.\n{g}, {t}: add one mana of any color.",
  },
  {
    name: "Storm God's Oracle",
    oracle: "{1}: this creature gets +1/-1 until end of turn.\nwhen this creature dies, it deals 3 damage to any target.",
  },
  {
    name: "Stone Haven Pilgrim",
    oracle: "whenever this creature attacks, if you control an artifact or enchantment, this creature gets +1/+1 and gains lifelink until end of turn.",
  },
  {
    name: "Scavenged Blade",
    oracle: "when this equipment enters, attach it to target creature you control.\nequipped creature gets +2/+0.\nequip {2}{r}",
  },
  {
    name: "Fyndhorn Pollen",
    oracle: "cumulative upkeep {1} \nall creatures get -1/-0.\n{1}{g}: all creatures get -1/-0 until end of turn.",
  },
  {
    name: "Ancestral Vengeance",
    oracle: "enchant creature\nwhen this aura enters, put a +1/+1 counter on target creature you control.\nenchanted creature gets -1/-1.",
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
    name: "Full Moon's Rise",
    oracle: "werewolf creatures you control get +1/+0 and have trample.\nsacrifice this enchantment: regenerate all werewolf creatures you control.",
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
    name: "Marker Beetles",
    oracle: "when this creature dies, target creature gets +1/+1 until end of turn.\n{2}, sacrifice this creature: draw a card.",
  },
  {
    name: "Undercity Scavenger",
    oracle: "when this creature enters, you may sacrifice another creature. if you do, put two +1/+1 counters on this creature, then scry 2.",
  },
  {
    name: "Nemata, Grove Guardian",
    oracle: "{2}{g}: create a 1/1 green saproling creature token.\nsacrifice a saproling: saproling creatures get +1/+1 until end of turn.",
  },
  {
    name: "Pack Guardian",
    oracle: "flash\nwhen this creature enters, you may discard a land card. if you do, create a 2/2 green wolf creature token.",
  },
  {
    name: "Blade-Tribe Berserkers",
    oracle: "metalcraft — when this creature enters, if you control three or more artifacts, this creature gets +3/+3 and gains haste until end of turn.",
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
    name: "Glade of the Pump Spells",
    oracle: "when ~ enters the battlefield, up to one target creature gets +2/+2 and gains trample until end of turn.\n{t}: add {g}{g}.",
  },
  {
    name: "Candy Trail",
    oracle: "when this artifact enters, scry 2.\n{2}, {t}, sacrifice this artifact: you gain 3 life and draw a card.",
  },
  {
    name: "Ornery Dilophosaur",
    oracle: "deathtouch \nwhenever this creature attacks, if you control a creature with power 4 or greater, this creature gets +2/+2 until end of turn.",
  },
  {
    name: "Shinewend",
    oracle: "flying\nthis creature enters with a +1/+1 counter on it.\n{1}{w}, remove a +1/+1 counter from this creature: destroy target enchantment.",
  },
  {
    name: "Selesnya Guildmage",
    oracle: "{3}{g}: create a 1/1 green saproling creature token.\n{3}{w}: creatures you control get +1/+1 until end of turn.",
  },
  {
    name: "Ironhoof Boar",
    oracle: "trample, haste\nchannel — {1}{r}, discard this card: target creature gets +3/+1 and gains trample until end of turn.",
  },
  {
    name: "Spinnerette, Arachnobat",
    oracle: "reach\nwhen ~ enters, open an attraction.\nas long as you control three or more attractions, ~ gets +2/+0 and has menace.",
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
    name: "You're Not Alone",
    oracle: "target creature gets +2/+2 until end of turn. if you control three or more creatures, it gets +4/+4 until end of turn instead.",
  },
  {
    name: "Astral Wingspan",
    oracle: "convoke \nenchant creature\nwhen this aura enters, draw a card.\nenchanted creature gets +2/+2 and has flying.",
  },
  {
    name: "Infernal Idol",
    oracle: "{t}: add {b}.\n{1}{b}{b}, {t}, sacrifice this artifact: you draw two cards and you lose 2 life.",
  },
  {
    name: "Collector's Case",
    oracle: "when this artifact enters, tap up to one target creature and put two stun counters on it. \n{3}{u}, {t}: tap target creature.",
  },
  {
    name: "Surveillance Room",
    oracle: "when this land enters, surveil 1. \n{t}: add {c}.\n{1}, {t}: add one mana of any color.",
  },
  {
    name: "Plague Dogs",
    oracle: "when this creature dies, all creatures get -1/-1 until end of turn.\n{2}, sacrifice this creature: draw a card.",
  },
  {
    name: "Foggy Bottom Swamp",
    oracle: "this land enters tapped.\n{t}: add {b} or {g}.\n{4}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Feather of Flight",
    oracle: "flash\nenchant creature\nwhen this aura enters, draw a card.\nenchanted creature gets +1/+0 and has flying.",
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
    name: "Graf Harvest",
    oracle: "zombies you control have menace. \n{3}{b}, exile a creature card from your graveyard: create a 2/2 black zombie creature token.",
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
    name: "Omen of the Forge",
    oracle: "flash\nwhen this enchantment enters, it deals 2 damage to any target.\n{2}{r}, sacrifice this enchantment: scry 2.",
  },
  {
    name: "Steelbane Hydra",
    oracle: "this creature enters with x +1/+1 counters on it.\n{2}{g}, remove a +1/+1 counter from this creature: destroy target artifact or enchantment.",
  },
  {
    name: "Shiv's Embrace",
    oracle: "enchant creature\nenchanted creature gets +2/+2 and has flying.\n{r}: enchanted creature gets +1/+0 until end of turn.",
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
    name: "Codex Shredder",
    oracle: "{t}: target player mills a card. \n{5}, {t}, sacrifice this artifact: return target card from your graveyard to your hand.",
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
    name: "Lofty Dreams",
    oracle: "convoke \nenchant creature\nwhen this aura enters, draw a card.\nenchanted creature gets +2/+2 and has flying.",
  },
  {
    name: "Stone Kavu",
    oracle: "{r}: this creature gets +1/+0 until end of turn.\n{w}: this creature gets +0/+1 until end of turn.",
  },
  {
    name: "Tumble Magnet",
    oracle: "this artifact enters with three charge counters on it.\n{t}, remove a charge counter from this artifact: tap target artifact or creature.",
  },
  {
    name: "Burner Rocket",
    oracle: "flash\nwhen this vehicle enters, target creature you control gets +2/+0 and gains trample until end of turn.\ncrew 1",
  },
  {
    name: "Armadillo Cloak",
    oracle: "enchant creature\nenchanted creature gets +2/+2 and has trample.\nwhenever enchanted creature deals damage, you gain that much life.",
  },
  {
    name: "Crawl from the Cellar",
    oracle: "return target creature card from your graveyard to your hand. put a +1/+1 counter on up to one target zombie you control.\nflashback {3}{b}",
  },
  {
    name: "Lightning Diadem",
    oracle: "enchant creature\nwhen this aura enters, it deals 2 damage to any target.\nenchanted creature gets +2/+2.",
  },
  {
    name: "Instant Ramen",
    oracle: "flash\nwhen this artifact enters, draw a card.\n{2}, {t}, sacrifice this artifact: you gain 3 life.",
  },
  {
    name: "Bramble Armor",
    oracle: "when this equipment enters, attach it to target creature you control.\nequipped creature gets +2/+1.\nequip {4}",
  },
  {
    name: "Timberland Ruins",
    oracle: "this land enters tapped.\n{t}: add {g}.\n{t}, sacrifice this land: add one mana of any color.",
  },
  {
    name: "Truefire Paladin",
    oracle: "vigilance\n{r}{w}: this creature gets +2/+0 until end of turn.\n{r}{w}: this creature gains first strike until end of turn.",
  },
  {
    name: "Pyrite Spellbomb",
    oracle: "{r}, sacrifice this artifact: it deals 2 damage to any target.\n{1}, sacrifice this artifact: draw a card.",
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
    name: "Manhole Cover",
    oracle: "flash \nwhen this artifact enters, target creature gains indestructible until end of turn. \n{2}, sacrifice this artifact: target player draws a card.",
  },
  {
    name: "Steel Wrecking Ball",
    oracle: "when this artifact enters, it deals 5 damage to target creature.\n{1}{r}, discard this card: destroy target artifact.",
  },
  {
    name: "Wickerbough Elder",
    oracle: "this creature enters with a -1/-1 counter on it.\n{g}, remove a -1/-1 counter from this creature: destroy target artifact or enchantment.",
  },
  {
    name: "Bear Umbra",
    oracle: "enchant creature\nenchanted creature gets +2/+2 and has \"whenever this creature attacks, untap all lands you control.\"\numbra armor",
  },
  {
    name: "Abzan Banner",
    oracle: "{t}: add {w}, {b}, or {g}.\n{w}{b}{g}, {t}, sacrifice this artifact: draw a card.",
  },
  {
    name: "Haunted Hellride",
    oracle: "whenever you attack, target creature you control gets +1/+0 and gains deathtouch until end of turn. untap it.\ncrew 1",
  },
  {
    name: "Kaleidostone",
    oracle: "when this artifact enters, draw a card.\n{5}, {t}, sacrifice this artifact: add {w}{u}{b}{r}{g}.",
  },
  {
    name: "Wolfkin Bond",
    oracle: "enchant creature\nwhen this aura enters, create a 2/2 green wolf creature token.\nenchanted creature gets +2/+2.",
  },
  {
    name: "Waylaying Pirates",
    oracle: "when this creature enters, if you control an artifact, tap target artifact or creature an opponent controls and put a stun counter on it.",
  },
  {
    name: "Abandoned Outpost",
    oracle: "this land enters tapped.\n{t}: add {w}.\n{t}, sacrifice this land: add one mana of any color.",
  },
  {
    name: "Ultimecia, Temporal Threat",
    oracle: "when ~ enters, tap all creatures your opponents control.\nwhenever a creature you control deals combat damage to a player, draw a card.",
  },
  {
    name: "Triclopean Sight",
    oracle: "flash\nenchant creature\nwhen this aura enters, untap enchanted creature.\nenchanted creature gets +1/+1 and has vigilance.",
  },
  {
    name: "Drana, Kalastria Bloodchief",
    oracle: "flying\n{x}{b}{b}: target creature gets -0/-x until end of turn and ~ gets +x/+0 until end of turn.",
  },
  {
    name: "Ravaged Highlands",
    oracle: "this land enters tapped.\n{t}: add {r}.\n{t}, sacrifice this land: add one mana of any color.",
  },
  {
    name: "Triskelion",
    oracle: "this creature enters with three +1/+1 counters on it.\nremove a +1/+1 counter from this creature: it deals 1 damage to any target.",
  },
  {
    name: "Startle",
    oracle: "target creature gets -2/-0 until end of turn. create a 2/2 black zombie creature token with decayed. \ndraw a card.",
  },
  {
    name: "Leonin Battlemage",
    oracle: "{t}: target creature gets +1/+1 until end of turn.\nwhenever you cast a spell, you may untap this creature.",
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
    name: "Glider Staff",
    oracle: "when this equipment enters, airbend up to one target creature. \nequipped creature gets +1/+1 and has flying.\nequip {2}",
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
    name: "Hagra Crocodile",
    oracle: "this creature can't block.\nlandfall — whenever a land you control enters, this creature gets +2/+2 until end of turn.",
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
    name: "Phalanx Tactics",
    oracle: "target creature you control gets +2/+1 until end of turn. each other creature you control gets +1/+1 until end of turn.",
  },
  {
    name: "Briar Shield",
    oracle: "enchant creature\nenchanted creature gets +1/+1.\nsacrifice this aura: enchanted creature gets +3/+3 until end of turn.",
  },
  {
    name: "Bestial Bloodline",
    oracle: "enchant creature\nenchanted creature gets +2/+2.\n{4}{g}: return this card from your graveyard to your hand.",
  },
  {
    name: "Screams from Within",
    oracle: "enchant creature\nenchanted creature gets -1/-1.\nwhen enchanted creature dies, return this card from your graveyard to the battlefield.",
  },
  {
    name: "Wintermoon Mesa",
    oracle: "this land enters tapped.\n{t}: add {c}.\n{2}, {t}, sacrifice this land: tap two target lands.",
  },
  {
    name: "Sunhome Enforcer",
    oracle: "whenever this creature deals combat damage, you gain that much life.\n{1}{r}: this creature gets +1/+0 until end of turn.",
  },
  {
    name: "Lamplighter of Selhoff",
    oracle: "when this creature enters, if you control another zombie, you may draw a card. if you do, discard a card.",
  },
  {
    name: "Pirate's Cutlass",
    oracle: "when this equipment enters, attach it to target pirate you control.\nequipped creature gets +2/+1.\nequip {2}",
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
    name: "Khalni Garden",
    oracle: "this land enters tapped.\nwhen this land enters, create a 0/1 green plant creature token.\n{t}: add {g}.",
  },
  {
    name: "Utility Knife",
    oracle: "when this equipment enters, attach it to target creature you control.\nequipped creature gets +1/+1.\nequip {3}",
  },
  {
    name: "Barkhide Troll",
    oracle: "this creature enters with a +1/+1 counter on it.\n{1}, remove a +1/+1 counter from this creature: this creature gains hexproof until end of turn.",
  },
  {
    name: "Necropolis Fiend",
    oracle: "delve \nflying\n{x}, {t}, exile x cards from your graveyard: target creature gets -x/-x until end of turn.",
  },
  {
    name: "Bog Wreckage",
    oracle: "this land enters tapped.\n{t}: add {b}.\n{t}, sacrifice this land: add one mana of any color.",
  },
  {
    name: "Bruna, the Fading Light",
    oracle: "when you cast this spell, you may return target angel or human creature card from your graveyard to the battlefield.\nflying, vigilance",
  },
  {
    name: "Lion Heart",
    oracle: "when this equipment enters, it deals 2 damage to any target.\nequipped creature gets +2/+1.\nequip {2}",
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
    name: "Kessig Wolf Run",
    oracle: "{t}: add {c}.\n{x}{r}{g}, {t}: target creature gets +x/+0 and gains trample until end of turn.",
  },
  {
    name: "Deepchannel Duelist",
    oracle: "at the beginning of your end step, untap target merfolk you control.\nother merfolk you control get +1/+1.",
  },
  {
    name: "Thundering Spineback",
    oracle: "other dinosaurs you control get +1/+1.\n{5}{g}: create a 3/3 green dinosaur creature token with trample.",
  },
  {
    name: "Iname, Life Aspect",
    oracle: "when ~ dies, you may exile it. if you do, return any number of target spirit cards from your graveyard to your hand.",
  },
  {
    name: "Deconstruction Hammer",
    oracle: "equipped creature gets +1/+1 and has \"{3}, {t}, sacrifice ~: destroy target artifact or enchantment.\"\nequip {1}",
  },
  {
    name: "Stimulus Package",
    oracle: "when this enchantment enters, create two treasure tokens. \nsacrifice a treasure: create a 1/1 green and white citizen creature token.",
  },
  {
    name: "Sunset Strikemaster",
    oracle: "{t}: add {r}.\n{2}{r}, {t}, sacrifice this creature: it deals 6 damage to target creature with flying.",
  },
  {
    name: "The Surgical Bay",
    oracle: "this land enters tapped.\n{t}: add {u}.\n{1}{u}, {t}, sacrifice this land: draw a card.",
  },
  {
    name: "Shaper Guildmage",
    oracle: "{w}, {t}: target creature gains first strike until end of turn.\n{b}, {t}: target creature gets +1/+0 until end of turn.",
  },
  {
    name: "Conclave Sledge-Captain",
    oracle: "backup 1, backup 1, backup 1 \ntrample\nwhenever this creature deals combat damage to a player, put that many +1/+1 counters on it.",
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
    name: "Web-Shooters",
    oracle: "equipped creature gets +1/+1 and has reach and \"whenever this creature attacks, tap target creature an opponent controls.\"\nequip {2}",
  },
  {
    name: "Seafloor Debris",
    oracle: "this land enters tapped.\n{t}: add {u}.\n{t}, sacrifice this land: add one mana of any color.",
  },
  {
    name: "Vineweft",
    oracle: "enchant creature\nenchanted creature gets +1/+1.\n{4}{g}: return this card from your graveyard to your hand.",
  },
  {
    name: "Ana Disciple",
    oracle: "{u}, {t}: target creature gains flying until end of turn.\n{b}, {t}: target creature gets -2/-0 until end of turn.",
  },
  {
    name: "Kolaghan, the Storm's Fury",
    oracle: "flying\nwhenever a dragon you control attacks, creatures you control get +1/+0 until end of turn.\ndash {3}{b}{r}",
  },
  {
    name: "Water Servant",
    oracle: "{u}: this creature gets +1/-1 until end of turn.\n{u}: this creature gets -1/+1 until end of turn.",
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
    name: "Yahenni, Undying Partisan",
    oracle: "haste\nwhenever a creature an opponent controls dies, put a +1/+1 counter on ~.\nsacrifice another creature: ~ gains indestructible until end of turn.",
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
    name: "Zephyr Boots",
    oracle: "equipped creature has flying.\nwhenever equipped creature deals combat damage to a player, draw a card, then discard a card.\nequip {2}",
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
    name: "Viridian Lorebearers",
    oracle: "{3}{g}, {t}: target creature gets +x/+x until end of turn, where x is the number of artifacts your opponents control.",
  },
  {
    name: "Gilded Scuttler",
    oracle: "this creature can't be blocked.\nwhen this creature enters, tap target creature an opponent controls and put a stun counter on it.",
  },
  {
    name: "Furystoke Giant",
    oracle: "when this creature enters, other creatures you control gain \"{t}: this creature deals 2 damage to any target\" until end of turn.\npersist",
  },
  {
    name: "Lunarch Mantle",
    oracle: "enchant creature\nenchanted creature gets +2/+2 and has \"{1}, sacrifice a permanent: this creature gains flying until end of turn.\"",
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
    name: "Foul Presence",
    oracle: "enchant creature\nenchanted creature gets -1/-1 and has \"{t}: target creature gets -1/-1 until end of turn.\"",
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
    name: "Thunderscape Apprentice",
    oracle: "{b}, {t}: target player loses 1 life.\n{g}, {t}: target creature gets +1/+1 until end of turn.",
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
    name: "Diamond Pick-Axe",
    oracle: "indestructible \nequipped creature gets +1/+1 and has \"whenever this creature attacks, create a treasure token.\" \nequip {2}",
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
    name: "Shriekhorn",
    oracle: "this artifact enters with three charge counters on it.\n{t}, remove a charge counter from this artifact: target player mills two cards.",
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
    name: "Prying Blade",
    oracle: "equipped creature gets +1/+0.\nwhenever equipped creature deals combat damage to a player, create a treasure token. \nequip {2}",
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
    name: "River Sneak",
    oracle: "this creature can't be blocked.\nwhenever another merfolk you control enters, this creature gets +1/+1 until end of turn.",
  },
];
