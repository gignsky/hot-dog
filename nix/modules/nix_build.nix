_: {
  perSystem = { self', pkgs, lib, ... }: {
    # Extended build configuration for the hot_dog project
    # This module augments the basic rust-project config from rust.nix
    
    # Additional build-specific configuration can be added here:
    rust-project.crates."hot_dog".crane.args = {
      # Example: Additional environment variables for the build
      # RUSTFLAGS = "-C target-cpu=native";
      
      # Example: Additional build inputs for specific features
      # buildInputs = with pkgs; [ openssl pkg-config ];
      
      # Example: Custom cargo arguments
      # cargoExtraArgs = "--features production";
    };
    
    # Extended package definitions beyond the basic ones in rust.nix
    packages = {
      # Example: Production build with optimizations
      # hot_dog-release = self'.packages.hot_dog.overrideAttrs (old: {
      #   CARGO_BUILD_RELEASE = "true";
      #   cargoExtraArgs = "--features production --release";
      # });
      
      # Example: Documentation-only package
      # hot_dog-docs = self'.packages.hot_dog-doc;
      
      # Example: Container image
      # hot_dog-docker = pkgs.dockerTools.buildImage {
      #   name = "hot_dog";
      #   tag = "latest";
      #   contents = [ self'.packages.hot_dog ];
      # };
    };
    
    # Extended build checks and validations
    checks = {
      # Custom build validation beyond basic clippy
      # hot_dog-security-audit = pkgs.runCommand "security-audit" {} ''
      #   ${pkgs.cargo-audit}/bin/cargo-audit audit
      #   touch $out
      # '';
      
      # Performance regression tests
      # hot_dog-benchmarks = pkgs.runCommand "benchmarks" {} ''
      #   ${self'.packages.hot_dog}/bin/hot_dog --benchmark
      #   touch $out
      # '';
    };
  };
}