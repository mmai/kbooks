/*
  Create a NixOS container for kbooks with a nginx reverse proxy and an host entry
  Link this file in /etc/nixos/configuration.nix imports declaration, example:

    imports = [
      /path/to/this/file/container.nix
    ];

  And rebuild the configuration:

    $ nixos-rebuild switch
*/
{ config, lib, pkgs, ... }:

let
  name    = "kbooks";
  domain  = "kbooks.org";
in
rec {
  # Container declaration
  containers."${name}" = {
    privateNetwork = true;
    hostAddress    = "10.0.0.1";
    localAddress   = "10.0.0.2";
 
    autoStart = true;

    config = { config, pkgs, ... }: { 
      # Link the module defined in this directory
      imports = [ ./module.nix ];
      services.kbooks.enable = true;
      networking.firewall.allowedTCPPorts = [ 8080 ];
    };
  };

  # Reverse proxy (host NixOS setting)
  services.nginx = {
    enable = true;
    httpConfig = ''
      server {
        listen 80;
        server_name ${domain};
        location / {
          proxy_pass          http://${name}:8080;
          proxy_http_version  1.1;
        }
      }
    '';
  };

  # Host entry & firewall setting (host NixOS setting)
  networking = {
    firewall.allowedTCPPorts = [ 80 ];
    extraHosts = ''
      127.0.0.1    ${domain}
    '';
  };
}
