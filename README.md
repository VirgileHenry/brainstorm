# Brainstorm

Brainstorm is a project aiming at parsing Magic: The Gathering (MTG) cards in a usefull manner.

The final goal is to have a structured representation for all cards information, and especially for the oracle text.

### Oracle text parsing

The main challenge for Brainstorm is to parse natural language oracle texts into ability trees.
Such trees can than be used to extract what the cards effects are in a easy way for a computer.

A quick demo for the oracle text parser [can be found here](https://virgilehenry.github.io/brainstorm).

### Status

We keep track of how much of the cards we can parse here.
The cards are pulled from [scryfall](https://scryfall.com/).

There are mutiple objectives on the card groups we aim to parse:
- The latest extension set, used internally to see  if new extensions break the parser.
- The Foundation set (FDN), as it is a good subset of basic abilities found in MTG.
- All Standard-legal cards, as a first milestone of the working project.
- All Commander-legal cards, as it is a widely played format.
- All non-"funny" cards, so everything except uncards.

<!-- BEGIN_TEST_RECAP -->
| Category | Cards total | Lexed (oracle text) | Parsed (oracle text) | Parsed (full card) |
|-----|-----|-----|-----|-----|
|Foundation set (FDN)|436|300 (68%)|39 (8%)|0 (0%)|
|Last set (ECL)|267|116 (43%)|14 (5%)|0 (0%)|
|Standard-legal cards|4168|1820 (43%)|110 (2%)|0 (0%)|
|Commander-legal cards|30654|14800 (48%)|1320 (4%)|0 (0%)|
|All (except uncards)|35464|16019 (45%)|1545 (4%)|0 (0%)|
<!-- END_TEST_RECAP -->

Uncards are not planned to be parsed, as their wording are unique enough that it would almost always require dedicating chunks of the tree representation for them.

### Structure:

At the time of me writing this, the project contains the following crates:
- krark: Test framework, allowing to run test on the entire mtg card database (or any subset of it) and provides result visualization.
- mtg-data: Regroups easy to parse MTG common data, such as card types, sets, etc in the form of Rust Enums.
- mtg-cardbase: The entire database of MTG cards, as raw, unparsed data fetched from the scryfall API. Currently holds 35k+ cards.
- boseiju: The MTG parser, able to convert an oracle text into an ability tree built only from MTG data Enums.
