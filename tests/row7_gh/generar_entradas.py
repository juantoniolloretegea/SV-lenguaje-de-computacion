"""Entradas del transporte desde el testigo recibido; nunca lee resultados Rust."""
from pathlib import Path
import hashlib
import json
import sys

ROOT = Path(__file__).parent


def compact(value):
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"))


def literal(text):
    assert '"###' not in text
    return 'r###"' + text + '"###'


def generate():
    source = json.loads((ROOT / "testigos-gh.json").read_bytes())
    lines = ["// Generado exclusivamente desde el testigo G/H: generar_entradas.py --check.",
             "pub const INPUTS: &[Input] = &["]
    for witness in source["witnesses"]:
        operation = compact(witness["operation"])
        for state in witness["domain"]["states"]:
            packet = state["packet"]
            reduced = {key: value for key, value in packet.items() if key != "detail"}
            for variant in ("F0", "H", "HS"):
                payload = compact(packet if variant == "F0" else reduced)
                side = compact({"detail": packet["detail"]}) if variant == "HS" else ""
                values = dict(witness=witness["id"], state=state["id"], variant=variant,
                              operation=witness["operation"]["id"], definition=operation,
                              payload=payload, side=side)
                for key in ("definition", "payload", "side"):
                    values[key + "_sha"] = hashlib.sha256(values[key].encode()).hexdigest()
                lines.append("    Input { " + ", ".join(key + ": " + literal(value)
                                                       for key, value in values.items()) + " },")
    return "\n".join(lines + ["];", ""])


if __name__ == "__main__":
    text = generate()
    target = ROOT / "entradas.rs"
    if sys.argv[1:] == ["--check"]:
        assert target.read_text() == text, "Entradas divergentes del testigo fuente"
        print("GH-LIG: entradas regenerables desde el testigo G/H exacto")
    elif not sys.argv[1:]:
        target.write_text(text)
    else:
        raise SystemExit("Uso: generar_entradas.py [--check]")
