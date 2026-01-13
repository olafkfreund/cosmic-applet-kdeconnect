{
  description = "COSMIC applet for KDE Connect - Development Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Rust toolchain
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" ];
        };

        # COSMIC Desktop libraries
        cosmicLibs = with pkgs; [
          libxkbcommon
          wayland
          wayland-protocols
          libGL
          libglvnd
          mesa
          pixman
          libinput
          libxcb
          xcb-util-wm
          xcb-util-image
          libdrm
          fontconfig
          freetype
          udev
          dbus
          libpulseaudio
          expat
        ];

        # Build dependencies
        buildInputs = with pkgs; [
          rustToolchain
          pkg-config
          cmake
          just
          openssl
          
          # COSMIC specific
          libxkbcommon
          libwayland
          libinput
          
          # TLS/Crypto
          rustls
          
          # Development tools
          git
          gnumake
        ] ++ cosmicLibs;

        # Runtime dependencies
        runtimeInputs = with pkgs; [
          glib
          gtk3
          pango
          cairo
          gdk-pixbuf
          atk
        ];

        # Shell environment
        shellHook = ''
          echo "🚀 COSMIC KDE Connect Applet Development Environment"
          echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
          echo "Rust version: $(rustc --version)"
          echo "Cargo version: $(cargo --version)"
          echo ""
          echo "📦 Available commands:"
          echo "  just build          - Build all components"
          echo "  just run-applet     - Run applet in development"
          echo "  just test           - Run tests"
          echo "  just fmt            - Format code"
          echo "  just lint           - Run clippy"
          echo ""
          echo "🔧 Environment configured for COSMIC Desktop development"
          echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
          
          # Set up environment variables
          export RUST_BACKTRACE=1
          export RUST_LOG=debug
          
          # Library paths for runtime
          export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath runtimeInputs}:$LD_LIBRARY_PATH"
          
          # PKG_CONFIG paths
          export PKG_CONFIG_PATH="${pkgs.lib.makeSearchPath "lib/pkgconfig" buildInputs}:$PKG_CONFIG_PATH"
        '';

      in
      {
        # Development shell
        devShells.default = pkgs.mkShell {
          inherit buildInputs shellHook;
          
          nativeBuildInputs = with pkgs; [
            pkg-config
            cmake
          ];

          # Additional environment variables
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };

        # Package definition
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "cosmic-applet-kdeconnect";
          version = "0.1.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = [ pkgs.pkg-config pkgs.cmake ];
          buildInputs = cosmicLibs ++ [ pkgs.openssl ];

          meta = with pkgs.lib; {
            description = "KDE Connect applet for COSMIC Desktop";
            homepage = "https://github.com/yourusername/cosmic-applet-kdeconnect";
            license = licenses.gpl3Plus;
            maintainers = [ ];
            platforms = platforms.linux;
          };
        };

        # Apps for running
        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.default;
        };
      }
    );
}
