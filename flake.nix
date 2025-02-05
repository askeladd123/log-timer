{
  inputs = {
    nixpkgs-stable.url = "github:nixos/nixpkgs/nixos-24.11";
  };
  outputs = {
    self,
    nixpkgs-stable,
  }: let
    system = "x86_64-linux";
    stable = nixpkgs-stable.legacyPackages.${system};
  in {
    packages.${system}.default = stable.rustPlatform.buildRustPackage {
      pname = "log-timer";
      version = "0.3.2";
      src = ./.;
      cargoLock.lockFile = ./Cargo.lock;
    };
  };
}
