{
  sources ? import ./nix/sources.nix,
  nixpkgs ? sources.nixpkgs,
  pkgs ? import nixpkgs { },
}:

pkgs.mkShell {
  packages = with pkgs; [
    rustup
    nodejs
    zizmor
    git-cliff
    python312
    uv
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
