{
  description = "cuid-rust";

  inputs = {
   nixpkgs.url = "nixpkgs/nixos-unstable";
   # Proivdes legacy compatibility for nix-shell
   flake-compat = { url = "github:edolstra/flake-compat"; flake = false; };
   # Provides some nice helpers for multiple system compatibility
   flake-utils.url = "github:numtide/flake-utils";
   # Provides rust and friends
   rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, flake-compat }:
    # Calls the provided function for each "default system", which
    # is the standard set.
    flake-utils.lib.eachDefaultSystem
      (system:
        # instantiate the package set for the supported system, with our
        # rust overlay
        let pkgs = import nixpkgs {
              inherit system;
              overlays = [ rust-overlay.overlays.default ];
            };
        in
        # "unpack" the pkgs attrset into the parent namespace
        with pkgs;
        {
          devShell = mkShell {
            # Packages required for development.
            buildInputs = [
              bashInteractive
              cargo-audit
              cargo-edit
              coreutils
              gnumake
              jq  # used for benchmark parsing
              # Read our toolchain file to determine which version of rust,
              # components, and targets to install.
              (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
              rust-analyzer
              util-linux  # lspcu utility for getting info about cores
            ];
          };
        });
}
