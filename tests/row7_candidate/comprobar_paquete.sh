#!/usr/bin/env bash
# Integridad e inventario del paquete. La autenticidad exige su ancla Git externa.
set -euo pipefail
fail() { printf '%s\n' "$1" >&2; exit 1; }
[[ $# == 1 ]] || { echo 'Uso: comprobar_paquete.sh DIRECTORIO_FUENTES' >&2; exit 2; }
cd "$1"
[[ -z $(find . -type l -print -quit) ]] || fail PAQUETE_ENLACE
[[ -z $(find . ! -type d ! -type f -print -quit) ]] || fail PAQUETE_TIPO_ESPECIAL
[[ -f INVENTARIO.sha256 && -f CORTE_GIT.txt ]] || fail PAQUETE_MANIFIESTO
[[ $(cat CORTE_GIT.txt) =~ ^[0-9a-f]{40}$ ]] || fail PAQUETE_CORTE
while IFS= read -r -d '' path; do
  case "$path" in
    ./LICENSE|./CORTE_GIT.txt|./INVENTARIO.sha256|./rust/*.toml|./rust/*.rs|./tests/*.svp|./tests/*.json|./tests/*.rs|./tests/*.sha256|./tests/row7_candidate/COMPILACION.md) ;;
    *) fail PAQUETE_TIPO ;;
  esac
done < <(find . -type f -print0)
declared=()
while IFS= read -r line; do
  [[ "$line" =~ ^[0-9a-f]{64}\ \ (\./.+)$ ]] || fail PAQUETE_MANIFIESTO
  path=${BASH_REMATCH[1]}
  [[ "$path" != *'/../'* && "$path" != *$'\t'* && "$path" != *'\'* && "$path" != './INVENTARIO.sha256' ]] || fail PAQUETE_RUTA
  [[ -f "$path" ]] || fail PAQUETE_AUSENTE
  declared+=("$path")
done < INVENTARIO.sha256
[[ ${#declared[@]} -gt 0 ]] || fail PAQUETE_VACIO
mapfile -d '' actual < <(find . -type f ! -path './INVENTARIO.sha256' -print0 | LC_ALL=C sort -z)
[[ ${#actual[@]} == ${#declared[@]} ]] || fail PAQUETE_INVENTARIO
for i in "${!actual[@]}"; do [[ "${actual[$i]}" == "${declared[$i]}" ]] || fail PAQUETE_INVENTARIO; done
sha256sum --check --status INVENTARIO.sha256 || fail PAQUETE_HUELLA
printf 'CONFORME_EN_INTEGRIDAD_DE_PAQUETE\n'
