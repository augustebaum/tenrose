let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-24.11";
  rust-overlay = fetchTarball "https://github.com/oxalica/rust-overlay/archive/7af16cbd1464fddde8ad0c4ed7baaa2292445ba4.tar.gz";

  pkgs = import nixpkgs {overlays = [(import rust-overlay)];};
in
  pkgs.mkShell {
    packages = with pkgs; [
      # To compile to WASM
      cargo
      (rust-bin.stable.latest.default.override {
        extensions = [];
        targets = ["wasm32-wasip1"];
      })
      lld

      # To download penrose/core
      nodejs
    ];
  }
