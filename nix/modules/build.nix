_: {
  perSystem = { self', pkgs, lib, ... }: {
    # Build-specific configuration for the hot_dog project
    rust-project.crates."hot_dog".crane.args = {
      buildInputs = lib.optionals pkgs.stdenv.isDarwin (
        with pkgs.darwin.apple_sdk.frameworks; [
          IOKit
        ]
      );
    };
    
    # Package definitions
    packages.default = self'.packages.hot_dog;
  };
}