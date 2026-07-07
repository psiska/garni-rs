{ lib, config, ... }: {
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

      port = lib.mkOption {
        type = lib.types.int;
        default = 8305;
        description = ''
          The port for service to listen on. Cannot be changed yet.
        '';
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
        ExecStart = "${config.services.garni-rs.package}/bin/garni-rs";
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
