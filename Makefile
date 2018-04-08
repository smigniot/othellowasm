all: othello_rust.wasm
	

othello_rust.wasm: src/wasm.rs
	rustc +nightly --target=wasm32-unknown-unknown --crate-type=cdylib src/wasm.rs -O -o othello_rust.wasm

clean:
	rm -f othello_rust.wasm
