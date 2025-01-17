{
  description = "Hexagonal boards on the terminal";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      rec {
        packages.default = pkgs.callPackage ./default.nix { };

        formatter = pkgs.nixfmt-rfc-style;

        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.cargo
            pkgs.cargo-nextest
            pkgs.bacon
            pkgs.clippy
            pkgs.rustfmt
            packages.default.buildInputs
          ];
        };
      }
    )
    // {
      nixosModules.default = import ./module.nix;
    };
}
