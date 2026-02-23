{
  description = "Brightness CLI";
  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = inputs @ {
    self,
    flake-parts,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.treefmt-nix.flakeModule
        ./crane.nix
        ./treefmt.nix
      ];
      systems = ["aarch64-darwin" "x86_64-darwin"];
      perSystem = {
        system,
        self',
        ...
      }: let
        pkgs = import self.inputs.nixpkgs {
          inherit system;
          overlays = [self.inputs.rust-overlay.overlays.default];
          config = {
            allowUnfree = true;
            allowBroken = true;
          };
        };
        craneLib = self.inputs.crane.mkLib pkgs;
      in {
        _module.args = {
          inherit pkgs craneLib;
        };
        packages.default = self'.packages.brightness;
      };
      flake = {};
    };
}
