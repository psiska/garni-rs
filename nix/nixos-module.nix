{ lib, config, pkgs, ... }:
let
  toml = pkgs.formats.toml { };
  configFilePath = toml.generate "config.toml" config.services.garni-rs.settings;
in
{
  options = {
    services.garni-rs = {
      package = lib.mkOption {
        defaultText = lib.literalMD "`packages.default` from the garni-rs flake";
      };
      enable = lib.mkOption {
        type = lib.types.bool;
        default = false;
        description = ''
          Enable the garni-rs meteo station collector
        '';
      };
      settings = {
        server = {
          host = lib.mkOption {
            type = str;
            default = "0.0.0.0";
            description = ''Interface to bind to.'';
          };
          port = lib.mkOption {
            type = port;
            default = 8305;
            description = ''Port to listen to.'';
          };
          garniUpdatePath = lib.mkOption {
            type = str;
            default = "/weatherstation/updateweatherstation.php";
            description = ''URL path where garni station sent updates.'';
          };
          prometheusV004Path = lib.mkOption {
            type = str;
            default = "/metrics/prometheus";
            description = ''URL path where prometheus metrics in v0.0.4 format are presented'';
          };
          prometheusV1Path = lib.mkOption {
            type = str;
            default = "/metrics/prometheus_v1";
            description = ''URL path where prometheus metrics in v1.0.0 format are presented'';
          };
          prometheusOpenmetricsPath = lib.mkOption {
            type = str;
            default = "/metrics/openmetrics_v1";
            description = ''URL path where openmetrics metrics in v1.0.0 format are presented'';
          };
        };
        logging = {
          level = lib.mkOption {
            type = enum [
              "info"
              "debug"
              "warn"
            ];
            default = "info";
            description = ''
              Logging level of the garni-rs.
            '';
          };
        };
      };
    };
  };
  config = lib.mkIf config.services.garni-rs.enable {
    users.extraGroups.garni = { };

    users.extraUsers.garni = {
      description = "Garni-rs server";
      group = "garni";
      # home = baseDir;
      isSystemUser = true;
    };

    environment.systemPackages = [ config.services.garni-rs.package ];

    systemd.services.garni-rs = {
      wantedBy = [ "multi-user.target" ];
      serviceConfig = {
        ExecStart = "${config.services.garni-rs.package}/bin/garni-rs --config-file ${configFilePath}";
        User = "garni";
        Group = "garni";
        PermissionsStartOnly = true;
        Restart = "on-failure";
        RestartSec = "3";
        ProtectSystem = "full";
        ProtectHostname = "true";
        ProtectKernelTunables = "true";
        ProtectControlGroups = "true";
        RestrictRealtime = "true";
      };
    };
  };
}
