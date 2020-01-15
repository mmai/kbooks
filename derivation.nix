# { lib, rustPlatform, pkgconfig, openssl, postgresql, sqlite, gettext}:
# rustPlatform.buildRustPackage rec {
{ lib, fetchFromGitHub, makeRustPlatform, pkgs, pkgconfig, openssl, postgresql, sqlite, gettext}:
let
  mozRepo = fetchFromGitHub {
    owner = "mozilla";
    repo = "nixpkgs-mozilla";
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
  version = "0.1.0";
  cargoSha256 = "0mbf6rknm2g3dg8mw7r4060mxnzlay87cxwfvm7qksrm8zbbm1qk";
  src = ./.;

  buildInputs = [
    pkgconfig
    openssl
    postgresql sqlite
    gettext
  ];

  # doCheck = false;
  checkPhase = ''
    runHook preCheck
    echo "Running cargo test on kbooks-api"
    cd kbooks-api && cargo test --features test
    cd -
    runHook postCheck
  '';

  meta = {
    description = "Books management";
    maintainers = with lib.maintainers; [ mmai ];
  };
}
