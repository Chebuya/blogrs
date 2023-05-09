{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  name = "workers-blog";
  
  buildInputs = [
    nodejs
    nodePackages.wrangler
  ];
}
