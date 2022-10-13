{ pkgs }: {
	deps = [
		pkgs.lldb_14
        pkgs.rustc
		pkgs.rustfmt
		pkgs.cargo
		pkgs.cargo-edit
        pkgs.rust-analyzer
	];
}