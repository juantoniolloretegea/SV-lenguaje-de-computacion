#!/usr/bin/env python3
"""Sensibilidad dirigida de los oráculos nativos del cierre correctivo.

Cada mutación se aplica por separado, debe compilar y debe hacer fallar la
prueba asignada. Los ficheros se restauran por bytes y se comprueban mediante
SHA-256. La muestra es deliberadamente finita y no expresa cobertura exhaustiva.
"""

from __future__ import annotations

from dataclasses import dataclass
import argparse
import difflib
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
CARGO = ["cargo", "test", "--manifest-path", "rust/Cargo.toml", "-p", "sv_core"]


@dataclass(frozen=True)
class Mutant:
    identifier: str
    purpose: str
    path: str
    old: str
    new: str
    test_binary: str
    test_name: str

    @property
    def command(self) -> list[str]:
        return CARGO + ["--test", self.test_binary, self.test_name, "--", "--exact"]


MUTANTS = (
    Mutant(
        "M01", "relajar b mínimo de 3 a 2", "rust/sv_core/src/wellformed.rs",
        'if nat_cmp_text(b, "3") == Ordering::Less {',
        'if nat_cmp_text(b, "2") == Ordering::Less {',
        "cell_geometry_native", "b_menor_que_tres_se_rechaza_nativamente_en_ambos_perfiles",
    ),
    Mutant(
        "M02", "neutralizar la longitud n=b² de CellState", "rust/sv_core/src/wellformed.rs",
        "if !nat_eq_usize(n, vector.len()) {", "if false {",
        "cell_geometry_native", "longitud_de_vector_distinta_de_n_se_rechaza_nativamente",
    ),
    Mutant(
        "M08b", "neutralizar el tipo de Frame.architecture", "rust/sv_core/src/wellformed.rs",
        'let graph = expect_object(symbols, architecture, "CompositionGraph", |k| matches!(k, IrObjectKind::CompositionGraph { .. }))?;',
        'let graph = expect_object(symbols, architecture, "CompositionGraph", |_| true)?;',
        "architecture_type_guards", "frame_architecture_exige_composition_graph_en_ambos_perfiles",
    ),
    Mutant(
        "M08c", "neutralizar el tipo de compose(graph)", "rust/sv_core/src/wellformed.rs",
        'expect_object(symbols, graph, "CompositionGraph", |k| matches!(k, IrObjectKind::CompositionGraph { .. }))?;',
        'expect_object(symbols, graph, "CompositionGraph", |_| true)?;',
        "architecture_type_guards", "compose_graph_exige_composition_graph_en_ambos_perfiles",
    ),
    Mutant(
        "P01", "emitir source_sha256 constante", "rust/sv_core/src/equivalence.rs",
        "js(program.source_sha256()),", 'js("constant"),',
        "projection_metadata_native", "bytes_fuente_distintos_modifican_la_huella_y_su_emision",
    ),
    Mutant(
        "P02", "emitir source_file constante", "rust/sv_core/src/equivalence.rs",
        "js(program.source_file()),", 'js("constant.svp"),',
        "projection_metadata_native", "source_file_modifica_la_proyeccion_sin_alterar_la_huella_del_contenido",
    ),
    Mutant(
        "P03", "emitir serializer_version constante", "rust/sv_core/src/equivalence.rs",
        "js(program.serializer_version()),", 'js("constant"),',
        "projection_metadata_native", "proyeccion_emite_una_vez_version_fichero_y_huella_reales",
    ),
    Mutant(
        "TD01", "neutralizar la unicidad local de sucesos", "rust/sv_core/src/transition_data_wellformed.rs",
        "if !event_types.insert(event_type.as_str()) {",
        "if !event_types.insert(event_type.as_str()) && false {",
        "transition_data_wellformed", "transition_data_rechaza_tipo_de_suceso_repetido_con_valor_igual_o_distinto",
    ),
    Mutant(
        "TD02", "admitir CellSpec donde se exige identidad de nodo", "rust/sv_core/src/transition_data_wellformed.rs",
        "Some(IrObjectKind::CoupledSpec { cell, .. }) => cell,\n                Some(_) => {",
        "Some(IrObjectKind::CoupledSpec { cell, .. }) => cell,\n                Some(IrObjectKind::CellSpec { .. }) => node_ref,\n                Some(_) => {",
        "transition_data_wellformed", "transition_data_rechaza_nodo_ausente_tipo_incorrecto_o_fuera_de_arquitectura",
    ),
    Mutant(
        "TD03", "neutralizar la pertenencia del nodo al grafo del horizonte", "rust/sv_core/src/transition_data_wellformed.rs",
        "if !graph_nodes.iter().any(|declared| declared == node_ref) {",
        "if false && !graph_nodes.iter().any(|declared| declared == node_ref) {",
        "transition_data_wellformed", "transition_data_rechaza_nodo_ausente_tipo_incorrecto_o_fuera_de_arquitectura",
    ),
    Mutant(
        "TD04", "neutralizar el rango posicional", "rust/sv_core/src/transition_data_wellformed.rs",
        'if position.as_decimal() == "0"\n                || decimal_cmp(position.as_decimal(), n.as_decimal()) == Ordering::Greater\n            {',
        "if false {",
        "transition_data_wellformed", "transition_data_rechaza_cero_y_posicion_superior_a_n_sin_estrechar_nat",
    ),
    Mutant(
        "TD05", "neutralizar la unicidad del destino inducido", "rust/sv_core/src/transition_data_wellformed.rs",
        "if !induced_targets.insert((node_ref.as_str(), position.as_decimal())) {",
        "if !induced_targets.insert((node_ref.as_str(), position.as_decimal())) && false {",
        "transition_data_wellformed", "transition_data_rechaza_destino_inducido_repetido_con_valor_igual_o_distinto",
    ),
)

CONTROL_TESTS = (
    "architecture_type_guards",
    "cell_geometry_native",
    "projection_metadata_native",
    "transition_data_wellformed",
)


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def run(command: list[str], *, timeout: int, env: dict[str, str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        command,
        cwd=ROOT,
        env=env,
        text=True,
        encoding="utf-8",
        errors="replace",
        capture_output=True,
        check=False,
        timeout=timeout,
    )


def tool_version(command: list[str]) -> str:
    proc = subprocess.run(command, cwd=ROOT, text=True, capture_output=True, check=False)
    return (proc.stdout or proc.stderr).strip()


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=ROOT / "artifacts" / "directed-mutation-sensitivity",
    )
    parser.add_argument("--timeout", type=int, default=180)
    args = parser.parse_args()

    output_dir = args.output_dir if args.output_dir.is_absolute() else ROOT / args.output_dir
    output_dir.mkdir(parents=True, exist_ok=True)

    env = os.environ.copy()
    env["CARGO_TARGET_DIR"] = str(ROOT / "rust" / "target" / "directed-mutation-sensitivity")

    control_command = CARGO.copy()
    for test in CONTROL_TESTS:
        control_command.extend(["--test", test])
    control = run(control_command, timeout=args.timeout, env=env)
    (output_dir / "control.stdout.txt").write_text(control.stdout, encoding="utf-8")
    (output_dir / "control.stderr.txt").write_text(control.stderr, encoding="utf-8")
    if control.returncode != 0:
        print("el control previo no es conforme", file=sys.stderr)
        return 1

    touched = sorted({mutant.path for mutant in MUTANTS})
    baseline = {path: (ROOT / path).read_bytes() for path in touched}
    results: list[dict[str, object]] = []

    for mutant in MUTANTS:
        path = ROOT / mutant.path
        original_bytes = baseline[mutant.path]
        original = original_bytes.decode("utf-8")
        count = original.count(mutant.old)
        if count != 1:
            results.append({
                "id": mutant.identifier,
                "disposition": "MUTACION_INVALIDA",
                "reason": f"patrón encontrado {count} veces",
            })
            print(f"{mutant.identifier}: MUTACION_INVALIDA", file=sys.stderr)
            continue

        changed = original.replace(mutant.old, mutant.new, 1)
        try:
            path.write_text(changed, encoding="utf-8", newline="")
            patch = "".join(difflib.unified_diff(
                original.splitlines(keepends=True),
                changed.splitlines(keepends=True),
                fromfile=f"a/{mutant.path}",
                tofile=f"b/{mutant.path}",
            ))
            (output_dir / f"{mutant.identifier}.patch").write_text(patch, encoding="utf-8")

            proc = run(mutant.command, timeout=args.timeout, env=env)
            (output_dir / f"{mutant.identifier}.stdout.txt").write_text(proc.stdout, encoding="utf-8")
            (output_dir / f"{mutant.identifier}.stderr.txt").write_text(proc.stderr, encoding="utf-8")
            combined = proc.stdout + "\n" + proc.stderr
            compile_error = "could not compile" in combined or "error[E" in combined or "error: expected" in combined
            if proc.returncode == 0:
                disposition = "SOBREVIVE"
            elif compile_error or "test result: FAILED" not in combined:
                disposition = "MUTACION_INVALIDA"
            else:
                disposition = "MUERTA"

            results.append({
                "id": mutant.identifier,
                "purpose": mutant.purpose,
                "path": mutant.path,
                "test_binary": mutant.test_binary,
                "test_name": mutant.test_name,
                "command": mutant.command,
                "returncode": proc.returncode,
                "disposition": disposition,
                "baseline_sha256": sha256(original_bytes),
                "mutated_sha256": sha256(changed.encode("utf-8")),
            })
            print(f"{mutant.identifier}: {disposition}")
        except subprocess.TimeoutExpired:
            results.append({"id": mutant.identifier, "disposition": "TIEMPO_EXCEDIDO"})
            print(f"{mutant.identifier}: TIEMPO_EXCEDIDO", file=sys.stderr)
        finally:
            path.write_bytes(original_bytes)
            if path.read_bytes() != original_bytes:
                raise RuntimeError(f"restauración no íntegra de {mutant.path}")

    restored = {path: sha256((ROOT / path).read_bytes()) == sha256(data) for path, data in baseline.items()}
    killed = sum(item.get("disposition") == "MUERTA" for item in results)
    survived = sum(item.get("disposition") == "SOBREVIVE" for item in results)
    invalid = len(results) - killed - survived

    summary = {
        "schema": "sv-directed-mutation-sensitivity/2",
        "github_sha": os.environ.get("GITHUB_SHA"),
        "rustc": tool_version(["rustc", "--version"]),
        "cargo": tool_version(["cargo", "--version"]),
        "control_returncode": control.returncode,
        "mutants_total": len(MUTANTS),
        "mutants_killed": killed,
        "mutants_survived": survived,
        "mutants_invalid": invalid,
        "restored": restored,
        "results": results,
    }
    (output_dir / "summary.json").write_text(
        json.dumps(summary, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )

    print(
        f"Mutaciones dirigidas: {killed}/{len(MUTANTS)} muertas; "
        f"{survived} supervivientes; {invalid} inválidas"
    )
    return 0 if killed == len(MUTANTS) and all(restored.values()) else 1


if __name__ == "__main__":
    raise SystemExit(main())
