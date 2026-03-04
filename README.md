# Brainstorm

Brainstorm is a project aiming at parsing Magic: The Gathering (MTG) cards in a usefull manner.

The final goal is to have a structured representation for all cards information, and especially for the oracle text.

### Oracle text parsing

The main challenge for Brainstorm is to parse natural language oracle texts into ability trees.
Such trees can than be used to extract what the cards effects are in a easy way for a computer.

A quick demo for the oracle text parser [can be found here](https://virgilehenry.github.io/brainstorm).

### Documentation

The documentation for the main crates are available here:
- [boseiju](https://virgilehenry.github.io/brainstorm/doc/boseiju)
- [mtg-cardbase](https://virgilehenry.github.io/brainstorm/doc/mtg-cardbase)
- [mtg-data](https://virgilehenry.github.io/brainstorm/doc/mtg-data)
- [idris](https://virgilehenry.github.io/brainstorm/doc/idris)

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
|Foundation set (FDN)|436|240 (55%)|47 (10%)|0 (0%)|
|Last set (ECL)|267|105 (39%)|16 (5%)|0 (0%)|
|Standard-legal cards|4168|1553 (37%)|144 (3%)|0 (0%)|
|Commander-legal cards|30654|12467 (40%)|1626 (5%)|0 (0%)|
|All (except uncards)|35464|13607 (38%)|1887 (5%)|0 (0%)|
<!-- END_TEST_RECAP -->

Uncards are not planned to be parsed, as their wording are unique enough that it would almost always require dedicating chunks of the tree representation for them.
