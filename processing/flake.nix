{
  description = "My flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }: (utils.lib.eachSystem ["aarch64-darwin" ] (system: rec {

    packages = {
      pythonEnv = nixpkgs.legacyPackages.${system}.python3.withPackages(ps: with ps; [ numpy pandas ]);
    };

    defaultPackage = packages.pythonEnv; # If you want to juist build the environment
    devShell = packages.pythonEnv.env; # We need .env in order to use `nix develop`
  }));
}
