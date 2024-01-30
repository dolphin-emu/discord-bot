{
  description = "dolphinbot, Dolphin's Discord bot";

  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
  inputs.cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
  inputs.cargo2nix.inputs.nixpkgs.follows = "nixpkgs";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  inputs.rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

  outputs = { self, nixpkgs, flake-utils, cargo2nix, rust-overlay }: {
    overlay = nixpkgs.lib.composeManyExtensions [
      cargo2nix.overlays.default
      rust-overlay.overlays.default
      (final: prev: {
        discord-bot = let
          rustPkgs = prev.rustBuilder.makePackageSet {
            rustToolchain = prev.rust-bin.stable.latest.default;
            packageFun = import ./Cargo.nix;
          };
        in
          (rustPkgs.workspace.dolphinbot {}).bin;
      })
    ];
  } // (flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ self.overlay ];
      };
    in with pkgs; {
      packages = { inherit discord-bot; };
      defaultPackage = self.packages.${system}.discord-bot;
    }
  ));
}
