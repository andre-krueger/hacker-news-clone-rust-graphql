let
  pkgs = import <nixpkgs> { };
  sources = import ./nix/sources.nix;
  napalm = pkgs.callPackage sources.napalm { };
in napalm.buildPackage ./. {

  # postInstall = ''
  #   mkdir -p $out/var/www/static
  #       cp -r static $out/var/www/static
  #     '';
  postInstall = ''

    ls $out
        pwd
               mkdir -p $out/share/static/
                     cp -rf static $out/share/static/
      '';
  npmCommands = [ "patchShebangs node_modules" " npm run parcel:build" ];
}
