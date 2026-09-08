#!/usr/bin/env bash
# Mismo corte bajo dos máscaras de proceso y dos configuraciones de Git distintas.
set -euo pipefail
[[ $# == 1 ]] || { echo 'Uso: autoprueba_permisos.sh COMMIT' >&2; exit 2; }
cut=$(git rev-parse --verify "$1^{commit}")
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
for mask in 0000 0077; do
  (
    umask "$mask"
    export GIT_CONFIG_COUNT=1 GIT_CONFIG_KEY_0=tar.umask GIT_CONFIG_VALUE_0="$mask"
    bash tests/row7_candidate/empaquetar.sh "$cut" "$scratch/$mask"
  )
done
cmp "$scratch/0000/fila7-fuentes.tar.gz" "$scratch/0077/fila7-fuentes.tar.gz"
printf 'PERMISOS_INDEPENDIENTES: máscaras de proceso y Git 0000/0077; paquete idéntico.\n'
sha256sum "$scratch/0000/fila7-fuentes.tar.gz" | cut -d ' ' -f 1
