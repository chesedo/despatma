{
  description = "Build the despatma library";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };


    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, fenix, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        inherit (pkgs) lib;

        craneLib = crane.mkLib pkgs;
        src = let
          # Allow markdown files used for documentation via `include_str!`
          markdownFilter = path: _type: lib.hasSuffix ".md" path;

          # Allow the trybuild error files for tests
          expectedTryerrorFilter = path: _type: lib.hasSuffix ".stderr" path;

          filter = path: type:
            (markdownFilter path type) ||
            (expectedTryerrorFilter path type) ||
            (craneLib.filterCargoSources path type);
        in
          lib.cleanSourceWith {
            inherit filter;
            src = ./.;
          };

        # Common arguments can be set here to avoid repeating them later
        # Note: changes here will rebuild all dependency crates
        commonArgs = {
          inherit src;
          strictDeps = true;

          buildInputs = [
            # Add additional build inputs here
          ];
        };

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        despatma = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;

          description = "Build the library and run all tests";

          nativeBuildInputs = with pkgs; [
            # Needed for the tests
            rustfmt
            cargo-expand
          ];
        });
      in
      {
        checks = {
          # Build the crate as part of `nix flake check` for convenience
          inherit despatma;

          # Run clippy (and deny all warnings) on the crate source,
          # again, reusing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            description = "Run clippy";
          });

          doc = craneLib.cargoDoc (commonArgs // {
            inherit cargoArtifacts;
            description = "Build documentation";
          });

          # Check formatting
          fmt = craneLib.cargoFmt {
            inherit src;
            description = "Check code formatting";
          };

          toml-fmt = craneLib.taploFmt {
            src = pkgs.lib.sources.sourceFilesBySuffices src [ ".toml" ];
            description = "Check toml formatting";
          };
        };

        packages = {
          default = despatma;
        };

        apps.default = flake-utils.lib.mkApp {
          drv = despatma;
        };

        devShells.default = let
          # Create check scripts for each check in self.checks.${system}
          checkScripts = lib.mapAttrs (name: _:
            pkgs.writeShellScriptBin "check-${name}" ''
              nix build .#checks.${system}.${name} "$@"
            ''
          ) self.checks.${system};
        in craneLib.devShell {
          # Inherit inputs from checks.
          checks = self.checks.${system};

          # Shell hooks to create executable scripts in a local bin directory
          shellHook = ''
            col_width=15;
            cargo_version=$(cargo --version 2>/dev/null)

            echo -e "\033[1;36m=== 🦀 Welcome to the Despatma development environment ===\033[0m"
            echo -e "\033[1;33m• $cargo_version\033[0m"
            echo ""
            echo -e "\033[1;33mAvailable commands:\033[0m"
            ${builtins.concatStringsSep "\n" (
              lib.mapAttrsToList (name: def: ''
                printf "  \033[1;37m%-''${col_width}s\033[0m - %s\n" "check-${name}" "${def.description}"
              '') self.checks.${system}
            )}
            printf "  \033[1;37m%-''${col_width}s\033[0m - %s\n" "nix flake check" "Run all checks"
            echo ""
          '';

          # Extra inputs can be added here; cargo and rustc are provided by default.
          packages = [
            fenix.packages.${system}.rust-analyzer
          ] ++ lib.attrValues checkScripts;
        };
      });
}
