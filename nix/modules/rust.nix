{ inputs, ... }:
{
  imports = [
    inputs.process-compose-flake.flakeModule
    inputs.cargo-doc-live.flakeModule
  ];

  perSystem =
    { config
    , self'
    , pkgs
    , lib
    , system
    , ...
    }:
    let
      rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        extensions = [
          "rust-src"
          "rust-analyzer"
          "clippy"
        ];
        targets = [ "wasm32-unknown-unknown" ];
      };

      rustBuildInputs = [
        pkgs.dioxus-cli
        pkgs.openssl
        pkgs.libiconv
        pkgs.pkg-config
        pkgs.sqlite
        pkgs.wasm-bindgen-cli
      ]
      ++ lib.optionals pkgs.stdenv.isLinux [
        pkgs.lld
        pkgs.gcc
        pkgs.glib
        pkgs.gtk3
        pkgs.libsoup_3
        pkgs.webkitgtk_4_1
        pkgs.xdotool
      ]
      ++ lib.optionals pkgs.stdenv.isDarwin (
        with pkgs.darwin.apple_sdk.frameworks;
        [
          IOKit
          Carbon
          WebKit
          Security
          Cocoa
        ]
      );

      cargoToml = builtins.fromTOML (builtins.readFile ../../Cargo.toml);

      rustPackage =
        package:
        { binary ? package
        , features ? [ ]
        ,
        }:
        (pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        }).buildRustPackage
          {
            pname = package;
            version = cargoToml.package.version;
            src = pkgs.lib.cleanSource ../../.;
            cargoLock.lockFile = ../../Cargo.lock;
            buildInputs = rustBuildInputs;
            nativeBuildInputs = [
              rustToolchain
              pkgs.pkg-config
            ];
            buildPhase = ''
              mkdir -p .cargo
              cp ${../../Cargo.lock} Cargo.lock
              cargo build --release --package ${package} ${
                lib.concatStringsSep " " (map (f: "--features ${f}") features)
              }
            '';
            installPhase = ''
              mkdir -p $out/bin
              ls -alR target/release
              cp target/release/${binary} $out/bin/
            '';
            doCheck = false; # Disable tests to avoid building deps for them
          };
    in
    {
      _module.args.pkgs = import inputs.nixpkgs {
        inherit system;
        overlays = [
          inputs.rust-overlay.overlays.default
        ];
      };

      packages = {
        hot_dog = (
          rustPackage "hot_dog" {
            binary = "hot_dog";
            features = [ "server" ];
          }
        );
        default = self'.packages.hot_dog;
      };

      devShells.default = pkgs.mkShell {
        name = "hot-dog-dev";
        buildInputs = rustBuildInputs;
        nativeBuildInputs = [
          rustToolchain
        ];
        shellHook = ''
          # For rust-analyzer 'hover' tooltips to work.
          export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library";
        '';
      };
    };
}
