{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.cargo
    pkgs.cargo-flamegraph
    pkgs.rustc
    pkgs.rustfmt
    pkgs.python313
    pkgs.python313Packages.requests
    pkgs.graphviz
    pkgs.lld_20
  ];
  RUST_BACKTRACE=1;
  TMPDIR="/tmp";
}
