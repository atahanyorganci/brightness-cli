{
  perSystem = {pkgs, ...}: let
    rustToolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);
    wrappedRustfmt = pkgs.writeShellScriptBin "rustfmt" ''
      export DYLD_LIBRARY_PATH="${rustToolchain}/lib:$DYLD_LIBRARY_PATH"
      exec ${rustToolchain}/bin/rustfmt "$@"
    '';
  in {
    treefmt = {
      projectRootFile = "flake.nix";
      programs = {
        alejandra.enable = true;
        deadnix.enable = true;
        mdsh.enable = true;
        taplo.enable = true;
      };
      settings.formatter = {
        taplo.options = [
          "--config"
          (builtins.toString ./taplo.toml)
        ];
        rustfmt-nightly = {
          command = "${wrappedRustfmt}/bin/rustfmt";
          options = [
            "--edition"
            "2021"
            "--config-path"
            (builtins.toString ./rustfmt.toml)
          ];
          includes = ["*.rs"];
        };
      };
    };
  };
}
