{
  description = "Tool zum automatischen generieren von Protokollen und Website Posts";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      rust-overlay,
      flake-utils,
      ...
    }:
    let
      systems = [
        "aarch64-linux"
        "x86_64-linux"
      ];
    in
    flake-utils.lib.eachSystem systems (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        inherit (pkgs) lib;

        craneLib = crane.mkLib pkgs;

        markdownFilter = path: _type: builtins.match ".*md$" path != null;
        markdownOrCargo = path: type: (markdownFilter path type) || (craneLib.filterCargoSources path type);

        src = lib.cleanSourceWith {
          src = craneLib.path ./.;
          filter = markdownOrCargo;
        };

        crossBuildFor =
          crossSystem:
          let
            crossPkgs = import nixpkgs {
              inherit crossSystem;
              localSystem = system;
              overlays = [ (import rust-overlay) ];
            };

            crossCraneLib = (crane.mkLib crossPkgs).overrideToolchain (p: p.rust-bin.stable.latest.default);

            systemTripleMap = {
              "x86_64-linux" = "X86_64_UNKNOWN_LINUX_GNU";
              "aarch64-linux" = "AARCH64_UNKNOWN_LINUX_GNU";
            };

            crossExpr =
              { stdenv }:
              crossCraneLib.buildPackage {
                inherit src;
                strictDeps = true;

                nativeBuildInputs = [ stdenv.cc ];

                "CARGO_TARGET_${systemTripleMap.${crossSystem}}_LINKER" = "${stdenv.cc.targetPrefix}cc";
                CARGO_BUILD_TARGET = (
                  lib.toLower (lib.replaceStrings [ "_" ] [ "-" ] systemTripleMap.${crossSystem})
                );
                HOST_CC = "${stdenv.cc.nativePrefix}cc";
                TARGET_CC = "${stdenv.cc.targetPrefix}cc";
              };
          in
          crossPkgs.callPackage crossExpr { };

        localCommonArgs = {
          inherit src;
          strictDeps = true;

          buildInputs = [ ];
        };

        localCargoArtifacts = craneLib.buildDepsOnly localCommonArgs;

        local-prototool-crate = craneLib.buildPackage (
          localCommonArgs
          // {
            cargoArtifacts = localCargoArtifacts;

            meta.mainProgram = "prototool";
          }
        );
      in
      {
        checks = {
          inherit local-prototool-crate;

          my-crate-test = craneLib.cargoTest (
            localCommonArgs
            // {
              cargoArtifacts = localCargoArtifacts;
            }
          );
        };

        packages =
          {
            default = local-prototool-crate;
          }
          // (lib.listToAttrs (
            map (crossSystem: lib.nameValuePair "cross-${crossSystem}" (crossBuildFor crossSystem)) systems
          ));

        devShells = {
          default = craneLib.devShell {
            checks = self.checks.${system};
            nativeBuildInputs = with pkgs; [
              cargo
              rustc
              rustfmt
              cargo-semver-checks
            ];
          };

          attic = pkgs.mkShell {
            nativeBuildInputs = [
              pkgs.attic-client
            ];
          };
        };
      }
    );
}
