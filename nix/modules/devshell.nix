_: {
  perSystem =
    { config
    , self'
    , pkgs
    , ...
    }:
    {
      devShells.default = pkgs.mkShell {
        name = "hot_dog-shell";
        inputsFrom = [
          self'.devShells.rust
          config.pre-commit.devShell # See ./nix/modules/pre-commit.nix
        ];
        packages = with pkgs; [
          # nix stuff
          nixd
          nixfmt-rfc-style
          wslu

          # rust stuff
          rustfmt
          clippy
          bacon
          config.process-compose.cargo-doc-live.outputs.package
          cargo-binstall

          # dioxus and deps
          dioxus-cli
          wasm-bindgen-cli
          sqlite

          ## dioxus bundling deps
          # binaryen

          ## tauri deps
          cargo-tauri
          cargo
          gobject-introspection
          pkg-config
          at-spi2-atk
          atkmm
          cairo
          gdk-pixbuf
          glib
          gtk3
          harfbuzz
          librsvg
          libsoup_3
          pango
          webkitgtk_4_1
          openssl
          xdotool
          wasm-pack

          # utilities
          gitflow

          # # gigdot programs
          # inputs.gigdot.packages.${system}.quick-results
          # inputs.gigdot.packages.${system}.upjust
          # inputs.gigdot.packages.${system}.upspell
          # inputs.gigdot.packages.${system}.upflake
          # inputs.gigdot.packages.${system}.cargo-update
        ];
        shellHook = ''
          echo "welcome to the rust development environment for the hot_dog package" | ${pkgs.cowsay}/bin/cowsay | ${pkgs.lolcat}/bin/lolcat 2> /dev/null;
        '';
      };
    };
}
