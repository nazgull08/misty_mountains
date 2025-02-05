{ pkgs ? import <nixpkgs> {} }:

let
  nodePackages = pkgs.nodejs_20.pkgs;
in
pkgs.mkShell {
  buildInputs = [
    pkgs.nodejs_20  # Node.js (v20)
    nodePackages.pnpm  # Используем pnpm вместо npm/yarn (по желанию)
  ];

  shellHook = ''
    echo "Trading Gateway environment activated!"

    # Если нет node_modules, устанавливаем зависимости
    if [ ! -d "node_modules" ]; then
      echo "Installing dependencies with pnpm..."
      pnpm install
    fi
  '';
}
