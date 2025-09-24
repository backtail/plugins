let
  pkgs = import (fetchTarball (
    "https://github.com/NixOS/nixpkgs/archive/refs/tags/25.05.tar.gz"
  )) { };

in
pkgs.mkShell {
  buildInputs = [
    pkgs.cargo
    pkgs.rustc
  ];
}
