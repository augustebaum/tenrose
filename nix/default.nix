let
  sources = import ./sources.nix {};
  pkgs = import sources.nixpkgs {};

  fenix = import sources.fenix {};
  toolchain = fenix.targets.wasm32-wasi.stable.toolchain;
in {
  javy = pkgs.callPackage ./javy.nix {
    rustPlatform = pkgs.makeRustPlatform {
      cargo = toolchain;
      rustc = toolchain;
    };
  };

}
