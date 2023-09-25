{
  description = "dolphinbot, Dolphin's Discord bot";

  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
  inputs.cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
  inputs.cargo2nix.inputs.nixpkgs.follows = "nixpkgs";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  inputs.rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

  outputs = { self, nixpkgs, flake-utils, cargo2nix, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            cargo2nix.overlays.default
            rust-overlay.overlays.default
          ];
        };

        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustToolchain = pkgs.rust-bin.stable.latest.default;
          packageFun = import ./Cargo.nix;
        };
      in rec {
        packages.discord-bot = (rustPkgs.workspace.dolphinbot {}).bin;
        defaultPackage = packages.discord-bot;
      }
    );
}
