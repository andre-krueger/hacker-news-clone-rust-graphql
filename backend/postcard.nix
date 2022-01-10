let
  # nodeDependencies = (pkgs.callPackage ../frontend_parcel/default.nix
  #
  #   { }).shell.nodeDependencies;
  pkgs = import <nixpkgs> { };
  nodeDependencies = import ../frontend_parcel/default.nix;
  # nodeDependencies = (pkgs.callPackage ../frontend_parcel/default.nix {
  #   # preRebuild = ''
  #   # override = { };
  #   #   sed -i -e "s|#!/usr/bin/env node|#! ${pkgs.nodejs}/bin/node|" node_modules/node-gyp-build/bin.js
  #   # '';
  #   # packageOverride = _: { };
  #   # postUnpack = ''
  #   #   lsxnent
  #   #       chmod -R +rw .
  #   #     '';
  # }).shell.nodeDependencies.override { };
  sources = import ./nix/sources.nix;
  naersk = pkgs.callPackage sources.naersk { };
in naersk.buildPackage {
  # src = [
  #   ../frontend_parcel/static

  #   ./.
  # ];
  src = ./.;
  root = ./.;
  # copySourcesFrom = ../frontend_parcel;
  # cargoBuildOptions = x: x ++ [ "SQLX_OFFLINE=true" ];
  SQLX_OFFLINE = true;
  NODE_PATH = "/lib/node_modules";
  PATH = "/bin:$PATH";
  # DATABASE_URL = "postgres://postgres:postgres@localhost:5432/postgres";
  # cargoBuildOptions = _: [ "--verbose --bin backend" ];
  overrideMain = attrs: {
    postInstall = ''
            mkdir -p $out/static/
      cp .env $out/bin/
                               cp -a static/. $out/static/
                                  '';
  };
  buildInputs = [
    pkgs.sqlx-cli
    pkgs.nodejs-16_x
    nodeDependencies

    #
  ];
  # preBuild = "";
  # override = _:
  #   {

  #     # postUnpack = ''
  #     #   lsxnent
  #     #       chmod -R +rw .
  #     #     '';
  #   };
  # overrideMain = _: { installPhase = x: x ++ "mkdir /var/www/postcard"; };
  # overrideMain = _:
  #   {
  #     # postInstall = ''
  #     #   lsx
  #     # '';

  #     # installPhase = ''
  #     #         # mkdir -p $out
  #     #   cp -r . $out
  #     #                   # cp -r static ''${out}/var/www/static
  #     #             '';
  #     # patchPhase = ''
  #     #   # ls
  #     #   #       cd frontend_parcel
  #     #   #       chmod +x /build/hacker-news-clone-rust-graphql/frontend_parcel/node_modules/.bin/parcel
  #     #   #       patchShebangs /build/hacker-news-clone-rust-graphql/frontend_parcel/node_modules/.bin/parcel
  #     #   #     '';
  #     # buildPhase = ''
  #     #   ls
  #     #        # cd frontend_parcel
  #     #              # npm run parcel:build
  #     #                                       '';

  #     # buildPhase = ''
  #     #   cd frontend_parcel
  #     #   npm run parcel:build
  #     #         #             ln -s $NODE_PATH node_modules
  #     #         # lxnts
  #     #                   '';
  #     # preBuild = ''
  #     #   ls
  #     #                       # cd frontend_parcel
  #     #                      ln -s ${nodeDependencies}/lib/node_modules ./node_modules
  #     #                      export PATH="${nodeDependencies}/bin:$PATH"

  #     #         #               npm run parcel:build
  #     #                       # cd ../backend
  #     #                   '';
  #     # postBuild = ''
  #     #   ls $out
  #     # '';
  #   };

  # override = _: {
  #   preBuild = ''

  #   '';
  # };
  # shellHook = ''

  #    '';
  # inherit nodeDependencies;
}
