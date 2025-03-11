let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-24.11";
  pkgs = import nixpkgs {};
in
  pkgs.mkShell {
    packages = with pkgs; [
      # To compile to WASM
      cargo
      lld
    ];
  }
