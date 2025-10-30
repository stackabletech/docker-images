{
  sources ? import ./nix/sources.nix,
  nixpkgs ? sources.nixpkgs,
  pkgs ? import nixpkgs { },
}:

pkgs.mkShell {
  packages = [
    pkgs.cargo
    pkgs.rustc
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
