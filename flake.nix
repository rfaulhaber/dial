{
  description = "dial";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        projectName = "dial";
      in rec {
        packages.${projectName} = pkgs.rustPlatform.buildRustPackage {
          pname = projectName;
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };

        packages.default = self.packages.${system}.${projectName};

        apps.${projectName} =
          flake-utils.lib.mkApp { drv = packages.${projectName}; };

        apps.default = self.apps.${system}.${projectName};

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            rustfmt
            clippy
            rust-analyzer
            rustup
          ];
        };
      });
}
