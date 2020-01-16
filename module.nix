/* 
  kbooks NixOS module, it can be imported in /etc/nixos/configuration.nix, and the options can be used.
  To use, link this file in /etc/nixos/configuration.nix imports declaration, example:

    imports = [
      /path/to/this/file/module.nix
    ];

  Enable the module option:

    services.kbooks.enable = true;
    networking.firewall.allowedTCPPorts = [ 8080 ];

  And rebuild the configuration:

    $ nixos-rebuild switch

  Documentation: https://nixos.org/nixos/manual/index.html#sec-writing-modules
*/

{ config, pkgs, lib ? pkgs.lib, ... }:

with lib;

let
  cfg = config.services.kbooks;
  # Using the kbooks build file in this directory
  kbooksPackage = (import ./. {});


  databasePassword = if (cfg.database.passwordFile != null) 
    then builtins.readFile cfg.database.passwordFile
    else cfg.database.password;
  databaseUrl = if (cfg.database.createLocally && cfg.database.socket != null) 
    then "postgresql:///${cfg.database.name}?host=${cfg.database.socket}" 
    else "postgresql://${cfg.database.user}:${databasePassword}@${cfg.database.host}:${toString cfg.database.port}/${cfg.database.name}";

  kbooksEnvironment = [
    "DATABASE_URL=${databaseUrl}"
    "SENDING_EMAIL_ADDRESS=${cfg.sending_email}"
    "BASE_URL=${cfg.base_url}"
    "FRONT_URL=${cfg.front_url}"
    "DOMAIN=${cfg.hostname}"
    "SECRET_KEY=${cfg.secret_key}"
    "SENDMAIL=${cfg.sendmail_path}"
    "HASH_ROUNDS=${cfg.hash_rounds}"
  ];
  kbooksEnvFileData = builtins.concatStringsSep "\n" kbooksEnvironment;
  kbooksEnvScriptData = builtins.concatStringsSep " " kbooksEnvironment;

  kbooksEnvFile = pkgs.writeText "kbooks.env" kbooksEnvFileData;
  kbooksEnv = {
    ENV_FILE = "${kbooksEnvFile}";
  };
in
{
  options = {
    services.kbooks = {
      enable = mkEnableOption "kbooks";
      user = mkOption {
        type = types.str;
        default = "kbooks";
        description = "User under which kbooks is ran.";
      };

      group = mkOption {
        type = types.str;
        default = "kbooks";
        description = "Group under which kbooks is ran.";
      };

      database = {
        host = mkOption {
          type = types.str;
          default = "localhost";
          description = "Database host address.";
        };

        port = mkOption {
          type = types.int;
          default = 5432;
          defaultText = "5432";
          description = "Database host port.";
        };

        name = mkOption {
          type = types.str;
          default = "kbooks";
          description = "Database name.";
        };

        user = mkOption {
          type = types.str;
          default = "kbooks";
          description = "Database user.";
        };

        password = mkOption {
          type = types.str;
          default = "";
          description = ''
              The password corresponding to <option>database.user</option>.
              Warning: this is stored in cleartext in the Nix store!
              Use <option>database.passwordFile</option> instead.
          '';
        };

        passwordFile = mkOption {
          type = types.nullOr types.path;
          default = null;
          example = "/run/keys/kbooks-dbpassword";
          description = ''
              A file containing the password corresponding to
              <option>database.user</option>.
          '';
        };

        socket = mkOption {
          type = types.nullOr types.path;
          default = "/run/postgresql";
          defaultText = "/run/postgresql";
          example = "/run/postgresql";
          description = "Path to the unix socket file to use for authentication for local connections.";
        };

        createLocally = mkOption {
          type = types.bool;
          default = true;
          description = "Create the database and database user locally.";
        };
      };

      hostname = mkOption {
        type = types.str;
        description = ''
            The definitive, public domain you will use for your instance.
        '';
        example = "kbooks.yourdomain.net";
      };

      protocol = mkOption {
        type = types.enum [ "http" "https" ];
        default = "https";
        description = ''
            Web server protocol.
        '';
      };

        apiIp = mkOption {
          type = types.str;
          default = "127.0.0.1";
          description = ''
            Kbooks API IP.
          '';
        };

        webWorkers = mkOption {
          type = types.int;
          default = 1;
          description = ''
            Kbooks number of web workers.
          '';
        };

        apiPort = mkOption {
          type = types.port;
          default = 8080;
          description = ''
            Kbooks API Port.
          '';
        };

      base_url = mkOption {
        type = types.str;
        description = ''
            The definitive, public url you will use for your instance.
        '';
        example = "http://kbooksapi.yourdomain.net";
      };

      front_url = mkOption {
        type = types.str;
        description = ''
            The definitive, public url you will use for your instance.
        '';
        example = "http://kbooks.yourdomain.net";
      };

      sending_email = mkOption {
        type = types.str;
        description = ''
            The email address to use to send system emails.
        '';
        example = "kbooks@yourdomain.net";
      };

      sendmail_path = mkOption {
        type = types.str;
        description = ''
            Path to sendmail binary
        '';
        example = "/usr/sbin/sendmail";
      };

      hash_rounds = mkOption {
        type = types.int;
        default = 12;
        description = ''
          between 4 and 31;  4 insecure but fast (for tests), bcrypt.DEFAULT_COST = 12
        '';
        example = "/usr/sbin/sendmail";
      };

      secret_key = mkOption {
        type = types.str;
        description = ''
              API secret key. Generate one using `openssl rand -base64 45` for example.
        '';
        example = "6VhAWVKlqu/dJSdz6TVgEJn/cbbAidwsFvg9ddOwuPRssEs0OtzAhJxLcLVC";
      };
    };
  };

  config = mkIf cfg.enable {
    assertions = [
      { assertion = cfg.database.passwordFile != null || cfg.database.password != "" || cfg.database.socket != null;
        message = "one of services.kbooks.database.socket, services.kbooks.database.passwordFile, or services.kbooks.database.password must be set";
      }
      { assertion = cfg.database.createLocally -> cfg.database.user == cfg.user;
        message = "services.kbooks.database.user must be set to ${cfg.user} if services.kbooks.database.createLocally is set true";
      }
      { assertion = cfg.database.createLocally -> cfg.database.socket != null;
        message = "services.kbooks.database.socket must be set if services.kbooks.database.createLocally is set to true";
      }
      { assertion = cfg.database.createLocally -> cfg.database.host == "localhost";
        message = "services.kbooks.database.host must be set to localhost if services.kbooks.database.createLocally is set to true";
      }
    ];

    services.postgresql = mkIf cfg.database.createLocally {
      enable = true;
      ensureDatabases = [ cfg.database.name ];
      ensureUsers = [
        { name = cfg.database.user;
          ensurePermissions = { "DATABASE ${cfg.database.name}" = "ALL PRIVILEGES"; };
        }
      ];
    };

      services.nginx = {
        enable = true;
        appendHttpConfig = ''
          upstream kbooks-api {
          server ${cfg.apiIp}:${toString cfg.apiPort};
          }
        '';
        virtualHosts = 
        let proxyConfig = ''
          # global proxy conf
          proxy_set_header Host $host;
          proxy_set_header X-Real-IP $remote_addr;
          proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
          proxy_set_header X-Forwarded-Proto $scheme;
          proxy_set_header X-Forwarded-Host $host:$server_port;
          proxy_set_header X-Forwarded-Port $server_port;
          proxy_redirect off;

          # websocket support
          proxy_http_version 1.1;
          proxy_set_header Upgrade $http_upgrade;
          proxy_set_header Connection $connection_upgrade;
        '';
        withSSL = cfg.protocol == "https";
        in {
          "${cfg.hostname}" = {
            enableACME = withSSL;
            forceSSL = withSSL;
            root = "${pkgs.funkwhale}/front";
          # gzip config is nixos nginx recommendedGzipSettings with gzip_types from funkwhale doc (https://docs.funkwhale.audio/changelog.html#id5)
            extraConfig = ''
              add_header Content-Security-Policy "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; font-src 'self' data:; object-src 'none'; media-src 'self' data:";
              add_header Referrer-Policy "strict-origin-when-cross-origin";

              gzip on;
              gzip_disable "msie6";
              gzip_proxied any;
              gzip_comp_level 5;
              gzip_types
              application/javascript
              application/vnd.geo+json
              application/vnd.ms-fontobject
              application/x-font-ttf
              application/x-web-app-manifest+json
              font/opentype
              image/bmp
              image/svg+xml
              image/x-icon
              text/cache-manifest
              text/css
              text/plain
              text/vcard
              text/vnd.rim.location.xloc
              text/vtt
              text/x-component
              text/x-cross-domain-policy;
              gzip_vary on;
            '';
            locations = {
              "/" = { 
                extraConfig = proxyConfig;
                proxyPass = "http://kbooks-api/";
              };
            };
          };
        };
      };

    environment.systemPackages = [ kbooksPackage ];

    systemd.targets.kbooks = {
      description = "Kbooks";
      wants = ["funkwhale-server.service"];
    }; 
    systemd.services = 
    let serviceConfig = {
      User = "${cfg.user}";
      WorkingDirectory = "${pkgs.kbooks}";
      EnvironmentFile =  "${kbooksEnvFile}";
    };
    in { 
      kbooks-psql-init = mkIf cfg.database.createLocally {
        description = "Kbooks database preparation";
        after = [ "postgresql.service" ];
        wantedBy = [ "kbooks-init.service" ];
        before   = [ "kbooks-init.service" ];
        serviceConfig = {
          User = "postgres";
          ExecStart = '' ${config.services.postgresql.package}/bin/psql -d ${cfg.database.name}  -c 'CREATE EXTENSION IF NOT EXISTS "unaccent";' '';
        };
      };
      kbooks-init = {
        description = "Kbooks initialization";
        wantedBy = [ "kbooks-server.service" ];
        before   = [ "kbooks-server.service" ];
        environment = kbooksEnv;
        serviceConfig = {
          User = "${cfg.user}";
          Group = "${cfg.group}";
        };
        script = ''
          # TODO : Create datadir & symkink .env ?

          # TODO : init / migrate database -> embed in executable ?
          diesel setup --config-file diesel-khnum.toml --migration-dir migrations/khnum/postgres/
          diesel migration run --config-file diesel-khnum.toml --migration-dir migrations/khnum/postgres/
          diesel migration run --migration-dir migrations/postgres/

          # TODO : Create superuser ?
        '';
      };
      kbooks-server = { 
        partOf    = [ "kbooks.target" ];
        after    = [ "network.target" ];
        wantedBy = [ "multi-user.target" ];
        serviceConfig = { 
          ExecStart = "${kbooksPackage}/bin/kbooks-api";
          Restart   = "always";
        };
      };
    };
  };

  meta = {
    maintainers = with lib.maintainers; [ mmai ];
  };
}

# TODO :
#  [ ] .env file
#  [ ] database config
