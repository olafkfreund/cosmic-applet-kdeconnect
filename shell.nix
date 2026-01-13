{ pkgs ? import <nixpkgs> {
    overlays = [
      (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
    ];
  }
}:

let
  # Rust toolchain with extensions
  rustToolchain = pkgs.rust-bin.stable.latest.default.override {
    extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
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

  # Build and runtime dependencies
  buildInputs = with pkgs; [
    rustToolchain
    pkg-config
    cmake
    just
    openssl
    
    # Development tools
    git
    gnumake
    cargo-watch
    cargo-edit
    
    # Debugging and profiling
    gdb
    valgrind
    
    # Documentation
    mdbook
  ] ++ cosmicLibs;

  runtimeLibs = with pkgs; [
    glib
    gtk3
    pango
    cairo
    gdk-pixbuf
    atk
  ];

in pkgs.mkShell {
  inherit buildInputs;

  nativeBuildInputs = with pkgs; [
    pkg-config
    cmake
  ];

  # Environment variables
  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
  RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
  RUST_BACKTRACE = "1";
  RUST_LOG = "debug";
  
  # Library paths
  LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath runtimeLibs}";
  PKG_CONFIG_PATH = "${pkgs.lib.makeSearchPath "lib/pkgconfig" buildInputs}";

  shellHook = ''
    echo "🚀 COSMIC KDE Connect Applet Development Environment"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "🦀 Rust Tools:"
    echo "  rustc:         $(rustc --version)"
    echo "  cargo:         $(cargo --version)"
    echo "  rust-analyzer: Available"
    echo "  clippy:        Available"
    echo ""
    echo "📦 Build System:"
    echo "  just:          $(just --version)"
    echo "  cmake:         $(cmake --version | head -1)"
    echo ""
    echo "🔧 Available Commands:"
    echo "  just build          - Build all components"
    echo "  just run-applet     - Run applet in development"
    echo "  just test           - Run all tests"
    echo "  just fmt            - Format code"
    echo "  just lint           - Run clippy linter"
    echo "  just check          - Run fmt + lint + test"
    echo "  just clean          - Clean build artifacts"
    echo ""
    echo "  cargo watch -x build     - Auto-rebuild on changes"
    echo "  cargo watch -x test      - Auto-test on changes"
    echo ""
    echo "🌐 Network Requirements:"
    echo "  Ports 1714-1764 (TCP/UDP) must be open for KDE Connect"
    echo ""
    echo "📝 Logging:"
    echo "  RUST_LOG=debug cargo run    - Enable debug logs"
    echo "  RUST_LOG=trace cargo run    - Enable trace logs"
    echo ""
    echo "🏗️  Project Structure:"
    echo "  kdeconnect-protocol/       - Core protocol library"
    echo "  cosmic-applet-kdeconnect/  - Panel applet"
    echo "  cosmic-kdeconnect/         - Full application"
    echo "  kdeconnect-daemon/         - Background service"
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "✨ Environment ready! Start coding..."
    echo ""
    
    # Create directories if they don't exist
    mkdir -p .cargo
    
    # Create .cargo/config.toml for better defaults
    cat > .cargo/config.toml << 'EOF'
[build]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.x86_64-unknown-linux-gnu]
linker = "clang"

[profile.dev]
split-debuginfo = "unpacked"
EOF
    
    echo "📄 Created .cargo/config.toml with optimized settings"
  '';
}
