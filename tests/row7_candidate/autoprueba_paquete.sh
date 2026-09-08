#!/usr/bin/env bash
set -euo pipefail
[[ $# == 1 ]] || { echo 'Uso: autoprueba_paquete.sh DIRECTORIO_FUENTES' >&2; exit 2; }
source_dir=$(realpath "$1")
checker=$(realpath tests/row7_candidate/comprobar_paquete.sh)
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
bash "$checker" "$source_dir"
for id in PYTHON INESPERADO AUSENTE ALTERADO FIFO FIFO_PERMITIDO FIFO_CORTE ENLACE; do
  cp -a "$source_dir" "$scratch/$id"
  case "$id" in
    PYTHON) printf 'print(1)\n' > "$scratch/$id/invitado.py"; expected=PAQUETE_TIPO ;;
    INESPERADO) printf '// no declarado\n' > "$scratch/$id/rust/sv_core/src/invitado.rs"; expected=PAQUETE_INVENTARIO ;;
    AUSENTE) rm "$scratch/$id/rust/sv_core/src/bindings.rs"; expected=PAQUETE_AUSENTE ;;
    ALTERADO) printf '\n// alteración\n' >> "$scratch/$id/rust/sv_core/src/bindings.rs"; expected=PAQUETE_HUELLA ;;
    FIFO) mkfifo "$scratch/$id/invitado.py"; expected=PAQUETE_TIPO_ESPECIAL ;;
    FIFO_PERMITIDO) mkfifo "$scratch/$id/rust/sv_core/src/invitado.rs"; expected=PAQUETE_TIPO_ESPECIAL ;;
    FIFO_CORTE) rm "$scratch/$id/CORTE_GIT.txt"; mkfifo "$scratch/$id/CORTE_GIT.txt"; expected=PAQUETE_TIPO_ESPECIAL ;;
    ENLACE) ln -s CORTE_GIT.txt "$scratch/$id/enlace"; expected=PAQUETE_ENLACE ;;
  esac
  set +e
  result=$(timeout 10s bash "$checker" "$scratch/$id" 2>&1)
  status=$?
  set -e
  [[ $status == 1 && "$result" == "$expected" ]] || { printf 'Control %s incorrecto: retorno=%s; causa=%s\n' "$id" "$status" "$result" >&2; exit 1; }
  printf '%s: RECHAZADO por %s\n' "$id" "$result"
done
printf 'Controles de paquete: 8/8 rechazados por su causa; control íntegro admitido.\n'
