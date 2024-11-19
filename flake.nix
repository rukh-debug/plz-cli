{
  description = "plz-cli Nix Flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    let
      systems = lib.platforms.all;
      lib = nixpkgs.lib;
    in
    {
      packages = lib.genAttrs systems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          lib = pkgs.lib;
        in
        {
          default = pkgs.rustPlatform.buildRustPackage rec {
            pname = "plz-cli";
            version = "0.1.0";

            src = self;

            # cargoHash = null;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            meta = with lib; {
              description = "plz-cli tool";
              homepage = "https://github.com/rukh-debug/plz-cli";
              license = licenses.mit;
              mainProgram = "plz"; # Important for CLI tools
            };

          };
        }
      );

      devShells = lib.genAttrs systems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
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
