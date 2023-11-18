{inputs, ...}: {
  perSystem = {
    config,
    pkgs,
    system,
    inputs',
    self',
    ...
  }: let
    extraPackages = [
      pkgs.pkg-config
    ];
    withExtraPackages = base: base ++ extraPackages;
    bevyDependencies = [
      pkgs.llvmPackages.bintools
      pkgs.udev
      pkgs.alsa-lib
      pkgs.vulkan-loader
      pkgs.xorg.libX11
      pkgs.xorg.libXcursor
      pkgs.xorg.libXrandr
      pkgs.xorg.libXi
      pkgs.libxkbcommon
      pkgs.wayland
      pkgs.clang
    ];

    craneLib = inputs.crane.lib.${system}.overrideToolchain self'.packages.rust-toolchain;

    common-build-args = rec {
      src = inputs.nix-filter.lib {
        root = ../.;
        include = [
          "crates"
          "Cargo.toml"
          "Cargo.lock"
        ];
      };

      # TODO: change the name to reflect the project
      pname = "rust-crane";

      nativeBuildInputs = withExtraPackages [];
      buildInputs = bevyDependencies;
      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath nativeBuildInputs;
    };

    deps-only = craneLib.buildDepsOnly ({} // common-build-args);

    packages = {
      default = packages.client;
      client = craneLib.buildPackage ({
          pname = "game-client";
          cargoArtifacts = deps-only;
          cargoExtraArgs = "--bin game-client";
          meta.mainProgram = "game-client";
        }
        // common-build-args);

      cargo-doc = craneLib.cargoDoc ({
          cargoArtifacts = deps-only;
        }
        // common-build-args);
    };

    checks = {
      clippy = craneLib.cargoClippy ({
          cargoArtifacts = deps-only;
          cargoClippyExtraArgs = "--all-features -- --deny warnings";
        }
        // common-build-args);

      rust-fmt = craneLib.cargoFmt ({
          inherit (common-build-args) src;
        }
        // common-build-args);

      rust-tests = craneLib.cargoNextest ({
          cargoArtifacts = deps-only;
          partitions = 1;
          partitionType = "count";
        }
        // common-build-args);
    };
  in rec {
    inherit packages checks;

    apps = {
      cli = {
        type = "app";
        program = pkgs.lib.getBin self'.packages.cli;
      };
      default = apps.cli;
    };

    legacyPackages = {
      cargoExtraPackages = extraPackages;
      bevyDependencies = bevyDependencies;
    };
  };
}
