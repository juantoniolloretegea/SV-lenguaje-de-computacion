# Recepción de Grok 002 y corrección ES27/3 · 10/09/2026

**Resultado acotado:** las nueve consultas pertinentes de entrega-002 obtienen ahora el dato correcto; antes lo obtenían cinco. Las otras tres mantienen rechazo. **Aislamiento global NO VERDE; rendimiento WASI pendiente; nueva captura independiente pendiente.**

## Entrada y comprobación

Se reciben sin alteración entrega [de47ea16ea80a8c46bd18ad75a1112e4788fd855](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/commit/de47ea16ea80a8c46bd18ad75a1112e4788fd855) e informe [921d7197818e94360ee2d6bcafa33574ae6a25db](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/commit/921d7197818e94360ee2d6bcafa33574ae6a25db). Son dos commits sucesivos que añaden exclusivamente sus respectivos archivos. La captura ocupa 1322 bytes, SHA-256 `2345442c8f4eee3fcfcdc7f49f0712722ef65d7ff6d1613e46877ca59856b37b`.

La ejecución original [34489800105/1](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/actions/runs/34489800105) usa ES27/2, fuente e18ed7313bf184bf585f351d0f7e29fdf149db9d. Se descargó y cotejó el ZIP completo, incluida identidad de fuentes, lanzador, binarios, entrada y resultado. El informe de Grok coincide con la ejecución; success prueba que se ejecutó el banco, no que todas sus consultas funcionaran.

## Fallos y reparación

| Consulta original | ES27/2 | ES27/3 |
| --- | --- | --- |
| G06: ¿Cuál es el marcador del universo ficticio de prueba? | SOLICITUD_NO_REPRESENTADA | DATO: SIM-007 |
| G07: Diga la ciudad que es capital de España. | SOLICITUD_NO_REPRESENTADA | DATO: Madrid |
| G08: Consulte el registro 1 y después el registro 7. | PARAMETRO_MAL_FORMADO | DATO: Madrid, seguido de SIM-007 |
| G09: Consulte el registro 7 y después el registro 1. | PARAMETRO_MAL_FORMADO | DATO: SIM-007, seguido de Madrid |

Estos cuatro fallos corresponden al reconocimiento de Watson. G01–G05 conservan respuestas correctas. G10 solicita un río ausente de la ficha; G11 una operación clínica; G12 añade una instrucción ajena. Los tres conservan SOLICITUD_NO_REPRESENTADA y respuesta vacía. Ese estado no acredita que el motor distinga ausencia de conocimiento, permiso y cobertura lingüística. El permiso lógico sí se contrasta mediante registros excluidos y peticiones mixtas 1/13 en ambos órdenes: se deniega toda la consulta sin emitir contenido parcial.

G-ES27/3 amplía formas de petición, objetos y referencias coordinadas. Consume la entrada completa, mantiene números canónicos, límite de dos referencias, rechazo de duplicados y orden. No obtiene el esperado de propuesta_ia. Base LAB-ES27/1 y política P-1-7/1 permanecen fijas. El cambio está limitado al banco artificial y documentado en su ALCANCE.md.

Fuente [387e9d5a76a60b620dc3d4124a1a155adc07767e](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/commit/387e9d5a76a60b620dc3d4124a1a155adc07767e); lanzador 762ed044655fad31f51db389393930d71a207680. La ejecución [34491895499/1](https://github.com/juantoniolloretegea/SV-matematica-semantica-cuaternaria/actions/runs/34491895499), trabajo 102920464577, compila la misma fuente en Rust 1.98.0 nativo y wasm32-wasip1 y exige los esperados literales y la paridad.

**Verificación:** 110 controles (44 positivos y 66 negativos), incluidos los 76 anteriores; 4400 respuestas positivas repetidas por destino con propuestas contradictorias; seis transportes inválidos; sensibilidad del observador. Ambas capturas originales se reejecutan con oráculos fijados por sus hashes y tres repeticiones por destino. Entrega-001 conserva nueve datos y tres rechazos; entrega-002 pasa de cinco a nueve datos y mantiene tres rechazos. Es reejecución de Watson, no nueva sesión ni nueva aprobación de Grok.

## Rendimiento observado

Se conservan 42 muestras de la ejecución original y 44 de la corregida. El lote de medición contiene las mismas 26 preguntas, con idéntico hash de entrada. En Actions, la mediana nativa fue 3,679 → 3,669 ms y WASI 49,301 → 54,403 ms. Los máximos RSS fueron 2188 → 2164 KiB nativo y 53328 → 53292 KiB WASI. Son procesos completos, incluido el anfitrión WASI; no tiempo aislado del analizador.

La mayor latencia WASI motivó una comparación adicional de ambos binarios verificados en un mismo host local, Node v24.19.0, Intel Xeon Platinum 8573C. Veinte pares AB/BA alternos por carga y destino, tras dos pares de calentamiento; 160 medidas, todas con salida comprobada. El script y los datos quedan en este expediente.

| Carga | Destino | Mediana ES27/2 | Mediana ES27/3 | Cambio de medianas |
| --- | --- | ---: | ---: | ---: |
| 26 consultas | Nativo | 2,332 ms | 2,144 ms | −8,07 % |
| 26 consultas | WASI | 50,182 ms | 52,574 ms | +4,77 % |
| 1300 consultas | Nativo | 4,563 ms | 4,632 ms | +1,52 % |
| 1300 consultas | WASI | 58,603 ms | 66,359 ms | +13,24 % |

Existe variabilidad amplia entre pares. La mediana del cociente pareado WASI fue 1,053 para 26 consultas y 1,073 para 1300. **No se declara ausencia de pérdida de rendimiento.** Se registra una señal de penalización WASI pendiente de caracterizar antes de consolidar; estos datos no identifican su causa ni justifican por sí solos una refactorización. No se han hecho optimizaciones ni comparaciones equivalentes con los checkpoints 018/030.

## Custodia y continuidad

[Expediente verificable](es27-grok-002/MANIFIESTO.json): capturas e informe originales, resultados antes/después, oráculos y mediciones, con tamaño y SHA-256 por archivo. Se espejan los mismos bytes en laboratorio/tareas-watson/tuberias-ia y docs/calidad/tuberias-ia. El manifiesto no incluye su propio hash para evitar una referencia circular.

ZIP original: artefacto 10157230730, 1722207 bytes, SHA-256 f5c417499c6389b461345c0f4306ba47d61b4fac911066655cfb071b8df9ae83. ZIP corregido: 10158095483, 1725889 bytes, SHA-256 e746e26fa2cbb54faa48ae326dfb1a0b58ac4fccfa0e415161270214e027c218. Ambos fueron recuperados y cotejados; Actions indica caducidad el 10/10/2026. Git conserva fuentes, entradas, oráculos y resultados; no se presenta el ZIP temporal como custodia perpetua de binarios.

Encargo 003 queda preparado en laboratorio/tareas-grok/consultas-es27 para una captura nueva con reformulaciones propias y lecturas realmente fuera de permiso. El acceso del participante al repositorio impide acreditar ceguera absoluta; el encargo no la declara. Claude aún no recibe encargo de auditoría; Qwen sigue siendo candidato. La aprobación funcional de este banco no equivale a controlar una IA productiva: la respuesta sigue siendo una consulta determinista y la propuesta externa carece de autoridad.

Mandato humano vigente: «sigue con la iniciativa», funcionamiento antes de optimización y espejo en Calidad. Corte de entrada: laboratorio 921d7197818e94360ee2d6bcafa33574ae6a25db y Lenguaje f497da7116b4b85fc23a67cf8488b5612881391a. Se comprobó la identidad de AGENTS, Pilares, perfiles/contratos y transición completos leídos previamente, sin cambios; sucesión aplicable Fase 003 y RETP-115–117. Esta acta y RETP-118 documentan el incremento. No cambia semántica, IR, dominio real, código productivo ni PR #89. Catálogo/localización y fila 9 conservan su pausa y orden. Integridad adversarial de base, revocación y aislamiento material siguen pendientes.
