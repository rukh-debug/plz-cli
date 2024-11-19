{
  description = "plz-cli Nix Flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    let
      systems = flake-utils.lib.systems.defaults;
    in
    {
      packages = flake-utils.lib.eachDefaultSystem (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          lib = pkgs.lib;
        in {
          default = pkgs.rustPlatform.buildRustPackage rec {
            pname = "plz-cli";
            version = "0.1.0";

            src = pkgs.fetchFromGitHub {
              owner = "rukh-debug";
              repo = "plz-cli";
              rev = "11cda7cd024004107c689b5f44ec1d6471c0fcd5";
              sha256 = "sha256-fW5Kp74PByTQ3uhxstkXy/tNNtTIgjh26px30QHgCgo=";
            };

            cargoHash = "sha256-YOsZAXgaklfSNj3wbL4IgUTs6hLOIjwG5Ij36bDhtFQ=";

            meta = with lib; {
              description = "plz-cli tool";
              homepage = "https://github.com/rukh-debug/plz-cli";
              license = licenses.mit;
              mainProgram = "plz";  # Important for CLI tools
            };
          };
        }
      );

      devShells = flake-utils.lib.eachDefaultSystem (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in {
          default = pkgs.mkShell {
            buildInputs = [ self.packages.${system}.default ];
          };
        }
      );

      nixosModules = {
        default = { pkgs, ... }: {
          environment.systemPackages = [ self.packages.x86_64-linux.default ];
        };
      };
    };
}