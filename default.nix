let
  defaultPkgs = import <nixpkgs> {};
in

{
  dbus-glib ? defaultPkgs.dbus-glib,
  pkg-config ? defaultPkgs.pkg-config,
  rustPlatform ? defaultPkgs.rustPlatform
}:

rustPlatform.buildRustPackage rec {
  name = "btrust-${version}";
  version = "unstable";

  src = ./.;

  cargoSha256 = "1v7l33z5nrvb29a58rlvwri34vw4dckfxgmixc446qbi1vz4hldl";

  nativeBuildInputs = [
    pkg-config
  ];
  buildInputs = [
    dbus-glib
  ];
}
