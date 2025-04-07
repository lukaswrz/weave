{
  description = "weave";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    devenv-root = {
      url = "file+file:///dev/null";
      flake = false;
    };
    devenv.url = "github:cachix/devenv";
  };

  outputs = {
    self,
    nixpkgs,
    flake-parts,
    ...
  } @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.devenv.flakeModule
      ];

      systems = ["x86_64-linux" "aarch64-linux"];

      perSystem = {
        pkgs,
        inputs',
        lib,
        ...
      }: {
        devenv.shells.default = {
          devenv.root = let
            devenvRootFileContent = builtins.readFile inputs.devenv-root.outPath;
          in
            lib.mkIf (devenvRootFileContent != "") devenvRootFileContent;

          name = "weave";

          imports = [
            ./devenv.nix
          ];
        };

        packages = let
          packages = lib.packagesFromDirectoryRecursive {
            inherit (pkgs) callPackage;
            directory = ./packages;
          };
        in packages // {
          default = packages.weave;
        };
      };
    };
}
