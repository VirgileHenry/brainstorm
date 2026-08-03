{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.cargo
    pkgs.cargo-flamegraph
    pkgs.clippy
    pkgs.cargo-llvm-cov # Fuck me, can't get this to work
    pkgs.rustc
    pkgs.rustfmt
    pkgs.python313
    pkgs.python313Packages.requests
    pkgs.python313Packages.unidecode
    pkgs.graphviz
    pkgs.lld_20
  ];
  RUST_BACKTRACE=1;
  TMPDIR="/tmp";
}
