{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        buildInputs = with pkgs; [ ];

        exocortexPkg = pkgs.rustPlatform.buildRustPackage {
          pname = "exocortex";
          version = "0.1.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [ rustToolchain ];

          inherit buildInputs;

          meta = with pkgs.lib; {
            homepage = "https://github.com/nejucomo/exocortex";
            license = licenses.mit;
            maintainers = [ ];
          };
        };
      in
      {
        packages.default = exocortexPkg;

        devShells.default = pkgs.mkShell {
          inputsFrom = [ exocortexPkg ];

          buildInputs = with pkgs; [
            rustToolchain
            rust-analyzer
            pkg-config
          ];

          shellHook = ''
            export PKG_CONFIG_PATH="${pkgs.lib.makeSearchPath "lib/pkgconfig" buildInputs}"
          '';
        };
      }
    );
}
