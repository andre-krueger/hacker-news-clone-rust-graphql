{ lib, pkgs, ... }:
let
  nodeDependencies = import ./frontend_parcel/default.nix;
  backend = import ./backend/postcard.nix;
in lib.mkMerge [
  (lib.mkIf (builtins.getEnv "DEV" == "true") {
    networking.hostName = "nixos-dev";
    services.postgresql.settings = { listen_addresses = pkgs.lib.mkForce "*"; };
    services.redis.bind = "*";
    services.postgresql.authentication = lib.mkForce ''
      local all all trust
            host all             all              0.0.0.0/0                       trust
            host  all             all              ::/0                            trust'';
    virtualisation.memorySize = 16000;
    virtualisation.cores = 4;
    virtualisation.diskSize = 100 * 1024;
    virtualisation.writableStoreUseTmpfs = false;
    services.nginx = {
      enable = true;
      recommendedProxySettings = true;
      recommendedTlsSettings = true;
      virtualHosts."localhost" = {
        forceSSL = true;
        sslCertificate =
          "${pkgs.path}/nixos/tests/common/acme/server/acme.test.cert.pem";
        sslCertificateKey =
          "${pkgs.path}/nixos/tests/common/acme/server/acme.test.key.pem";
        locations."/".proxyPass = "http://10.0.2.2:8000";
        locations."/static/".alias =
          "/var/www/postcard-website/frontend/static/";
      };

    };

    virtualisation.forwardPorts = [
      {
        from = "host";
        host.port = 8080;
        guest.port = 80;
      }
      {
        from = "host";
        host.port = 6379;
        guest.port = 6379;
      }
      {
        from = "host";
        host.port = 5432;
        guest.port = 5432;
      }
      {
        from = "host";
        host.port = 8081;
        guest.port = 443;
      }
    ];

    nixos-shell.mounts.extraMounts = {
      "/var/www/postcard-website/frontend" = {
        target = ./backend;
        cache = "none";
      };
      "/var/www/postcard-website/backend" = {
        target = ./backend;
        cache = "none";
      };
    };
    nixos-shell.mounts = {
      mountHome = false;
      mountNixProfile = false;
      cache = "none";
    };
  })
  (lib.mkIf (builtins.getEnv "PROD" == "true") {
    environment.systemPackages =
      # [ lib.packages.${pkgs.system}.postcard-backend ];
      [ backend pkgs.sqlx-cli ];
    nixos-shell.mounts = {
      mountHome = false;
      mountNixProfile = false;
      cache = "none";
    };

    virtualisation.forwardPorts = [
      {
        from = "host";
        host.port = 8080;
        guest.port = 80;
      }
      {
        from = "host";
        host.port = 6379;
        guest.port = 6379;
      }
      {
        from = "host";
        host.port = 5432;
        guest.port = 5432;
      }
      {
        from = "host";
        host.port = 8081;
        guest.port = 443;
      }
    ];

    services.postgresql.authentication = lib.mkForce ''
      local all all trust
            host all             all              0.0.0.0/0                       trust
            host  all             all              ::/0                            trust'';

    systemd.services.backend = {
      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ];
      description = "Start the irc client of username.";
      serviceConfig = {
        # Type = "forking";
        # User = "username";
        # ExecStart = ''${pkgs.screen}/bin/screen -dmS irc ${pkgs.irssi}/bin/irssi'';
        EnvironmentFile = "${backend}/bin/.env";
        ExecStart = "${backend}/bin/backend";
        # ExecStop = ''${pkgs.screen}/bin/screen -S irc -X quit'';
      };
    };

    services.nginx = {
      enable = true;
      recommendedProxySettings = true;
      recommendedTlsSettings = true;
      virtualHosts."localhost" = {
        forceSSL = true;
        sslCertificate =
          "${pkgs.path}/nixos/tests/common/acme/server/acme.test.cert.pem";
        sslCertificateKey =
          "${pkgs.path}/nixos/tests/common/acme/server/acme.test.key.pem";
        locations."/".proxyPass = "http://localhost:8000";
        locations."/static/".alias = "${backend}/static/";
      };

    };
  })
  {
    services.openssh.enable = true;
    services.postgresql.enable = true;
    services.redis.enable = true;
    nix.nixPath = [ "nixpkgs=${pkgs.path}" ];
  }
]
