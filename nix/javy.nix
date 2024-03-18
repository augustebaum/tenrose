{
  fetchFromGitHub,
  rustPlatform,
  pkg-config,
  openssl,
}: let
  version = "unstable-2024-03-05";
in
  rustPlatform.buildRustPackage {
    pname = "javy";
    inherit version;

    src = fetchFromGitHub {
      owner = "bytecodealliance";
      repo = "javy";
      rev = "0a32f865ee17accd38d1b2fe0b8371ee30e4fa5b";
      hash = "sha256-vZXZ+Q/nkDmG6br4B87q7rZZIEWYf4Pdz4Yk7PntV6c=";
    };

    nativeBuildInputs = [pkg-config];

    buildInputs = [openssl];

    buildPhase = ''
      cargo build --package="javy-core" --target="wasm32-wasi" --release
      cargo build --package=javy-cli --release
    '';

    cargoHash = "sha256-2cF2SabPU0iqUWwZXjzsUhSvV9vb61mesOmD8lVPEI0=";
  }
