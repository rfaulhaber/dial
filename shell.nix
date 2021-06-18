{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell {
  buildInputs = [ cargo rustc rustfmt clippy rust-analyzer rls rustup ];
}
