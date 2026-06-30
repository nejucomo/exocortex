{
  description = "Exocortex — a micro-note taking app for quick jots";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.stable.latest.default;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        src = craneLib.cleanCargoSource ./.;

        # Native build inputs (build-time tools)
        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        # Runtime/link-time libraries
        buildInputs = with pkgs;
          lib.optionals stdenv.isLinux [
            # OpenGL / GPU
            libGL
            # Wayland
            libxkbcommon
            wayland
            # X11
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            # global-hotkey on Linux requires X11
            xorg.libXtst
          ]
          ++ lib.optionals stdenv.isDarwin [
            darwin.apple_sdk.frameworks.AppKit
            darwin.apple_sdk.frameworks.Carbon
            darwin.apple_sdk.frameworks.CoreFoundation
            darwin.apple_sdk.frameworks.CoreServices
          ];

        # LD_LIBRARY_PATH string used both in the dev shell and the package wrapper.
        runtimeLibraryPath = pkgs.lib.optionalString pkgs.stdenv.isLinux
          (pkgs.lib.makeLibraryPath buildInputs);

        commonArgs = {
          inherit src nativeBuildInputs buildInputs;
          strictDeps = true;
        };

        # Pre-build all cargo dependencies so they can be cached.
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        exocortex = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;

          # Make runtime libraries findable on Linux (needed for GL/Wayland at
          # run-time; nix wraps the binary automatically via LD_LIBRARY_PATH).
          postInstall = pkgs.lib.optionalString pkgs.stdenv.isLinux ''
            wrapProgram $out/bin/exocortex \
              --prefix LD_LIBRARY_PATH : ${runtimeLibraryPath}
          '';

          nativeBuildInputs = nativeBuildInputs
            ++ pkgs.lib.optionals pkgs.stdenv.isLinux [ pkgs.makeWrapper ];
        });
      in
      {
        packages.default = exocortex;

        # `nix develop --command cargo build`
        devShells.default = craneLib.devShell {
          inherit buildInputs;
          nativeBuildInputs = nativeBuildInputs
            ++ pkgs.lib.optionals pkgs.stdenv.isLinux [ pkgs.makeWrapper ];
          packages = [
            pkgs.rust-analyzer
            pkgs.cargo-watch
            pkgs.fd
            pkgs.jq
            pkgs.ripgrep
            pkgs.sd
            pkgs.taplo
          ];
          # Make GPU/Wayland libraries available for interactive development.
          LD_LIBRARY_PATH = runtimeLibraryPath;
        };

        checks.default = exocortex;
      });
}
