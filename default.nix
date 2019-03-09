{ pkgsPath ? <nixpkgs>, crossSystem ? null }:

let
    mozOverlay = import (
        builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz
    );
    pkgs = import pkgsPath {
        overlays = [ mozOverlay ];
        inherit crossSystem;
    };
    targets = [ pkgs.stdenv.targetPlatform.config ];
in

with pkgs;

stdenv.mkDerivation {
    name = "btrust";

    # build time dependencies targeting the build platform
    depsBuildBuild = [
        buildPackages.stdenv.cc
    ];
    HOST_CC = "cc";

    # build time dependencies targeting the host platform
    nativeBuildInputs = [
        (buildPackages.buildPackages.latest.rustChannels.nightly.rust.override { inherit targets; })
        buildPackages.buildPackages.rustfmt
        buildPackages.buildPackages.pkg-config
    ];
    shellHook = ''
        export RUSTFLAGS="-C linker=$CC"
    '';
    CARGO_BUILD_TARGET = targets;

    buildInputs = [ dbus-glib ];
}
