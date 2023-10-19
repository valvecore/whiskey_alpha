


make_compiled_folder:

	mkdir compiled 

cargo_compile:
	cd whiskey_cli && cargo build --release 

get_whiskey_out:
	cp whiskey_cli/target/release/whiskey ./compiled/whiskey

build: make_compiled_folder cargo_compile get_whiskey_out
