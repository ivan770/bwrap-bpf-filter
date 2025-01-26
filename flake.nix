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
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "bwrap-bpf-filter";
          version = "0.1.0";

          src = ./.;

          cargoHash = "sha256-GlbNt3EXr+hmOCX4sy/zkx5u6HBtsxZ2xz2HlQtTkSk=";

          buildInputs = [pkgs.libseccomp];
        };

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
