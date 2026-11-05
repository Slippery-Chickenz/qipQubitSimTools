{
	description = "Rust dev env";
	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
		utils.url = "github:numtide/flake-utils";
	};
	outputs = { self, nixpkgs, utils }:
		let
			system = "x86_64-linux";
			pkgs = import nixpkgs { inherit system; };
		in {
			devShells.${system}.default = pkgs.mkShell {
				packages = with pkgs; [
					pkg-config
					cargo
					gdb
					rustc
					rustfmt
					rust-analyzer
					openblasCompat
				];
				shellHook = ''
					exec fish
					'';
				buildInputs = with pkgs; [
					openssl
					hdf5
				];
				env = {
					OPENSSL_NO_VENDOR = "1";
				};
			};
		};

		# utils.lib.eachDefaultSystem (system:
		# 	let
		# 		pkgs = import nixpkgs { inherit system; };
		# 	in {
		# 		devShells.default = pkgs.mkShell {
		# 			nativeBuildInputs = with pkgs; [
		# 				pkg-config
		# 				cargo
		# 				gdb
		# 				rustc
		# 				rustfmt
		# 				rust-analyzer
		# 				openblasCompat
		# 			];
		#
		# 			buildInputs = with pkgs; [
		# 				openssl
		# 				hdf5
		# 			];
		# 			env = {
		# 				OPENSSL_NO_VENDOR = "1";
		# 			};
		# 		};
		# 	});
}
