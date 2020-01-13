# { lib, rustPlatform }:
# rustPlatform.buildRustPackage rec {
{ lib, fetchFromGitHub, makeRustPlatform, pkgs }:
let
  mozRepo = fetchFromGitHub {
    owner = "mozilla";
    repo = "nixpkgs-mozilla";
    # rev = "ac8e9d7bbda8fb5e45cae20c5b7e44c52da3ac0c";
    rev = "b5f2af80f16aa565cef33d059f27623d258fef67";
    sha256 = "0s552nwnxcn6nnzrqaazhdgx5mm42qax9wy1gh5n6mxfaqi6dvbr";
  };
  # `mozPkgs` is the package set of `mozRepo`; this differs from their README
  # where they use it as an overlay rather than a separate package set
  mozPkgs = import "${mozRepo}/package-set.nix" { inherit pkgs; };
  channel = mozPkgs.rustChannelOf { date = "2019-11-29"; channel = "nightly"; };
  nightlyRustPlatform = makeRustPlatform {
    rustc = channel.rust;
    cargo = channel.cargo;
  };
in

nightlyRustPlatform.buildRustPackage rec {
  pname = "kbooks";
  version = "0.0.1";
  cargoSha256 = "1iga3320mgi7m853la55xip514a3chqsdi1a1rwv25lr9b1p7vd3";
  src = ./.;

  meta = {
    description = "Books management";
    maintainers = with lib.maintainers; [ mmai ];
  };
}
