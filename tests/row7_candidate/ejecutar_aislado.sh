#!/usr/bin/env bash
# Se ejecuta dentro del contenedor sin red, con sólo paquete, herramientas y salida.
set -euo pipefail
export PATH=/opt/rust/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
export CARGO_HOME=/tmp/sv-cargo
export CARGO_TARGET_DIR=/work/target
cd /work
for command in python python3 node nodejs; do
  if command -v "$command"; then echo 'INTERPRETE_PRESENTE' >&2; exit 1; fi
done
dpkg-query -W -f='${binary:Package}\t${Version}\n' > /out/paquetes-entorno.tsv
if cut -f1 /out/paquetes-entorno.tsv | grep -Eq '^(python|libpython|nodejs|libnode)([^a-z]|$)'; then
  echo 'PAQUETE_INTERPRETE_PRESENTE' >&2; exit 1
fi
rustc --version | tee /out/rustc.txt
cargo --version | tee /out/cargo.txt
[[ $(rustc --version) == 'rustc 1.98.0 '* && $(cargo --version) == 'cargo 1.98.0 '* ]] || exit 1
gcc --version > /out/gcc.txt
sha256sum --check --status INVENTARIO.sha256
printf 'INTERPRETES_PYTHON_NODE_AUSENTES\n'
cargo build --manifest-path rust/Cargo.toml --workspace --release --offline
cargo test --manifest-path rust/Cargo.toml --workspace --offline
rustc --edition=2021 tests/row7_gh/generar_entradas.rs -o /work/target/gh-generate
/work/target/gh-generate --check
cargo run --manifest-path rust/Cargo.toml -p sv_core --example binding_probe --release --offline > /out/lig.json
cargo run --manifest-path rust/Cargo.toml -p sv_core --example gh_binding_probe --release --offline > /out/gh.json
/work/target/release/sv-native --profile en tests/row7_bindings/base.svp > /out/cli-en.json
/work/target/release/sv-native --profile es tests/row7_bindings/base.es.svp > /out/cli-es.json
[[ -s /out/cli-en.json && -s /out/cli-es.json ]]
set +e
/work/target/release/sv-native tests/conformance/invalid/transition_induced_position_cero.svp > /out/rechazo-ir.json 2> /out/rechazo.txt
status=$?
set -e
[[ $status == 1 && ! -s /out/rechazo-ir.json ]]
grep -Fq 'posición 0 fuera de [1, 9] para el nodo CC' /out/rechazo.txt
sha256sum --check --status INVENTARIO.sha256
cp /work/target/release/sv-native /out/sv-native
sha256sum /out/sv-native /out/lig.json /out/gh.json > /out/salidas.sha256
printf 'CONSTRUCCION_Y_PRUEBAS_RUST_SIN_INTERPRETES_CONFORMES\n'
