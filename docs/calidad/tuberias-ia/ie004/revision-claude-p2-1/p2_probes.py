#!/usr/bin/env python3
from p2_parser import analizar

PROBES = [
 # --- inversion semantica de "solo" bajo negacion ---
 "No sólo consulte la IgG.",
 "Sólo consulte la IgG.",
 "No consulte la IgG.",
 # --- disyuncion G10 ---
 "El valor de la IgG o la IgA.",
 "No consulte la IgG o la IgA.",
 "No quiero la IgG o la IgA.",
 # --- momento con articulo tras preposicion ---
 "El valor de la IgG anterior.",
 "El valor de la IgG del anterior.",
 "El valor de la IgG de la anterior.",
 "El valor de la IgG del corte anterior.",
 "El valor de la IgG en el corte anterior.",
 # --- separadores ---
 "El valor\nde la IgG.",
 "El  valor  de  la  IgG.",
 "El valor de la IgG ?",
 "¿El valor de la IgG.",
 "El valor de la IgG",
 # --- orden de complementos (posible ambiguedad de Comps) ---
 "El valor de la IgG del caso-a actual.",
 "El valor actual de la IgG del caso-a.",
 "El valor de la IgG de caso-a de este caso.",
 # --- papel sintactico dato/registro ---
 "La procedencia del dato anterior de IgG.",
 "El dato del registro de la IgG.",
 "El dato del dato.",
 "El registro de la IgG.",
 "Qué valor tiene el registro de la IgG.",
 "En qué unidad está expresada la IgG.",
 "En qué unidad figura el registro de la IgG.",
 # --- elipsis y contextual ---
 "El valor de este ...",
 "El valor de este caso.",
 "El valor del caso que estamos viendo.",
 "El valor de el caso que estamos viendo.",
 # --- escritura / eliminacion ---
 "Escriba 7 en el valor de la IgG.",
 "No escriba 7 en el valor de la IgG.",
 "Elimine el valor de la IgG.",
 "No elimine el valor de la IgG.",
 "No cambie nada.",
 "El valor de la IgG, no cambie nada.",
 # --- G02 combinaciones ---
 "No quiero la IgA; el valor de la IgG.",
 "No quiero la IgA: el valor de la IgG.",
 "El valor de la IgG, no quiero la IgA.",
 "El valor de la IgG; no quiero la IgA.",
 "El valor, no quiero la IgA: la IgG.",
 "Del caso-b no; del caso-a, el valor de la IgG.",
 "No la IgG.",
 "No.",
 # --- cortesia / motivo ---
 "El valor de la IgG, por favor.",
 "El valor de la IgG por favor.",
 "Consulte el valor de la IgG porque estoy revisando caso-a, por favor.",
 "Consulte el valor de la IgG porque estoy revisando caso-a por favor.",
 # --- aperturas ---
 "Cuál es el valor de la IgG.",
 "Adónde estará el valor de la IgG.",
 "Y el valor de la IgG.",
 "¿Cuál es el valor de la inmunoglobina?",
 "¿Adónde estará el valor de la linmunoglobina del caso-a?",
 # --- fuera de cobertura ---
 "El valor de la hemoglobina glicosilada.",
 "El intervalo de referencia de la IgG.",
 # --- formas humanas naturales no cubiertas (control negativo) ---
 "Necesito el valor de la IgG y el de la IgA.",
 "¿Me puedes dar la IgG?",
 "El valor de la IgG del paciente.",
 "Dígame el valor de inmunoglobulina.",
 "Traiga la cifra registrada de la IgM del caso-b actual.",
]

print("%-4s %-4s  %s" % ("n", "tok", "pregunta"))
print("-" * 90)
for q in PROBES:
    toks, n = analizar(q)
    marca = "   " if n == 1 else ("*0*" if n == 0 else "*%d*" % n)
    print("%-4s %-4d %s  %s" % (n, len(toks), marca, q.replace("\n", "\\n")))
