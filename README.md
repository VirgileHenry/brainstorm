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
- [mtg_cardbase](https://virgilehenry.github.io/brainstorm/doc/mtg_cardbase)
- [mtg_data](https://virgilehenry.github.io/brainstorm/doc/mtg_data)
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

| Category              | Cards total | JSON Parsed | Lexed        | Parsed     |
|-----------------------|-------------|-------------|--------------|------------|
| Foundation set (FDN)  | 428         | 428 (100%)  | 428 (100%)   | 82 (19%)   |
| Last set (HOB)        | 7           | 7 (100%)    | 7 (100%)     | 5 (71%)    |
| Standard-legal cards  | 4702        | 4406 (93%)  | 4406 (100%)  | 410 (9%)   |
| Commander-legal cards | 30402       | 29227 (96%) | 29227 (100%) | 3801 (13%) |
| All paper cards       | 30476       | 29300 (96%) | 29300 (100%) | 3803 (12%) |
<!-- END_TEST_RECAP -->

Uncards are not planned to be parsed, as their wording are unique enough that it would almost always require dedicating chunks of the tree representation for them.

### Tooling / Nex set release

#### Epithet map

The names without epithets are generated from the card.
The script is far from perfect, and is a best effort that needs to be ajusted if it breaks parsing.

To run the generation script (and overwrite existing source):
```
cargo run --release --bin generate_epithet_map > epithets.txt
```

Then copy / paste the epithets to boseiju/src/lexer/epithets.rs
