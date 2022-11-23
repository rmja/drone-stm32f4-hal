{
  description = "STM32F4 peripheral drivers for Drone, an Embedded Operating System";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/nixos-22.05";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, utils, nixpkgs, fenix }:
    utils.lib.eachDefaultSystem (system:
      let
        buildTarget = "thumbv7em-none-eabihf";
        rustFlags = ''--cfg drone_cortexm="cortexm4f_r0p1" --cfg drone_stm32_map="stm32f429"'';
        rustChannel = {
          channel = "nightly";
          date = "2022-11-12";
          sha256 = "NZrKSshDgITZuDSffP89NpZl/pQlblc7arXatkV+O9A=";
        };

        pkgs = nixpkgs.legacyPackages.${system};
        rustToolchain = with fenix.packages.${system}; combine
          ((with toolchainOf rustChannel; [
            rustc
            cargo
            clippy
            rustfmt
            rust-src
          ]) ++ (with targets.${buildTarget}.toolchainOf rustChannel; [
            rust-std
          ]));
        rustAnalyzer = fenix.packages.${system}.rust-analyzer;

        crossEnv = {
          CARGO_BUILD_TARGET = buildTarget;
        };
        nativeEnv = {
          CARGO_BUILD_TARGET = pkgs.stdenv.targetPlatform.config;
        };

        checkAll = pkgs.writeShellScriptBin "check-all" ''
          set -ex
          cargo fmt --all --check
          cargo clippy --workspace --features all -- --deny warnings
          nix develop '.#native' -c cargo test --features host
          # Build Rustdoc documentation and ensure there are no warnings.
          RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --features all
        '';

        mkShell = extraEnv: pkgs.mkShell ({
          nativeBuildInputs = [
            rustToolchain
            rustAnalyzer
            checkAll
          ];
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          CARGO_BUILD_RUSTFLAGS = rustFlags;
        } // extraEnv);
      in
      {
        devShells = rec {
          cross = mkShell (crossEnv // { name = "cross"; });
          native = mkShell (nativeEnv // { name = "native"; });
          default = cross;
        };
      }
    );
}
