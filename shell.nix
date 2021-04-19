let
  pkgs = import (fetchTarball
    "https://github.com/NixOS/nixpkgs/archive/4cb48cc25622334f17ec6b9bf56e83de0d521fb7.tar.gz")
    { };
in with pkgs;
mkShell {
  buildInputs =
    [ cargo-outdated cargo-watch sqlx-cli cargo-edit nodejs-15_x ];
}
