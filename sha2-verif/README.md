# SHA2

## Build

```sh
export SAW_RUST_LIBRARY_PATH="/path/to/mir-json/rlibs"
cargo saw-build
```

Make sure the `.linked-mir.json` path in the `linking <n> mir files into`
message from `cargo saw-build` matches the path given to the `mir_load_module`
command in `sha2.saw`. If not you will have to update the path in `sha2.saw`.

## Run proof

```sh
CRYPTOLPATH="../cryptol-specs" saw sha2.saw
```
