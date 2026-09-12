# Diagnósticos del compilador y procedencia por unidad

**RETP-158 · 12 de septiembre de 2026 · Candidata experimental.**

E004/E115 y las colisiones conservan ahora causas y referencias a declaraciones originales. EOF mantiene su unidad responsable. La versión anterior y la candidata producen exactamente los mismos resultados en las 120 fuentes canónicas. El cambio está ensayado y conservado como cápsula; no se ha promovido al núcleo productivo.

## Leer en este orden

1. [Plan previo](PLAN_PREVIO.md) e [inventario de emisores previo](INVENTARIO_PREVIO.json).
2. [Contrato y límites](CONTRATO_Y_ALCANCE.md).
3. [Hallazgos, incluida la corrección del instrumento](HALLAZGOS.md).
4. [Catálogo](CATALOGO.json), [español](idiomas/es/diagnosticos.json) e [inglés](idiomas/en/diagnosticos.json).
5. [Resultado del receptor](evidencia/RESULTADO.json), [secuencia](evidencia/SECUENCIA.json), [observaciones](evidencia/OBSERVACIONES.json) y [registros completos](evidencia/REGISTROS_COMPLETOS.zip).
6. [Estado del encargo entre modelos](ESTADO_PARA_OTRAS_IA.md).

## Resultado comprobado

| Comprobación | Resultado |
|---|---|
| Unitarios heredados de RETP-157 | 239/239 en debug y 239/239 en release |
| Corpus canónico | 14 válidos y 106 inválidos; resultado heredado idéntico entre ambas candidatas en ambos modos |
| Casos focales | 29 distintos; tres repeticiones por modo; 174 observaciones en el receptor |
| Procedencia | UTF-8, CRLF, tabulación, EOF, colisiones y E115 entre unidades homónimas |
| Localización | 12 claves locales con mensajes ES/EN; E004/E115 mantienen sus códigos canónicos |
| Fronteras de acceso | Cuatro clientes negativos, seis errores de compilación esperados; cliente público conforme |
| Receptor portátil | 41 procesos, comandos, bytes de salida, huellas y medidas conservados |
| Otros proveedores | Ninguna ejecución recibida |

Las capturas de cualificación inicial y del receptor son distintas y están conservadas; no se suman repeticiones como casos nuevos. La cualificación inicial midió tiempo transcurrido; el receptor también registra CPU y máximo acumulado de memoria de hijos. Este último no es memoria por caso. No se midió coste monetario ni inferencia de proveedores.

## Reproducción

Descargue la carpeta completa o [PAQUETE_REPRODUCIBLE.zip](PAQUETE_REPRODUCIBLE.zip). Con Python 3.10 o posterior en Linux y rustc con biblioteca estándar nativa:

```sh
python reproducir.py /ruta/absoluta/rustc /ruta/a/un/directorio/nuevo
```

Se ha comprobado con rustc 1.98.0 (88d9e12ae178fab0fb5cc050a94da85685d449ea), destino x86_64-unknown-linux-gnu. El receptor recupera las cápsulas por SHA-256, ejecuta la base y la candidata, compara el corpus, ejecuta unitarios y casos focales y comprueba catálogo y clientes. No requiere red. Límite: 120 segundos por proceso y 2 MiB por flujo. El presupuesto del receptor no es un presupuesto de razonamiento de otra IA.

[Fuentes candidatas](CANDIDATA_FUENTES.json), [base RETP-157](BASE_FUENTES.json), [corpus](CORPUS_FUENTES.json), [parche](CAMBIO_INCREMENTAL.patch) y scripts están disponibles. El oráculo de las 28 primeras pruebas fue fijado en las assertions de focal.rs antes de ejecutarlo; INDICE_ESPERADOS.json es su índice posterior para el receptor, no un segundo oráculo independiente. El caso relacional adicional tiene su esperado fijado por separado.

## Corte y continuidad

Lenguaje: 4cacf3ec6bd5d0f31206197c7374515a56b34a89. Laboratorio: 1ac32131e2c328e395d828268a5e3517ce43fcca. Rectoras consultadas: pilares; acta de perfiles; acta de transición IMM y sus relevos; workflow acotado V2; contrato diagnóstico 109/110. Se mantiene el paso 6 del plan de integración, distinto de P6 del workflow. Los cimientos conceptuales del frame y de la auditoría del trabajo IA se conservan en sus carpetas troncales.

**Estado global: NO VERDE.** Continúan los emisores restantes, DG global, destinos WASI/navegador, recepción profesional y las compuertas de la campaña. La conformidad de esta candidata nativa no cambia esos estados.
