#!/bin/sh

WASM="./rs-wat2imports.wasm"

input_wasm() {
	cat "./rs-wat2imports.wasm"
}

input_wat() {
	echo '(module
      (type (;0;) (func (param i32)))
      (import "wasi_snapshot_preview1" "proc_exit" (func (type 0)))
    )'
}

input_wasm | wasmtime run "${WASM}" --input-size-limit 16777216 | jq -c
input_wat  | wasmtime run "${WASM}" --input-size-limit     1024 | jq -c
