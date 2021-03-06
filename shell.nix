with import <nixpkgs> { };

# troubleshooting : if error libmariadb.so.x not found => reinstall diesel_cli (cargo install diesel_cli --no-default-features --features postgres,sqlite)

stdenv.mkDerivation rec {
  name = "kbooks-env";
  buildInputs = with pkgs; [ 
    # rustup
    openssl pkgconfig # needed for installing various cargo packages
    postgresql mysql sqlite # needed for `cargo install diesel_cli`
    docker_compose 

    # needed for app
    gettext
  ];

  # (DATABASE_URL env variable overrides value in .env file)
  shellHook = ''
    export DATABASE_URL=postgres://dbuser:password@localhost:5432/kbooks
    which diesel >/dev/null 2>&1 || cargo install diesel_cli
    which cargo-tarpaulin >/dev/null 2>&1 || cargo install cargo-tarpaulin
  '';

}
