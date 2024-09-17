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

        nativeBuildInputs = [
          pkgs.pkg-config
        ];

        buildInputs =
          []
          ++ lib.optionals pkgs.stdenv.isDarwin [
            pkgs.libiconv
          ];
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      my-crate = craneLib.buildPackage (commonArgs
        // {
          inherit cargoArtifacts;
        });
    in {
      checks = {
        inherit my-crate;

        my-crate-test = craneLib.cargoTest (commonArgs
          // {
            inherit cargoArtifacts;
          });
      };

      packages.default = my-crate;

      apps.default = flake-utils.lib.mkApp {
        drv = my-crate;
      };

      devShells.default = craneLib.devShell {
        checks = self.checks.${system};
      };
    });
}
