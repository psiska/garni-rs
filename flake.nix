{
  description = "Nix Flake for garni-rs - local weather server to collect metrics from weather station and expose them to prometheus.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-parts.inputs.nixpkgs-lib.follows = "nixpkgs";
    systems.url = "github:nix-systems/default";
    rust-flake = {
      url = "github:juspay/rust-flake";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ { flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } (top@{ config, withSystem, moduleWithSystem, ... }: {
      #      systems = [ "x86_64-linux" ]; # "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      systems = import inputs.systems;

      imports = [
        inputs.rust-flake.flakeModules.default
        inputs.rust-flake.flakeModules.nixpkgs
      ];

      flake = { };

      perSystem = { pkgs, self', config, ... }: {
        formatter = pkgs.nixpkgs-fmt;
        devShells.default = pkgs.mkShell {
          inputsFrom = [
            # self'.devShells.haskell
            self'.devShells.rust
          ];
          # Add your devShell tools here
          packages = with pkgs; [
            self'.packages.garni-rs
            jq
            nixpkgs-fmt
          ];
        };
        #        devShells.default = self'.devShells.rust;
        packages.default = self'.packages.garni-rs;
      };

      flake.nixosModules.default = { pkgs, ... }: {
        imports = [ ./nix/nixos-module.nix ];
        services.garni-rs.package = withSystem pkgs.stdenv.hostPlatform.system (
          { config, ... }: config.packages.default
        );
      };
    });
}
