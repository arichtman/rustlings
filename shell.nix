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
  export PATH="$PATH:$HOME/.cargo/bin"
  cargo install --path .
  '';
  RUST_BACKTRACE = 1;
}
