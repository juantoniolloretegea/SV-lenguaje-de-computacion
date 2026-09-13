#!/bin/sh
# ES: Prepara la referencia ya utilizada por CI; no usa el canal mutable stable.
# EN: Prepares the reference already used by CI; it does not use mutable stable.
set -eu
if ! command -v rustup >/dev/null 2>&1; then
    if [ -x "${CARGO_HOME:-$HOME/.cargo}/bin/rustup" ]; then
        PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"
        export PATH
    else
        printf '%s\n' 'Instale rustup desde https://rust-lang.org/tools/install/ y vuelva a ejecutar este script.' >&2
        exit 2
    fi
fi
rustup toolchain install 1.98.0 --profile minimal --no-self-update
rustup target add --toolchain 1.98.0 wasm32-wasip1 wasm32-unknown-unknown
rustup run 1.98.0 rustc --version --verbose
rustup run 1.98.0 cargo --version
printf '%s\n' 'Preparado. Para esta consola: . "${CARGO_HOME:-$HOME/.cargo}/env"'
