# Rust FFI with OSXFUSE

Build steps:
* run `brew install autoconf automake libtool gettext`
* go to `src/osxfuse` and run `./makeconf.sh`;
* run `./configure` in `src/osxfuse`;
* back to `ffi_osxfuse` run `cargo build`.
