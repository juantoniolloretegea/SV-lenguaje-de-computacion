#!/usr/bin/env python3
"""Busqueda de ambiguedad acotada.

Genera cadenas con un vocabulario REDUCIDO (un representante por clase lexica,
conservando intacta toda la estructura G01-G23) y las cuenta con el analizador
COMPLETO. Toda ambiguedad hallada es real; su ausencia queda acotada al
vocabulario reducido y a la longitud explorada.
"""
import sys, copy
import p2_parser as P

LMAX = int(sys.argv[1]) if len(sys.argv) > 1 else 8

FULL = P.G
RED = copy.deepcopy(FULL)
def t(*ws): return [("t", w) for w in ws]
RED["Campo"] = [t("el", "valor"), t("valor"), t("la", "unidad"), t("el", "dato")]
RED["AtomoParametro"] = [t("igg"), t("la", "igg"), t("iga")]
RED["Temporal"] = [t("actual"), t("la", "actual")]
RED["Momento"] = [t("actual")]
RED["Orden"] = [t("consulte"), t("sólo", "consulte"), t("elimine")]
RED["ObjetoExplicito"] = [t("caso-a")]
RED["ObjetoContextual"] = [t("este", "caso"), t("el", "caso", "que", "estamos", "viendo")]
RED["Cabeza"] = [[("n", "Campo")], [("n", "Parametro")], [("n", "Temporal")],
                 t("el", "registro")]
RED["SujCab"] = [[("n", "Parametro")], t("el", "registro")]
RED["Cortesia"] = [t("por", "favor"), t(",", "por", "favor")]

memo = {}
def gen(sym, n):
    if n <= 0: return frozenset()
    key = (sym, n)
    if key in memo: return memo[key]
    memo[key] = frozenset()
    if sym == "Entero":
        r = frozenset({("7",)}) if n == 1 else frozenset()
        memo[key] = r; return r
    out = set()
    for alt in RED[sym]:
        out |= gen_seq(tuple(alt), n)
    r = frozenset(out); memo[key] = r; return r

seqmemo = {}
def gen_seq(items, n):
    if not items:
        return frozenset({()}) if n == 0 else frozenset()
    key = (items, n)
    if key in seqmemo: return seqmemo[key]
    seqmemo[key] = frozenset()
    kind, val = items[0]; rest = items[1:]
    res = set()
    if kind == "t":
        if n >= 1:
            for r in gen_seq(rest, n - 1):
                res.add((val,) + r)
    else:
        for k in range(1, n + 1):
            hs = gen(val, k)
            if not hs: continue
            ts = gen_seq(rest, n - k)
            if not ts: continue
            for h in hs:
                for tl in ts:
                    res.add(h + tl)
    r = frozenset(res); seqmemo[key] = r; return r

# punto fijo por profundidad de recursion (Comps)
todas = set()
for it in range(LMAX + 1):
    memo.clear(); seqmemo.clear()
    nuevas = set()
    for n in range(1, LMAX + 1):
        nuevas |= gen("Peticion", n)
    if nuevas <= todas: break
    todas |= nuevas

print("cadenas generadas (vocabulario reducido, long <= %d): %d" % (LMAX, len(todas)))
amb = []
for s in todas:
    p = P.make_parser(tuple(s))
    c = p("Peticion", 0, len(s))
    if c != 1:
        amb.append((c, s))
print("con derivaciones != 1: %d" % len(amb))
for c, s in sorted(amb, key=lambda x: (-x[0], len(x[1])))[:40]:
    print("  n=%s  %s" % (c, " ".join(s)))
