# ES: Activación de la instalación autónoma SV; no requiere rustup.
# EN: Activate the standalone SV installation; rustup is not required.
export SV_RUST_PREFIX=/opt/sv-rust-1.98.0
if [ -x "$SV_RUST_PREFIX/bin/rustc" ] && [ -x "$SV_RUST_PREFIX/bin/cargo" ]; then
    case ":$PATH:" in
        *":$SV_RUST_PREFIX/bin:"*) ;;
        *) export PATH="$SV_RUST_PREFIX/bin:$PATH" ;;
    esac
fi
