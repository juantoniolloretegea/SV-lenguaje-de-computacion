#!/usr/bin/env bash
set -euo pipefail
[[ $# == 1 ]] || { echo 'Uso: autoprueba_paquete.sh DIRECTORIO_FUENTES' >&2; exit 2; }
source_dir=$(realpath "$1")
checker=$(realpath tests/row7_candidate/comprobar_paquete.sh)
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
bash "$checker" "$source_dir"
for id in PYTHON INESPERADO AUSENTE ALTERADO; do
  cp -a "$source_dir" "$scratch/$id"
  case "$id" in
    PYTHON) printf 'print(1)\n' > "$scratch/$id/invitado.py"; expected=PAQUETE_TIPO ;;
    INESPERADO) printf '// no declarado\n' > "$scratch/$id/rust/sv_core/src/invitado.rs"; expected=PAQUETE_INVENTARIO ;;
    AUSENTE) rm "$scratch/$id/rust/sv_core/src/bindings.rs"; expected=PAQUETE_AUSENTE ;;
    ALTERADO) printf '\n// alteración\n' >> "$scratch/$id/rust/sv_core/src/bindings.rs"; expected=PAQUETE_HUELLA ;;
  esac
  set +e
  result=$(bash "$checker" "$scratch/$id" 2>&1)
  status=$?
  set -e
  [[ $status == 1 && "$result" == "$expected" ]] || { printf 'Control %s incorrecto: retorno=%s; causa=%s\n' "$id" "$status" "$result" >&2; exit 1; }
  printf '%s: RECHAZADO por %s\n' "$id" "$result"
done
printf 'Controles de paquete: 4/4 rechazados por su causa; control íntegro admitido.\n'
