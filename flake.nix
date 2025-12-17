{
  description = "FSLint - Cross-platform file system intelligence tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rustfmt" "clippy" ];
        };

        # Build inputs for all platforms
        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
          just
          cargo-watch
          cargo-audit
          cargo-outdated
          cargo-license
        ];

        buildInputs = with pkgs; [
          openssl
          git
        ] ++ lib.optionals stdenv.isDarwin [
          darwin.apple_sdk.frameworks.Security
          darwin.apple_sdk.frameworks.SystemConfiguration
        ];

        # FSLint package
        fslint = pkgs.rustPlatform.buildRustPackage {
          pname = "fslint";
          version = "0.1.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = nativeBuildInputs;
          buildInputs = buildInputs;

          # Run tests during build
          doCheck = true;

          # Build only the CLI binary
          cargoBuildFlags = [ "-p" "fslint" ];

          # Install completion scripts
          postInstall = ''
            # Future: Add shell completions here
          '';

          meta = with pkgs.lib; {
            description = "Cross-platform file system intelligence tool with plugin architecture";
            homepage = "https://github.com/Hyperpolymath/file-soup";
            license = with licenses; [ mit agpl3Plus ];
            maintainers = [ ];
            platforms = platforms.unix;
          };
        };

      in
      {
        # Default package
        packages = {
          default = fslint;
          fslint = fslint;
        };

        # Development shell
        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;

          shellHook = ''
            echo "FSLint Development Environment"
            echo ""
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo ""
            echo "Available commands:"
            echo "  just --list    - List all development commands"
            echo "  just build     - Build FSLint"
            echo "  just test      - Run tests"
            echo "  just ci        - Run all CI checks"
            echo "  just scan      - Run FSLint on current directory"
            echo ""
          '';

          # Set environment variables
          RUST_BACKTRACE = "1";
          RUST_LOG = "info";
        };

        # Alternative development shells
        devShells = {
          # Minimal shell (just Rust)
          minimal = pkgs.mkShell {
            buildInputs = [ rustToolchain ];
          };

          # Full shell with all tools
          full = pkgs.mkShell {
            buildInputs = buildInputs ++ nativeBuildInputs ++ (with pkgs; [
              tokei
              ripgrep
              fd
              bat
              exa
              cargo-flamegraph
              cargo-criterion
            ]);

            shellHook = ''
              echo "FSLint Full Development Environment (All Tools)"
            '';
          };
        };

        # Apps
        apps.default = flake-utils.lib.mkApp {
          drv = fslint;
        };

        # Formatter for nix files
        formatter = pkgs.nixpkgs-fmt;

        # Checks (run with: nix flake check)
        checks = {
          # Build check
          build = fslint;

          # Format check
          fmt = pkgs.runCommand "fmt-check" { } ''
            ${pkgs.cargo}/bin/cargo fmt --check --manifest-path=${./.}/Cargo.toml
            touch $out
          '';

          # Clippy check
          clippy = pkgs.runCommand "clippy-check" {
            buildInputs = nativeBuildInputs ++ buildInputs;
          } ''
            ${pkgs.cargo}/bin/cargo clippy --manifest-path=${./.}/Cargo.toml -- -D warnings
            touch $out
          '';

          # Test check
          test = pkgs.runCommand "test-check" {
            buildInputs = nativeBuildInputs ++ buildInputs;
          } ''
            ${pkgs.cargo}/bin/cargo test --manifest-path=${./.}/Cargo.toml --workspace
            touch $out
          '';
        };

        # Hydra jobs (for CI)
        hydraJobs = {
          inherit (self.packages.${system}) fslint;
          inherit (self.checks.${system}) build fmt clippy test;
        };
      }
    );
}
