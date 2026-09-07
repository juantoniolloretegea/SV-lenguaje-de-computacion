#!/usr/bin/env python3
"""Conformidad causal acotada para el cierre correctivo de la fila 7."""

from __future__ import annotations

import argparse
from pathlib import Path
import sys

from oracle_support import assert_rejection, run

ROOT = Path(__file__).resolve().parents[1]
CASES_DIR = ROOT / "tests" / "conformance" / "invalid"

EXPECTED = {
    "compose_graph_tipo_incorrecto.svp": "K: se esperaba CompositionGraph",
    "frame_architecture_tipo_incorrecto.svp": "K: se esperaba CompositionGraph",
    "transition_event_tipo_repetido.svp": (
        "E406 (InsufficientTransitionData): TransitionData T: "
        "tipo de suceso repetido: Cambio"
    ),
    "transition_induced_node_ausente.svp": (
        "E406 (InsufficientTransitionData): TransitionData T: "
        "referencia de nodo no declarada: Fantasma"
    ),
    "transition_induced_node_fuera_arquitectura.svp": (
        "E406 (InsufficientTransitionData): TransitionData T: "
        "nodo CCFuera ajeno a la arquitectura G"
    ),
    "transition_induced_node_tipo_incorrecto.svp": (
        "E406 (InsufficientTransitionData): TransitionData T: "
        "C: se esperaba CoupledSpec"
    ),
    "transition_induced_destino_repetido.svp": (
        "E406 (InsufficientTransitionData): TransitionData T: "
        "destino inducido repetido: (CC, 1)"
    ),
    "transition_induced_position_cero.svp": (
        "E406 (InsufficientTransitionData): TransitionData T: "
        "posición 0 fuera de [1, 9] para el nodo CC"
    ),
    "transition_induced_position_fuera_rango.svp": (
        "E406 (InsufficientTransitionData): TransitionData T: "
        "posición 10 fuera de [1, 9] para el nodo CC"
    ),
}


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--rust-bin",
        default=str(ROOT / "rust" / "target" / "debug" / "sv-native"),
    )
    args = parser.parse_args()

    missing = sorted(name for name in EXPECTED if not (CASES_DIR / name).is_file())
    if missing:
        print(f"faltan casos causales compartidos: {missing}", file=sys.stderr)
        return 1

    failures: list[str] = []
    for name, token in sorted(EXPECTED.items()):
        proc = run([args.rust_bin, str(CASES_DIR / name)])
        try:
            assert_rejection(proc, token)
        except AssertionError as exc:
            failures.append(f"{name}: {exc}")
        else:
            print(f"Fila 7 causal OK: {name}")

    if failures:
        print("\n".join(failures), file=sys.stderr)
        return 1

    print(f"Conformidad causal de fila 7: {len(EXPECTED)}/{len(EXPECTED)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
