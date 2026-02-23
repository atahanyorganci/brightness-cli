# `brightness` CLI

A small CLI to control display brightness on macOS using DisplayServices.

> [!WARNING]
> This tool uses Apple's private `DisplayServices` framework, which is undocumented and not part of the public API. It may stop working or break in future macOS releases.

## Usage

```bash
# Show current brightness (0–100%)
brightness get

# Set brightness (0.0–1.0)
brightness set 0.5

# Increase brightness (default step 0.1)
brightness up
brightness up --step 0.2

# Decrease brightness (default step 0.1)
brightness down
brightness down --step 0.15
```

## Installation

### Using Nix Flakes

Add `brightness` to your `flake.nix` as a flake input.

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    brightness.url = "github:atahanyorganci/brightness-cli";
  };
}
```

Then use `brightness` as a package in your `flake.nix`.

```nix
environment.systemPackages = with pkgs; [
  brightness.packages.${system}.brightness
];
```

## License

[MIT](./LICENSE)
