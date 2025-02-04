{
  inputs = {
    nixpkgs-stable.url = "github:nixos/nixpkgs/nixos-24.11";
  };
  outputs = {
    self,
    nixpkgs-stable,
  }: let
    stable = nixpkgs-stable.legacyPackages.x86_64-linux;
  in {
    devShells.x86_64-linux.default = stable.mkShell {
      packages = with stable; [cargo rustc];
    };
  };
}
