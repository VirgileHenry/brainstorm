# This python script will generate the rust source code from the data directory.

import re
import sys
from pathlib import Path
from unidecode import unidecode


DATA_DIR = Path("data")
SRC_DIR = Path("src")


def sanitize(name: str) -> str:
    """Turn an arbitrary line into a Rust PascalCase identifier.

    Handles accents (é, à, ...), apostrophes, dots, spaces, hyphens, etc.
    """
    # Transliterate accents/unicode to ASCII (é -> e, à -> a, ...)
    ascii_name = unidecode(name)
    # avoid 's to be a capital S
    ascii_name = ascii_name.replace("'", "")
    # Split on anything that isn't a letter or digit
    parts = re.split(r"[^A-Za-z0-9]+", ascii_name)
    pascal = "".join(p[:1].upper() + p[1:].lower() for p in parts if p)
    if not pascal:
        raise ValueError(f"Cannot sanitize line into identifier: {name!r}")
    # Prepend underscore if it starts with a digit (Rust identifier rule)
    if pascal[0].isdigit():
        pascal = "_" + pascal
    return pascal


def snake_to_pascal(snake: str) -> str:
    return "".join(part.capitalize() for part in snake.split("_"))


def generate_enum(enum_name: str, source_file: Path, dest_file: Path) -> None:
    lines = [
        line.strip()
        for line in source_file.read_text(encoding="utf-8").splitlines()
        if line.strip()
    ]
    variants = [(line.lower(), sanitize(line)) for line in lines]

    # Detect duplicate variant names (would cause Rust compile error)
    seen: dict[str, str] = {}
    for original, variant in variants:
        if variant in seen and seen[variant] != original:
            raise ValueError(
                f"{source_file}: duplicate variant {variant!r} "
                f"from {seen[variant]!r} and {original!r}"
            )
        seen[variant] = original

    out: list[str] = []
    w = out.append

    # Enum declaration
    w("#[derive(idris_derive::Idris)]")
    w("#[idris(repr = usize)]")
    w("#[derive(serde::Serialize, serde::Deserialize)]")
    w("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]")
    w('#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]')
    w(f"pub enum {enum_name} {{")
    for _, variant in variants:
        w(f"    {variant},")
    w("}")
    w("")

    # FromStr
    w(f"impl std::str::FromStr for {enum_name} {{")
    w("    type Err = crate::ParsingError;")
    w("    fn from_str(s: &str) -> Result<Self, Self::Err> {")
    w("        match s {")
    for original, variant in variants:
        w(f'            "{original}" => Ok(Self::{variant}),')
    w("            _ => Err(crate::ParsingError {")
    w(f'                item: "{enum_name}",')
    w('                message: "provided source does not match",')
    w("            }),")
    w("        }")
    w("    }")
    w("}")
    w("")

    # as_str
    w(f"impl {enum_name} {{")
    w("    pub fn as_str(&self) -> &'static str {")
    w("        match self {")
    for original, variant in variants:
        w(f'            Self::{variant} => "{original}",')
    w("        }")
    w("    }")
    w("}")
    w("")

    # Display
    w(f"impl std::fmt::Display for {enum_name} {{")
    w("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {")
    w('        write!(f, "{}", self.as_str())')
    w("    }")
    w("}")
    w("")

    # all()
    w(f"impl {enum_name} {{")
    w("    pub fn all() -> impl Iterator<Item = Self> {")
    w("        [")
    for _, variant in variants:
        w(f"            Self::{variant},")
    w("        ].into_iter()")
    w("    }")
    w("}")

    dest_file.write_text("\n".join(out) + "\n", encoding="utf-8")
    print(f"  {source_file}  ->  {dest_file}  ({len(variants)} variants)")


def main() -> int:
    if not DATA_DIR.is_dir():
        sys.exit(f"Data directory not found: {DATA_DIR.resolve()}")
    SRC_DIR.mkdir(exist_ok=True)

    sources = sorted(DATA_DIR.glob("*.txt"))
    if not sources:
        sys.exit(f"No .txt files found in {DATA_DIR.resolve()}")

    print(f"Generating {len(sources)} enum(s):")
    for source_file in sources:
        stem = source_file.stem  # e.g. "creature_type"
        enum_name = snake_to_pascal(stem)  # "CreatureType"
        dest_file = SRC_DIR / f"{stem}.rs"
        generate_enum(enum_name, source_file, dest_file)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
