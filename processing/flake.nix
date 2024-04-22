{
  description = "My flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }: (utils.lib.eachSystem ["aarch64-darwin" ] (system: rec {

    packages = {
      pythonEnv = nixpkgs.legacyPackages.${system}.python312.withPackages(ps: with ps; [ 
        face-recognition
        pika
        requests
      ]);
    };

    defaultPackage = packages.pythonEnv; # If you want to just build the environment
    devShell = packages.pythonEnv.env.overrideAttrs(oldAttrs: {
      shellHook = ''
        echo "Lessgo"
      '';
    });
  }));
}
