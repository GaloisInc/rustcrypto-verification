# AES

## Build

```sh
export SAW_RUST_LIBRARY_PATH="/path/to/mir-json/rlibs"
RUSTFLAGS="--cfg aes_force_soft" cargo saw-build
```

Make sure the `.linked-mir.json` path in the `linking <n> mir files into`
message from `cargo saw-build` matches the path given to the `mir_load_module`
command in `aes.saw`. If not you will have to update the path in `aes.saw`.

## Run proof

```sh
CRYPTOLPATH="../cryptol-specs" saw aes.saw
```
