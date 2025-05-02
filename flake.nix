{
  inputs = {
    nixpkgs = {
      type = "github";
      owner = "nixos";
      repo = "nixpkgs";
      ref = "nixos-unstable";
    };

    flake-utils = {
      type = "github";
      owner = "numtide";
      repo = "flake-utils";
    };
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
        };

        package = {
          libseccomp,
          rustPlatform,
        }:
          rustPlatform.buildRustPackage {
            pname = "bwrap-bpf-filter";
            version = "0.1.0";

            src = ./.;

            cargoHash = "sha256-cqi5T/F5DC9nKL/LhM/E3lZf+GiARJ1spc+j23Mtb1Q=";

            buildInputs = [libseccomp];
          };
      in {
        packages.default = pkgs.callPackage package {};

        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.rustc
            pkgs.cargo
            pkgs.clippy
            pkgs.rustfmt
            pkgs.libseccomp
          ];
        };

        formatter = pkgs.alejandra;
      }
    );
}
