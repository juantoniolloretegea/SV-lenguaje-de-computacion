#!/usr/bin/env python3
# Implementacion literal del lexico (§3) y la gramatica G01-G23 (§4) del
# PERFIL_INTERACCION_ES_IE004_CANDIDATO_1.md, para medir cobertura y ambiguedad.
# NO implementa la semantica §5. Solo cuenta derivaciones sintacticas.
import json, sys, unicodedata
from functools import lru_cache

# ---------- §3 lexico ----------
CONV = {}
for a, b in zip("ABCDEFGHIJKLMNOPQRSTUVWXYZ", "abcdefghijklmnopqrstuvwxyz"):
    CONV[a] = b
for a, b in zip("ÁÉÍÓÚÜÑ", "áéíóúüñ"):
    CONV[a] = b

SEPS = {" ", "\t", "\n", "\r"}
SYMS = {"¿", "?", ".", ",", ":", ";"}

def convertir(s):
    return "".join(CONV.get(c, c) for c in s)

def lex(texto):
    """Devuelve (tokens_no_separadores, tokens_todos). Los puntos: '...' -> elipsis."""
    s = convertir(texto)
    out = []
    i = 0
    n = len(s)
    while i < n:
        c = s[i]
        if c in SEPS:
            out.append(("SEP", c)); i += 1; continue
        if c == ".":
            if s[i:i+3] == "...":
                out.append(("SYM", "...")); i += 3; continue
            out.append(("SYM", ".")); i += 1; continue
        if c in SYMS:
            out.append(("SYM", c)); i += 1; continue
        j = i
        while j < n and s[j] not in SEPS and s[j] not in SYMS and s[j] != ".":
            j += 1
        out.append(("W", s[i:j])); i = j
    toks = [v for (k, v) in out if k != "SEP"]
    return toks, out

# ---------- §4 gramatica, con opcionales expandidos ----------
def T(*words):
    return [("t", w) for w in words]

G = {}
def R(nt, *alts):
    G[nt] = list(alts)

# G22 Art = ["el"|"la"]  -> expandido en cada uso (Art0 = vacio)
ART = [[], T("el"), T("la")]

def with_art(tail_alts):
    res = []
    for a in ART:
        for t in tail_alts:
            res.append(a + t)
    return res

# G17 Momento
R("Momento", T("actual"), T("ahora"), T("anterior"), T("previo"))
# G12 Temporal = Art (actual|ahora|anterior|previo)
R("Temporal", *with_art([T("actual"), T("ahora"), T("anterior"), T("previo")]))
# G09 Campo
campo_tails = [T("valor"), T("dato"), T("cifra"), T("cifra", "registrada"),
               T("unidad"), T("estado"), T("estado", "registrado"),
               T("fuente"), T("procedencia"), T("límites"), T("alcance"),
               T("intervalo", "de", "referencia")]
R("Campo", *with_art(campo_tails))
# G11 AtomoParametro = ["el"|"la"|"esa"] (...)
ap_dets = [[], T("el"), T("la"), T("esa")]
ap_tails = [T("igg"), T("iga"), T("igm"), T("inmunoglobulina"),
            T("inmunoglobina"), T("linmunoglobina"),
            T("hemoglobina"), T("hemoglobina", "glicosilada")]
R("AtomoParametro", *[d + t for d in ap_dets for t in ap_tails])
# G10 Parametro
R("Parametro", [("n", "AtomoParametro")],
   [("n", "AtomoParametro"), ("t", "o"), ("n", "AtomoParametro")])
# G15 / G16 / G14
R("ObjetoExplicito", T("caso-a"), T("caso-b"))
R("ObjetoContextual", T("este", "caso"), T("ese", "caso"),
   T("este", "paciente"), T("ese", "paciente"),
   T("el", "caso", "que", "estamos", "viendo"),
   [("t", "este"), ("t", "...")])
R("Objeto", [("n", "ObjetoExplicito")], [("n", "ObjetoContextual")])
# G13 Complemento
R("Complemento",
  [("n", "Temporal")],
  [("t", "de"), ("n", "Parametro")],
  [("t", "para"), ("n", "Parametro")],
  [("t", "de"), ("n", "Objeto")],
  [("t", "para"), ("n", "Objeto")],
  [("t", "del"), ("n", "ObjetoExplicito")],
  [("t", "de"), ("n", "Momento")],
  [("t", "de"), ("t", "corte"), ("n", "Momento")],
  [("t", "del"), ("n", "Momento")],
  [("t", "del"), ("t", "corte"), ("n", "Momento")],
  [("t", "en"), ("t", "el"), ("t", "corte"), ("n", "Momento")],
  [("t", "del"), ("t", "dato")],
  [("t", "del"), ("t", "registro")],
)
# G07 Cabeza = Campo | Parametro | Temporal | Art "registro"
R("Cabeza", [("n", "Campo")], [("n", "Parametro")], [("n", "Temporal")],
   *with_art([T("registro")]))
# G06 Grupo = Cabeza {Complemento}
R("Comps", [("n", "Complemento")], [("n", "Complemento"), ("n", "Comps")])
R("Grupo", [("n", "Cabeza")], [("n", "Cabeza"), ("n", "Comps")])
# G08 Sujeto = (Parametro | Art "registro") {Complemento}
R("SujCab", [("n", "Parametro")], *with_art([T("registro")]))
R("Sujeto", [("n", "SujCab")], [("n", "SujCab"), ("n", "Comps")])
# G05 Orden = ["sólo"|"solo"] (verbo)
verbos = ["consulte", "lea", "traiga", "dígame", "necesito", "cambie", "elimine"]
R("Orden", *([T(v) for v in verbos] +
             [T(s, v) for s in ("sólo", "solo") for v in verbos]))
# G04 Nucleo
R("Nucleo",
  [("n", "Orden"), ("n", "Grupo")],
  [("t", "escriba"), ("n", "Entero"), ("t", "en"), ("n", "Grupo")],
  [("n", "Grupo")],
  [("t", "cuál"), ("t", "es"), ("n", "Grupo")],
  [("t", "adónde"), ("t", "estará"), ("n", "Grupo")],
  [("t", "y"), ("n", "Grupo")],
  [("t", "qué"), ("n", "Campo"), ("t", "tiene"), ("n", "Sujeto")],
  [("t", "en"), ("t", "qué"), ("t", "unidad"), ("t", "está"), ("t", "expresada"), ("n", "Sujeto")],
  [("t", "en"), ("t", "qué"), ("t", "unidad"), ("t", "figura"), ("n", "Sujeto")],
)
# G20 Motivo / G21 Cortesia
R("Motivo", [("t", "porque"), ("t", "estoy"), ("t", "revisando"), ("n", "Objeto")])
R("Cortesia", T("por", "favor"), [("t", ","), ("t", "por"), ("t", "favor")])
# G03 Consulta = Nucleo [Motivo] [Cortesia]
R("Consulta",
  [("n", "Nucleo")],
  [("n", "Nucleo"), ("n", "Motivo")],
  [("n", "Nucleo"), ("n", "Cortesia")],
  [("n", "Nucleo"), ("n", "Motivo"), ("n", "Cortesia")])
# G18 Negacion
R("Negacion",
  [("t", "no"), ("n", "Orden"), ("n", "Grupo")],
  [("t", "no"), ("t", "quiero"), ("n", "Grupo")],
  [("t", "no"), ("n", "Grupo")],
  T("no", "cambie", "nada"))
# G19 RecorteObjeto
R("RecorteObjeto", [("t", "del"), ("n", "ObjetoExplicito")], [("t", "de"), ("n", "Objeto")])
# G02 Cuerpo
R("Cuerpo",
  [("n", "Consulta")],
  [("n", "Negacion"), ("t", ";"), ("n", "Consulta")],
  [("n", "Negacion"), ("t", ":"), ("n", "Consulta")],
  [("n", "Consulta"), ("t", ","), ("n", "Negacion")],
  [("n", "Consulta"), ("t", ";"), ("n", "Negacion")],
  [("n", "Grupo"), ("t", ","), ("n", "Negacion"), ("t", ":"), ("n", "Consulta")],
  [("n", "RecorteObjeto"), ("t", "no"), ("t", ";"), ("n", "RecorteObjeto"), ("t", ","), ("n", "Consulta")],
  [("n", "Negacion")])
# G01 Peticion
_pet = []
for pre in ([], [("t", "¿")]):
    for post in ([], [("t", "?")], [("t", ".")]):
        _pet.append(pre + [("n", "Cuerpo")] + post)
R("Peticion", *_pet)

CAP = 5000

def make_parser(toks):
    N = len(toks)
    def is_entero(i, j):
        if j - i != 1: return 0
        w = toks[i]
        return 1 if (1 <= len(w) <= 10 and all(c in "0123456789" for c in w)) else 0
    from functools import lru_cache
    @lru_cache(maxsize=None)
    def parse(sym, i, j):
        if sym == "Entero":
            return is_entero(i, j)
        total = 0
        for alt in G[sym]:
            total += seq(tuple(alt), i, j)
            if total > CAP: return CAP
        return total
    @lru_cache(maxsize=None)
    def seq(items, i, j):
        if not items:
            return 1 if i == j else 0
        kind, val = items[0]
        rest = items[1:]
        if kind == "t":
            if i < j and toks[i] == val:
                return seq(rest, i + 1, j)
            return 0
        total = 0
        for k in range(i, j + 1):
            c = parse(val, i, k)
            if c:
                r = seq(rest, k, j)
                if r:
                    total += c * r
                    if total > CAP: return CAP
        return total
    return parse

def analizar(pregunta):
    toks, _ = lex(pregunta)
    p = make_parser(tuple(toks))
    return toks, p("Peticion", 0, len(toks))

if __name__ == "__main__":
    ruta = sys.argv[1]
    d = json.load(open(ruta))
    casos = d["casos"]
    res = []
    for c in casos:
        q = c["solicitud_original"]["pregunta"]
        toks, n = analizar(q)
        res.append((c["id"], c["solicitud_original"]["id"], q, len(toks), n))
    fallan = [r for r in res if r[4] == 0]
    ambiguas = [r for r in res if r[4] > 1]
    print("TOTAL casos: %d" % len(res))
    print("derivaciones = 0 (NO PARSEA): %d" % len(fallan))
    print("derivaciones = 1 (unica)    : %d" % len([r for r in res if r[4] == 1]))
    print("derivaciones > 1 (AMBIGUA)  : %d" % len(ambiguas))
    print()
    print("--- NO PARSEAN ---")
    for r in fallan:
        print("  %s / %s  toks=%d\n      %s" % (r[0], r[1], r[3], r[2]))
    print()
    print("--- AMBIGUAS (n derivaciones) ---")
    for r in sorted(ambiguas, key=lambda x: -x[4]):
        print("  %s / %s  n=%s toks=%d\n      %s" % (r[0], r[1], r[4], r[3], r[2]))
