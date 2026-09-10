#!/usr/bin/env python3
"""Traduccion sintactica de G01-G25 de IE004-ES-P2/2 (§4) y del lexico §3.

PAPEL Y LIMITE (exigidos por el encargo §2): herramienta auxiliar propia,
escrita por Claude, NO ejecutada por Watson, NO autoridad sobre la semantica
§5 ni sobre Rust. Cuenta unicamente derivaciones sintacticas. Uso en esta
revision: regresion de cobertura de las 48 preguntas publicas de /1 y de los
16 contrastes publicos de /2. No se ejecuta generacion exhaustiva.
"""
import json, sys

CONV = {}
for a, b in zip("ABCDEFGHIJKLMNOPQRSTUVWXYZ", "abcdefghijklmnopqrstuvwxyz"): CONV[a] = b
for a, b in zip("ÁÉÍÓÚÜÑ", "áéíóúüñ"): CONV[a] = b
SEPS = {" ", "\t", "\n", "\r"}
SYMS = {"¿", "?", ".", ",", ":", ";"}

def lex(texto):
    s = "".join(CONV.get(c, c) for c in texto)
    toks, i, n = [], 0, len(texto)
    while i < n:
        c = s[i]
        if c in SEPS: i += 1; continue
        if c == ".":
            if s[i:i+3] == "...": toks.append("..."); i += 3
            else: toks.append("."); i += 1
            continue
        if c in SYMS: toks.append(c); i += 1; continue
        j = i
        while j < n and s[j] not in SEPS and s[j] not in SYMS and s[j] != ".": j += 1
        toks.append(s[i:j]); i = j
    return toks

G = {}
def R(nt, *alts): G[nt] = list(alts)
def T(*ws): return [("t", w) for w in ws]
def N(x): return ("n", x)
ART = [[], T("el"), T("la")]
def wa(tails): return [a + t for a in ART for t in tails]

R("Momento", T("actual"), T("ahora"), T("anterior"), T("previo"))                       # G17
R("Temporal", *wa([T("actual"), T("ahora"), T("anterior"), T("previo")]))               # G12
R("Campo", *wa([T("valor"), T("dato"), T("cifra"), T("cifra", "registrada"),            # G09
                T("unidad"), T("estado"), T("estado", "registrado"), T("fuente"),
                T("procedencia"), T("límites"), T("alcance"),
                T("intervalo", "de", "referencia")]))
R("AtomoParametro", *[d + t for d in ([], T("el"), T("la"), T("esa"))                    # G11
                      for t in (T("igg"), T("iga"), T("igm"), T("inmunoglobulina"),
                                T("inmunoglobina"), T("linmunoglobina"),
                                T("hemoglobina"), T("hemoglobina", "glicosilada"))])
R("Parametro", [N("AtomoParametro")],                                                    # G10
   [N("AtomoParametro"), ("t", "o"), N("AtomoParametro")])
R("ObjetoExplicito", T("caso-a"), T("caso-b"))                                           # G15
R("ObjetoContextual", T("este", "caso"), T("ese", "caso"), T("este", "paciente"),         # G16
   T("ese", "paciente"), T("el", "caso", "que", "estamos", "viendo"), T("este", "..."))
R("Objeto", [N("ObjetoExplicito")], [N("ObjetoContextual")])                             # G14
R("ReferenciaObjeto",                                                                    # G25
   [("t", "de"), N("ObjetoExplicito")], [("t", "del"), N("ObjetoExplicito")],
   T("de", "este", "caso"), T("de", "ese", "caso"),
   T("de", "este", "paciente"), T("de", "ese", "paciente"),
   T("de", "este", "..."),
   T("del", "caso", "que", "estamos", "viendo"))
R("Complemento",                                                                          # G13
   [N("Temporal")],
   [("t", "de"), N("Parametro")], [("t", "para"), N("Parametro")],
   [("t", "para"), N("Objeto")],
   [N("ReferenciaObjeto")],
   [("t", "de"), N("Momento")], [("t", "de"), ("t", "corte"), N("Momento")],
   [("t", "del"), N("Momento")], [("t", "del"), ("t", "corte"), N("Momento")],
   T("de", "la") + [N("Momento")],
   T("en", "el", "corte") + [N("Momento")],
   T("del", "dato"), T("del", "registro"))
R("Cabeza", [N("Campo")], [N("Parametro")], [N("Temporal")], *wa([T("registro")]))        # G07
R("Comps", [N("Complemento")], [N("Complemento"), N("Comps")])
R("Grupo", [N("Cabeza")], [N("Cabeza"), N("Comps")])                                      # G06
R("SujCab", [N("Parametro")], *wa([T("registro")]))
R("Sujeto", [N("SujCab")], [N("SujCab"), N("Comps")])                                     # G08
R("Verbo", *[T(v) for v in ("consulte", "lea", "traiga", "dígame", "necesito",            # G24
                            "cambie", "elimine")])
R("Orden", [N("Verbo")], [("t", "sólo"), N("Verbo")], [("t", "solo"), N("Verbo")])        # G05
R("Nucleo",                                                                               # G04
   [N("Orden"), N("Grupo")],
   [("t", "escriba"), N("Entero"), ("t", "en"), N("Grupo")],
   [N("Grupo")],
   T("cuál", "es") + [N("Grupo")], T("adónde", "estará") + [N("Grupo")],
   [("t", "y"), N("Grupo")],
   [("t", "qué"), N("Campo"), ("t", "tiene"), N("Sujeto")],
   T("en", "qué", "unidad", "está", "expresada") + [N("Sujeto")],
   T("en", "qué", "unidad", "figura") + [N("Sujeto")])
R("Motivo", T("porque", "estoy", "revisando") + [N("Objeto")])                            # G20
R("Cortesia", T("por", "favor"), T(",", "por", "favor"))                                  # G21
R("Consulta", [N("Nucleo")], [N("Nucleo"), N("Motivo")], [N("Nucleo"), N("Cortesia")],    # G03
   [N("Nucleo"), N("Motivo"), N("Cortesia")])
R("Negacion",                                                                             # G18
   [("t", "no"), N("Verbo"), N("Grupo")],
   T("no", "escriba") + [N("Entero"), ("t", "en"), N("Grupo")],
   T("no", "quiero") + [N("Grupo")],
   [("t", "no"), N("Grupo")],
   T("no", "cambie", "nada"))
R("RecorteObjeto", [N("ReferenciaObjeto")])                                               # G19
R("Cuerpo",                                                                               # G02
   [N("Consulta")],
   [N("Negacion"), ("t", ";"), N("Consulta")], [N("Negacion"), ("t", ":"), N("Consulta")],
   [N("Consulta"), ("t", ","), N("Negacion")], [N("Consulta"), ("t", ";"), N("Negacion")],
   [N("Consulta"), ("t", ","), N("Negacion"), ("t", ":"), N("Consulta")],
   [N("RecorteObjeto"), ("t", ","), N("Consulta")],
   [N("RecorteObjeto"), ("t", "no"), ("t", ";"), N("RecorteObjeto"), ("t", ","), N("Consulta")],
   [N("Negacion")])
R("Peticion", *[pre + [N("Cuerpo")] + post                                                # G01
                for pre in ([], [("t", "¿")]) for post in ([], [("t", "?")], [("t", ".")])])

CAP = 5000
def make_parser(toks):
    from functools import lru_cache
    @lru_cache(maxsize=None)
    def parse(sym, i, j):
        if sym == "Entero":
            if j - i != 1: return 0
            w = toks[i]
            return 1 if (1 <= len(w) <= 10 and w.isdigit() and w.isascii()) else 0
        tot = 0
        for alt in G[sym]:
            tot += seq(tuple(alt), i, j)
            if tot > CAP: return CAP
        return tot
    @lru_cache(maxsize=None)
    def seq(items, i, j):
        if not items: return 1 if i == j else 0
        k0, v0 = items[0]; rest = items[1:]
        if k0 == "t":
            return seq(rest, i + 1, j) if (i < j and toks[i] == v0) else 0
        tot = 0
        for k in range(i, j + 1):
            c = parse(v0, i, k)
            if c:
                r = seq(rest, k, j)
                if r:
                    tot += c * r
                    if tot > CAP: return CAP
        return tot
    return parse

def analizar(q):
    t = lex(q)
    return t, make_parser(tuple(t))("Peticion", 0, len(t))

if __name__ == "__main__":
    base = sys.argv[1]
    print("== 48 preguntas publicas de /1 (regresion de cobertura) ==")
    d = json.load(open(base + "/ie004/perfil-es-p2-1/COBERTURA_DOCUMENTAL.json"))
    mal = []
    for c in d["casos"]:
        q = c["solicitud_original"]["pregunta"]
        t, n = analizar(q)
        if n != 1: mal.append((c["id"], q, n)); print("  n=%s %s | %s" % (n, c["id"], q))
    print("  no unicos: %d de %d" % (len(mal), len(d["casos"])))
    print()
    print("== 16 contrastes publicos de /2 ==")
    e = json.load(open(base + "/ie004/perfil-es-p2-2/CONTRASTES_PUBLICOS.json"))
    for c in e["casos"]:
        q = c["pregunta"]; t, n = analizar(q)
        esp = c["esperado_de_diseno"]["clase"]
        deriva_esperado = (esp != "SOLICITUD_NO_REPRESENTADA")
        ok = (n >= 1) == deriva_esperado
        print("  %-4s n=%-4s tok=%-3d %-28s %s  %s" %
              (c["id"], n, len(t), esp, "OK " if ok else "<<<", repr(q)[:60]))
