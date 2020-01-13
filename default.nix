/* Derivation file for the binserver project
   callPackage style is used for better integration with nixpkgs.
*/

/* 
  For improved determinism, nixpkgs version can be pinned to a specific version or even commit:

    { pkgs ? import (fetchTarball https://github.com/nixos/nixpkgs-channels/archive/nixos-16.09.tar.gz) {} }:
    { pkgs ? import (fetchTarball https://github.com/nixos/nixpkgs-channels/archive/nixos-unstable.tar.gz) {} }:
    { pkgs ? import (fetchTarball https://github.com/nixos/nixpkgs/58d44a3.tar.gz) {} }:

*/

{ pkgs ? import <nixpkgs> {} }:

pkgs.callPackage ./derivation.nix {}
