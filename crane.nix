{inputs, ...}: {
  perSystem = {
    pkgs,
    craneLib,
    self',
    ...
  }: let
    inherit (pkgs) lib;
    rustToolchainFor = p:
      p.rust-bin.selectLatestNightlyWith (
        toolchain:
          toolchain.default.override {
            extensions = ["rust-src" "rustfmt"];
          }
      );
    rustToolchain = rustToolchainFor pkgs;
    craneLibNightly = craneLib.overrideToolchain rustToolchainFor;
    src = lib.cleanSourceWith {
      src = ./.;
      filter = path: type:
        (craneLib.filterCargoSources path type)
        || (lib.hasSuffix "wrapper.h" path);
      name = "brightness-cli-src";
    };
    commonArgs = {
      inherit src;
      strictDeps = true;
      buildInputs = [] ++ lib.optionals pkgs.stdenv.isDarwin [pkgs.libiconv];
    };
    cargoArtifacts = craneLibNightly.buildDepsOnly commonArgs;
    individualCrateArgs =
      commonArgs
      // {
        inherit cargoArtifacts;
        inherit (craneLibNightly.crateNameFromCargoToml {inherit src;}) version;
        doCheck = false;
      };
    brightness = craneLibNightly.buildPackage (
      individualCrateArgs
      // {
        pname = "brightness";
        src = src;
      }
    );
  in {
    checks = {
      brightness-audit = craneLib.cargoAudit {
        inherit src;
        advisory-db = inputs.advisory-db;
      };
      brightness-deny = craneLib.cargoDeny {
        inherit src;
      };
    };
    packages.brightness = brightness;
    devShells.default = craneLibNightly.devShell {
      checks = self'.checks;
      packages = [rustToolchain pkgs.cargo-watch];
      RUST_SRC_PATH = "${rustToolchain.passthru.availableComponents.rust-src}/lib/rustlib/src/rust/library";
    };
  };
}
