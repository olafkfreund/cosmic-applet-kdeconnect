{ pkgs ? import <nixpkgs> { }, ... }:

let
  # Import the package
  cosmic-connect = pkgs.callPackage ./package.nix { };

in {
  # Test 1: Basic package build and installation
  package-build = pkgs.runCommand "cosmic-connect-build-test" { } ''
    # Test that the package builds
    ${cosmic-connect}/bin/cosmic-connect-daemon --version > $out || true
    ${cosmic-connect}/bin/cosmic-applet-connect --version >> $out || true

    # Verify binaries exist
    test -f ${cosmic-connect}/bin/cosmic-connect-daemon || exit 1
    test -f ${cosmic-connect}/bin/cosmic-applet-connect || exit 1

    echo "Package build test: PASSED" >> $out
  '';

  # NixOS VM tests disabled until module is stable
  # Run with: nix build .#checks.<system>.package-build
}
