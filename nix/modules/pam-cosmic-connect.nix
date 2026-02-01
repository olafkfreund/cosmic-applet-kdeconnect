{
  config,
  lib,
  pkgs,
  ...
}:

with lib;

let
  cfg = config.security.pam.services.cosmic-connect;

in
{
  options.security.pam.services.cosmic-connect = {
    enable = mkEnableOption "COSMIC Connect phone authentication for PAM services";

    timeout = mkOption {
      type = types.int;
      default = 30;
      example = 60;
      description = ''
        Timeout in seconds for phone authentication requests.
        If the phone does not respond within this time, authentication falls back to password.
      '';
    };

    services = mkOption {
      type = types.listOf types.str;
      default = [
        "login"
        "sudo"
        "cosmic-greeter"
      ];
      example = [
        "login"
        "sudo"
        "cosmic-greeter"
        "polkit-1"
      ];
      description = ''
        List of PAM services to enable phone authentication for.
        Common services include:
        - login: Console login
        - sudo: Privilege escalation
        - cosmic-greeter: Display manager login
        - polkit-1: PolicyKit authentication dialogs
      '';
    };

    fallbackToPassword = mkOption {
      type = types.bool;
      default = true;
      description = ''
        Whether to fall back to password authentication if phone authentication fails.
        Disabling this is NOT recommended as it may lock you out of your system.
      '';
    };

    package = mkOption {
      type = types.package;
      default = pkgs.cosmic-connect or (throw "cosmic-connect package not available in nixpkgs");
      defaultText = literalExpression "pkgs.cosmic-connect";
      description = ''
        The cosmic-connect package that provides the PAM module.
        The package must include lib/security/pam_cosmic_connect.so
      '';
    };
  };

  config = mkIf cfg.enable {
    # Assertions for safety
    assertions = [
      {
        assertion = cfg.timeout > 0;
        message = "security.pam.services.cosmic-connect.timeout must be greater than 0";
      }
      {
        assertion = cfg.services != [ ];
        message = "security.pam.services.cosmic-connect.services must not be empty";
      }
      {
        assertion = cfg.fallbackToPassword || (builtins.elem "sudo" cfg.services == false);
        message = ''
          Disabling fallbackToPassword for sudo service is extremely dangerous.
          You may lock yourself out of the system. Please reconsider this configuration.
        '';
      }
    ];

    # Warning for security considerations
    warnings =
      optional
        (!cfg.fallbackToPassword)
        ''
          COSMIC Connect phone authentication is configured WITHOUT password fallback.
          If your phone is unavailable or the daemon fails, you will be locked out.
          This configuration is NOT recommended for production systems.
        ''
      ++ optional
        (builtins.elem "polkit-1" cfg.services)
        ''
          Phone authentication is enabled for polkit-1. This affects all PolicyKit
          authentication dialogs. Ensure your phone is always available when using
          applications that require elevated privileges.
        '';

    # Ensure the PAM module is available system-wide
    # The actual .so file should be at: ${cfg.package}/lib/security/pam_cosmic_connect.so
    environment.systemPackages = [ cfg.package ];

    # Configure PAM services
    security.pam.services = genAttrs cfg.services (serviceName: {
      text = mkBefore ''
        # COSMIC Connect phone authentication
        # Request authentication from paired phone via D-Bus
        # [success=done] means if this succeeds, skip remaining auth modules
        # [default=ignore] means if this fails, continue to next auth module
        auth  [success=done default=ignore]  pam_cosmic_connect.so timeout=${toString cfg.timeout}
      '';

      # Ensure standard password authentication is available as fallback
      # This is critical for system stability
      rules = mkIf cfg.fallbackToPassword {
        auth = {
          unix = {
            control = "required";
            modulePath = "${pkgs.pam}/lib/security/pam_unix.so";
            settings = [ "try_first_pass" ];
          };
        };
      };
    });

    # Ensure D-Bus service is available for PAM module communication
    services.dbus.packages = [ cfg.package ];

    # Configure Polkit policy for phone authentication requests
    # This allows the PAM module to communicate with the daemon
    security.polkit.extraConfig = ''
      // COSMIC Connect Phone Authentication Policy
      // Allow local active sessions to request phone authentication
      polkit.addRule(function(action, subject) {
        if (action.id == "org.cosmicde.PhoneAuth.request" &&
            subject.local && subject.active) {
          return polkit.Result.YES;
        }
      });
    '';

    # Ensure the daemon is running for PAM authentication to work
    # The daemon must be available as a user service
    systemd.user.services.cosmic-connect-daemon = {
      # If the main module hasn't enabled the daemon, we need to ensure it exists
      # This is a runtime dependency for PAM authentication
      after = [ "network.target" ];
      wants = [ "network.target" ];

      # Add a condition to restart if it crashes during auth
      serviceConfig = {
        Restart = mkDefault "on-failure";
        RestartSec = mkDefault 3;
      };
    };

    # Configuration file for phone auth preferences
    # This is read by the PAM module to determine behavior
    environment.etc."xdg/cosmic-connect/phone-auth.toml".text = ''
      # COSMIC Connect Phone Authentication Configuration
      # Generated by NixOS module

      [auth]
      # Timeout for phone authentication requests (seconds)
      timeout = ${toString cfg.timeout}

      # Whether to fall back to password if phone auth fails
      fallback_to_password = ${if cfg.fallbackToPassword then "true" else "false"}

      # Enabled services
      services = [${concatMapStringsSep ", " (s: ''"${s}"'') cfg.services}]
    '';
  };

  meta = {
    maintainers = with maintainers; [ ]; # Add maintainer info
  };
}
