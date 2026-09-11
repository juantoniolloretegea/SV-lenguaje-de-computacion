# Preparación del adaptador de lote y compatibilidad P3

**RETP-2026-138 · 11/09/2026.** Continúa RETP-137 por luz verde expresa de Juan Antonio. Corte Calidad `b5bb421a57d897faf003654fcaf9e5a7a120a1b0`; laboratorio `2885ed16ef2a578bd968ae2ff894c8dfcf7f417f`. Se cotejaron sin cambios AGENTS, Pilares, perfiles/ensamblaje, transición §§1–30 y arquitectura respecto de sus lecturas íntegras, y se consultaron contrato de captura, encargo, compromiso público, recibo y perfil /2.

[Contrato del adaptador](ie004/lote-p3/CONTRATO_ADAPTADOR_LOTE_1.md), [matriz de compatibilidad](ie004/lote-p3/MATRIZ_COMPATIBILIDAD_P3.md) y [decisión preparada, no emitida](ie004/lote-p3/DECISION_PREPARADA_CUSTODIO.json). Se implementa Rust sin unsafe: recepción limitada, huella previa a decodificación, esquema cerrado, 24 objetos ordenados y extracción conservadora de pregunta/contexto. Las fuentes del receptor A y semántica no se modifican. No se reciben notas, V ni oráculo.

[Fijación previa](ie004/lote-p3/FIJACION_PREVIA.json): 36 controles públicos, tres reproducciones por configuración nativo/WASI debug/release; recorrido de 24 posiciones públicas hasta los cuerpos A, tres veces. Diez preguntas públicas heredadas, repetidas con procedencia y esperados previos: no es validación inédita. Se han efectuado dos comprobaciones de tipos sin ejecutar consultas, ambas satisfactorias, la segunda tras añadir el registro de rechazos; no hubo corrección funcional por resultados observados. La campaña queda pendiente de este depósito espejado.

El millón de unidades /3 no se presenta como millón de intentos /2. El calendario obligatorio de /2 cambió expresamente en RETP-134; la compatibilidad integral de P3 sigue NO ACREDITADA. El tamaño público anunciado del lote cabe en la cota, pero no prueba validez de bytes aún reservados. La custodia humana ya confirmada se conserva.

Siguiente acto: una matriz pública según [reproducción](ie004/lote-p3/REPRODUCIR.md), conservación de todos los resultados y decisión acotada. La reserva sigue cerrada; no se abre captura de Grok ni se declara P4/P5 o promoción productiva. Catálogo/localización y fila 9 mantienen su secuencia.
