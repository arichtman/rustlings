{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
    python311
  ];
  shellHook = ''
  rustup override set nightly
  export PATH="$PATH:$HOME/.cargo/bin"
  #cargo install --path .
  '';
  RUST_BACKTRACE = 1;
}
