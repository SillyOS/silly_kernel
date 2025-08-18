{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustup
    pkgs.qemu
    pkgs.gnumake
    pkgs.pkg-config
  ];

  shellHook = ''
    rustup default nightly
    echo "Nightly Rust environment activated"
  '';
}
