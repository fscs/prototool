{
  description = "Tool zum automatischen generieren von Protokollen und Website Posts";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};

      inherit (pkgs) lib;

      craneLib = crane.mkLib pkgs;

      markdownFilter = path: _type: builtins.match ".*md$" path != null;
      markdownOrCargo = path: type:
        (markdownFilter path type) || (craneLib.filterCargoSources path type);

      src = lib.cleanSourceWith {
        src = craneLib.path ./.;
        filter = markdownOrCargo;
      };

      commonArgs = {
        inherit src;
        strictDeps = true;

        buildInputs =
          []
          ++ lib.optionals pkgs.stdenv.isDarwin [
            pkgs.libiconv
          ];
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      prototool = craneLib.buildPackage (commonArgs
        // {
          inherit cargoArtifacts;
          
          meta.mainProgram = "prototool";
        });
    in {
      checks = {
        inherit prototool;

        prototool-test = craneLib.cargoTest (commonArgs
          // {
            inherit cargoArtifacts;
          });
      };

      packages.default = prototool;
      hydraJobs.prototool = prototool;

      devShells.default = craneLib.devShell {
        checks = self.checks.${system};
        nativeBuildInputs = with pkgs; [
          cargo 
          rustc
          rustfmt
          cargo-semver-checks
        ];
      };
    });
}
