let
  nixpkgs = import <nixpkgs> {};
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "music-reader-env";
    buildInputs = [
      cargo
      rustc
      pkgconfig
      openssl.dev
      nix
      protobuf
    ];
    OPENSSL_DEV=openssl.dev;
  }