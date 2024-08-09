{ pkgs ? import <nixpkgs> {} }:

let
  rustOverlay = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
  pkgs = import <nixpkgs> {
    overlays = [
      (import rustOverlay)
    ];
  };
  rustVersion = "1.80.0";

  # List of extra tools
  toolList = with pkgs; [
    rust-analyzer
    cargo-watch
    cargo-outdated
  ];

  # Function to get the name of a derivation
  getName = drv: drv.pname or drv.name or "unknown";

  # Generate the tool list string
  toolListString = builtins.concatStringsSep "\n  - " (map getName toolList);

in
pkgs.mkShell {
  buildInputs = with pkgs; [
    (rust-bin.stable.${rustVersion}.default.override {
      extensions = [ "rust-src" ];
    })
  ] ++ toolList;

shellHook = ''
    # Welcome message
    printf "\n\033[1;34m=============================================\033[0m"
    printf "\n\033[1;32m🦀 Rust Development Environment Activated 🦀\033[0m"
    printf "\n\033[1;34m=============================================\033[0m"
    printf "\n\033[1;33m• Rust Version: ${rustVersion}\033[0m"
    printf "\n\033[1;33m• Available Tools:\033[0m"
    printf "\n  - ${toolListString}"
    source scripts.sh
    printf "\n\033[1;34m=============================================\033[0m"

    printf "\n\033[1;33m• Checking for any outdated packages...\033[0m\n"
    cargo outdated
  '';
}
