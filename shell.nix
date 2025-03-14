{ sources ? import ./nix/sources.nix
, nixpkgs ? sources.nixpkgs
, pkgs ? import nixpkgs { }
}:

let
  bake = pkgs.callPackage (sources.image-tools + "/image-tools.nix") { };
in
pkgs.mkShell {
  packages = [
    bake
  ];

  buildInputs = [
    # Required by patchable
    pkgs.openssl
  ];

  nativeBuildInputs = [
    # Required for libraries to be discoverable
    pkgs.pkg-config
  ];

}
