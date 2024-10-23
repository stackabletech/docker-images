{ sources ? import ./nix/sources.nix
, nixpkgs ? sources.nixpkgs
, pkgs ? import nixpkgs { }
}:

let
  bake = pkgs.callPackage (sources.image-tools + "/image-tools.nix") { };
in
pkgs.mkShell {
  packages = with pkgs; [
    bake
  ];
}
