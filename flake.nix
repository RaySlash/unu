{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs@{ self, nixpkgs, systems, rust-overlay, ... }:
    let
      # Using nix-systems to identify architecture
      eachSystem = nixpkgs.lib.genAttrs (import systems);
      pkgsFor = eachSystem (system:
      import nixpkgs {
        localSystem = system;
        overlays = [
          rust-overlay.overlays.default
        ];
      });

      mkUnu = ({ rustPlatform, lib, pkgs, ... }:
      rustPlatform.buildRustPackage {
        pname = "unu";
        name = "unu";       # attribute name for packages
        src = ./.;

        cargoLock.lockFile = ./Cargo.lock;

        meta = {
          description = "Wrapper made to use pacman, nix aur in one system";
          license = lib.licenses.gpl3Plus;
          platforms = lib.platforms.unix;
          mainProgram = "unu";
        };
      });
    in
    {
      overlays.default = final: prev: {
        unu = prev.callPackage mkUnu { };
      };

      # `nix build` works
      packages = eachSystem (system: {
        default = pkgsFor.${system}.callPackage mkUnu { };
      });

      # `nix develop` works
      devShells = eachSystem (system:
        let
          rust-toolchain = (pkgsFor.${system}.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
            extensions = [ "rust-src" "rust-analyzer" ];
          };
        in {
          default = pkgsFor.${system}.mkShell {
            packages = [
	    	rust-toolchain
	    ];
          };
        }
      );
    };
}
