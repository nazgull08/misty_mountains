{ pkgs ? import <nixpkgs> {} }:

let
  nodePackages = pkgs.nodejs_20.pkgs;
in
pkgs.mkShell {
  buildInputs = [
    pkgs.nodejs_20  
    nodePackages.pnpm  
  ];

  shellHook = ''
    echo "Trading Gateway environment activated!"

    if [ ! -d "node_modules" ]; then
      echo "Installing dependencies with pnpm..."
      pnpm install
    fi
  '';
}
