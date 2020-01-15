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


    environment.systemPackages = [ kbooksPackage ];

    systemd.services.kbooks = { 
      after    = [ "network.target" ];
      wantedBy = [ "multi-user.target" ];
      serviceConfig = { 
        ExecStart = "${kbooksPackage}/bin/kbooks-api";
        Restart   = "always";
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
