{
  sources ? import ./nix/sources.nix,
  nixpkgs ? sources.nixpkgs,
  pkgs ? import nixpkgs { },
}:

pkgs.mkShell {
  packages = with pkgs; [
    cargo
    rustc
    nodejs
    zizmor
  ];

  buildInputs = with pkgs; [
    # Required by patchable
    openssl
  ];

  nativeBuildInputs = with pkgs; [
    # Required for libraries to be discoverable
    pkg-config
  ];
}
