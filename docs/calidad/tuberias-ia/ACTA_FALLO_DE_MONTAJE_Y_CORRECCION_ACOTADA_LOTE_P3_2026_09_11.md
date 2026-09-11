# Fallo de montaje y corrección acotada del conductor

**RETP-2026-139 · 11/09/2026.** Se conserva la [resultado desfavorable original](ie004/lote-p3/resultados-1/RESUMEN_ORIGINAL.json) como FALLO. En cada configuración: 36 controles del adaptador, repetidos tres veces, conformes; 63/72 recorridos conformes. Las otras nueve observaciones fallan por montaje del conductor, no se promueven como éxito. Preparación previa: Calidad `efcfe753c9066112367bf73ae372df4802cb2964`; laboratorio `f0c5030d5d4a526e87a20244e1ea26403b5144b6`. Rectoras cotejadas de RETP-138 sin modificaciones.

[Causa, testigo e intervención](ie004/lote-p3/CORRECCION_CAUSAL_CONDUCTOR.md): omití `revocado` al invocar A04 heredada en P3-04/14/24. La única diferencia de cuerpos es vigencia; la ambigüedad permanece anterior a política. Se restituye la configuración explícita de las fichas públicas anteriores. Ningún Rust, binario, pregunta ni esperado cambia.

[Comprobación de capturas y binarios](ie004/lote-p3/COMPROBACION_RONDA_1.json); [36 observaciones acotadas de corrección](ie004/lote-p3/CONTROLES_CORRECCION.json); [fijación de la corrección](ie004/lote-p3/FIJACION_CORRECCION.json). No se repiten los 432 controles del adaptador ni los 252 recorridos conformes. La corrección, única y causal, se ejecutará después de este depósito.

El manifiesto inicial falló por EISDIR ante un directorio temporal de Rust después de guardar todos los resultados. Se repara el inventario sin volver a ejecutar la matriz; se preservan stdout/stderr y se preparará su paquete recuperable. La reserva continúa cerrada y la compatibilidad integral /2–/3 sigue pendiente.
