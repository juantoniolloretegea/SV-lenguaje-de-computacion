#!/usr/bin/env bash
# RETP-102: toma archivos del commit exacto, nunca del directorio de trabajo.
set -euo pipefail
# Fija tanto los metadatos generados como los permisos extraídos por git archive.
umask 0022
[[ $# == 2 ]] || { echo 'Uso: empaquetar.sh COMMIT DIRECTORIO_NUEVO' >&2; exit 2; }
cut=$(git rev-parse --verify "$1^{commit}")
destination=$2
[[ ! -e "$destination" ]] || { echo 'PAQUETE_DIRECTORIO_EXISTENTE' >&2; exit 1; }
mkdir -p "$destination/fuentes"
files=()
while IFS= read -r -d '' path; do
  case "$path" in
    LICENSE|rust/*.toml|rust/*.rs|tests/*.svp|tests/*.json|tests/*.rs|tests/*.sha256|tests/row7_candidate/COMPILACION.md|tests/retorno_cyb/reglas_documentales.mjs|tests/retorno_cyb/consumir.mjs)
      [[ "$path" != *$'\n'* && "$path" != *$'\t'* && "$path" != *'\'* ]] || { echo 'PAQUETE_RUTA' >&2; exit 1; }
      files+=("$path") ;;
  esac
done < <(git ls-tree -r -z --name-only "$cut")
[[ ${#files[@]} -gt 0 ]] || { echo 'PAQUETE_VACIO' >&2; exit 1; }
git -c tar.umask=0022 archive "$cut" -- "${files[@]}" | tar -x -C "$destination/fuentes"
printf '%s\n' "$cut" > "$destination/fuentes/CORTE_GIT.txt"
(
  cd "$destination/fuentes"
  mapfile -d '' paths < <(find . -type f ! -path './INVENTARIO.sha256' -print0 | LC_ALL=C sort -z)
  sha256sum -- "${paths[@]}" > INVENTARIO.sha256
)
bash tests/row7_candidate/comprobar_paquete.sh "$destination/fuentes"
tar --sort=name --mtime=@0 --owner=0 --group=0 --numeric-owner -C "$destination" -cf - fuentes | gzip -n > "$destination/fila7-fuentes.tar.gz"
(
  cd "$destination"
  sha256sum fila7-fuentes.tar.gz > fila7-fuentes.tar.gz.sha256
)
printf 'Paquete del corte %s; %s archivos Git y dos objetos de inventario.\n' "$cut" "${#files[@]}"
