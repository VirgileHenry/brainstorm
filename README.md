# Brainstorm

Brainstorm is a project aiming at creating a vector embedding space for Magic: The Gathering (MTG) cards.

The cards can then be stored in a vector database, and can be used for efficient ability aware searching, as well as providing pertinent deck building candidates.

### Structure:

At the time of me writing this, the project contains the following crates:
- krark: Test framework, allowing to run test on the entire mtg card database (or any subset of it) and provides result visualization.
- mtg-data: Regroups easy to parse MTG common data, such as card types, sets, etc in the form of Rust Enums.
- mtg-cardbase: The entire database of MTG cards, as raw, unparsed data fetched from the scryfall API. Currently holds 35k+ cards.
- odin: The MTG parser, able to convert an oracle text into an ability tree built only from MTG data Enums.
- tolaria: The library of all MTG cards, parsed, and ready for usage. This is the finality of the project.
