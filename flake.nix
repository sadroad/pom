{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      systems,
      rust-overlay,
    }:
    let
      forEachSystem =
        f:
        nixpkgs.lib.genAttrs (import systems) (
          system:
          let
            pkgs = import nixpkgs {
              inherit system;
              overlays = [ (import rust-overlay) ];
            };
            rustToolchain = pkgs.rust-bin.stable.latest.default;
          in
          f pkgs rustToolchain
        );
    in
    {
      packages = forEachSystem (
        pkgs: rustToolchain:
        let
          rustPlatform = pkgs.makeRustPlatform {
            cargo = rustToolchain;
            rustc = rustToolchain;
          };
        in
        rec {
          default = rustPlatform.buildRustPackage {
            pname = "pom";
            version = "0.1.0";

            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            nativeBuildInputs = pkgs.lib.optionals pkgs.stdenv.isLinux [
              pkgs.pkg-config
            ];

            buildInputs = pkgs.lib.optionals pkgs.stdenv.isLinux [
              pkgs.alsa-lib
            ];

            meta = {
              description = "Simple pomodoro timer CLI with desktop notifications";
              license = pkgs.lib.licenses.unlicense;
              mainProgram = "pom";
            };
          };
          pom = default;
        }
      );

      devShells = forEachSystem (
        pkgs: rustToolchain:
        let
          rustToolchainWithExtensions = rustToolchain.override {
            extensions = [
              "rust-src"
              "rust-analyzer"
            ];
          };
        in
        {
          default = pkgs.mkShell {
            inputsFrom = [ self.packages.${pkgs.stdenv.hostPlatform.system}.pom ];
            packages = with pkgs; [
              rustToolchainWithExtensions
            ];
          };
        }
      );
    };
}
