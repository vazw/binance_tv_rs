{
  description = "binance_tv_rs - Personal challenge, Rewrote Tradingview_Binance_API Project With Rust.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/master";
    parts.url = "github:hercules-ci/flake-parts";
    crane.url = "github:ipetkov/crane";
    rust.url = "github:oxalica/rust-overlay";
    nix-filter.url = "github:numtide/nix-filter";
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    parts,
    crane,
    rust,
    nix-filter,
    ...
  }:
    parts.lib.mkFlake {inherit inputs;} {
      systems = ["aarch64-linux" "x86_64-linux"];

      perSystem = {
        self',
        lib,
        system,
        ...
      }: let
        pkgs = nixpkgs.legacyPackages.${system}.extend rust.overlays.default;
        rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        craneLib = crane.lib.${system}.overrideToolchain rust-toolchain;
        craneArgs = {
          pname = "binance_tv_rs";
          version = self.rev or "dirty";

          src = nix-filter.lib.filter {
            root = ./.;
            include = [
              ./src
              ./Cargo.toml
              ./Cargo.lock
              ./resources
            ];
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
            autoPatchelfHook
          ];

          buildInputs = with pkgs; [
            fontconfig
            stdenv.cc.cc.lib
          ];
           runtimeDependencies = with pkgs; [
             openssh
          ];
        };

        cargoArtifacts = craneLib.buildDepsOnly craneArgs;
        binance_tv_rs = craneLib.buildPackage (craneArgs // {inherit cargoArtifacts;});
      in {
        apps.binance_tv_rs = {
          type = "app";
          program = lib.getExe self'.packages.default;
        };

        checks.binance_tv_rs = binance_tv_rs;
        packages.default = binance_tv_rs;

        devShells.default = pkgs.mkShell {
          LD_LIBRARY_PATH = lib.makeLibraryPath (__concatMap (d: d.runtimeDependencies) (__attrValues self'.checks));
          buildInputs = with pkgs; [
            fontconfig
            stdenv.cc.cc.lib
            rust-bin.stable.latest.default
            rust-analyzer
            rustfmt
            pkg-config
            openssh
            openssl
          ];
        };
      };
    };
}
