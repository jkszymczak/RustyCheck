{
  description = "Basic Flake for Rust dev";
  nixConfig.bash-prompt = "[Rust] -> ";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux.pkgs;
    in {
      devShells.x86_64-linux.default = pkgs.mkShell {
        name = "Rust";
        buildInputs = with pkgs;[
          cargo
          cargo-expand
          rustfmt
          rust-analyzer
          rustc
          plantuml
      ];
    };
  };
}
