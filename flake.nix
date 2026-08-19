{
	description = "Rust dev env";

	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
		utils.url = "github:numtide/flake-utils";
	};

	outputs = { self, nixpkgs, utils }:
		utils.lib.eachDefaultSystem (system:
			let
				pkgs = import nixpkgs { inherit system; };
			in {
				devShells.default = pkgs.mkShell {
					nativeBuildInputs = with pkgs; [
						pkg-config
						cargo
						gdb
						rustc
						rust-analyzer
						openblasCompat
					];

					buildInputs = with pkgs; [
						openssl
						hdf5
					];
					env = {
						OPENSSL_NO_VENDOR = "1";
					};
				};
			});
}
