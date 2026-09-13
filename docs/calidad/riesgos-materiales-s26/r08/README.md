# S26 R08 · Candidata Rust para control de auxiliares

**Banco previo: quince casos especificados, aún no ejecutados en esta revisión.**

Entrada Lenguaje: `337058da503b9e08cbb9805b9cc920adca75f6b5`. Entrada laboratorio: `3680ba0c357506940fc4540a344daa3a4516d138`. Se reciben R07/RETP-234 y el Control de auxiliares, sin modificar los antecedentes. Pilares, perfiles/contratos/ensamblaje y transición secuencial conservan su rango y fueron consultados en esta continuidad. La candidata sirve al trabajo administrativo del Lenguaje; no modifica núcleo, IR, dominio, agente, BD, GUI o host.

## Reparación propuesta

1. `publicacion` vuelve a leer dos descriptores explícitos: expectativa y observación. Comprueba repositorio, rama, base, commit, completitud y todas las entradas no directorio del árbol, con modo/tipo/SHA. Rechaza rutas duplicadas y árboles truncados. No acepta un checkpoint `verified` ni omite verificaciones por un resultado anterior.
2. `preparar` recibe solicitud, cinco archivos de origen y cinco candidatos. Comprueba revisión siguiente, vínculo entre fila actual e historial, conservación de otras filas, prefijos históricos, nuevo RETP y concordancias documentales implementadas. Lee el conjunto antes de crear salida. Escribe exclusivamente una carpeta nueva y rechaza destinos existentes. Una marca final sólo se escribe después de releer los cinco archivos y compararlos con los bytes preparados.
3. `comprobar` relee marca, conjunto y candidatos; valida otra vez. La marca sola no acredita nada. La carpeta de origen nunca es destino de escritura de estos comandos.

Todas las guardas operativas son condiciones explícitas con `Result`. No dependen de `assert!` ni `debug_assert!`. Se ensayará la misma fuente con y sin optimización, con `debug-assertions=no` en el segundo perfil.

La reparación cambia el procedimiento: prepara un conjunto revisable fuera del vigente. No reemplaza simultáneamente cinco archivos del checkout ni promete una transacción local de ese tipo. El único publicador remoto continúa siendo el conector directo; el verificador Rust no contiene red ni credenciales y no mueve ramas.

## Perfil de entradas y límites

Las interfaces TSV son formatos locales de esta herramienta administrativa; no son sintaxis SV ni una nueva identidad soberana. UTF-8 obligatorio. `SV-AUX-PUB/1` recibe cinco campos únicos (`repo`, `branch`, `base`, `commit`, `complete`) y filas `entry`, ruta, modo, tipo, SHA separados por tabuladores. Los árboles se aplanan conservando blobs y gitlinks; se excluyen únicamente los nodos directorio. Las rutas con tabulador, salto de línea, barra inversa, componentes vacíos, punto o doble punto están fuera del perfil. Los modos admitidos son 100644/100755/120000 para blob y 160000 para commit. SHA hexadecimal minúsculo de 40 caracteres; no es una firma ni acredita autoridad.

La recepción de árbol completo y su correspondencia con el servidor dependen del adaptador de observación. El verificador no detectaría dos descriptores falsificados coherentemente ni que el llamador sustituyera ambos por una expectativa antigua. Exige una expectativa autorizada y una lectura nueva; no constituye esa autoridad por comparar textos. Tampoco ejecuta un compare-and-swap remoto ni protege la ventana entre cotejo y actualización.

`SV-AUX-REG/1` recibe `id`, `revision` y `retp` únicos. Perfil acotado: actualización de un suceso existente, una revisión nueva y un RETP-2026 nuevo. No admite altas nuevas ni sustituye todas las reglas editoriales/de workflow. Valida las correspondencias codificadas; no demuestra la verdad de la prosa ni su suficiencia técnica. Los cinco candidatos siguen sujetos al cotejo humano/documental del cambio.

Se admiten archivos regulares de hasta 4 MiB cada uno y hasta 10 000 entradas de árbol. La lectura se acota a límite+1 antes de admitir el archivo. No se acredita una cuota global de RAM, latencia, resistencia al host, inmutabilidad física ni recuperación tras corte de energía. `sync_all` y la relectura local no se convierten en una certificación de durabilidad. El fallo después de escribir una marca pero antes de confirmar su sincronización puede dejar estado por reconciliar; el consumidor debe releer el conjunto y las referencias, sin inferir éxito del silencio.

La escritura preparatoria presupone directorios bajo custodia del operador, sin escritor concurrente hostil. Rechaza archivos no regulares observados, pero no pretende resolver carreras hostiles de rutas. Mantiene visibles los restos de una preparación fallida y nunca los reutiliza automáticamente. Una ejecución interrumpida requiere identificación de su salida y comprobación, no una repetición ciega.

## Banco previo

Los fixtures reutilizan literalmente los cinco archivos de R07 P06 (origen intacto tras rechazo) y R07 P05 (candidato válido). Proceden de `r07/EVIDENCIA.tar.gz`, SHA-256 `531de46bfe752d6a7f1c1b8996d34afed711875a66cb3aa2ce63afcdf5174782`. No hay datos clínicos. El banco y los oráculos están escritos en Rust; las copias Python permanecen sólo como antecedente R07 y no se ejecutan aquí.

| Caso | Contraste previo y esperado |
| --- | --- |
| P01 | Árbol y contexto iguales admitidos. |
| P02 | Base observada distinta rechazada PUB_BASE. |
| P03 | Tras positivo, cambiar expectativa y reutilizar observación anterior: PUB_COMMIT; checkpoint verified: PUB_FORMATO. |
| P04 | Ruta duplicada rechazada antes de sobrescribir entrada en el índice. |
| P05 | Preparación válida de cinco archivos, comprobación posterior y origen literal intacto. |
| P06 | Revisión 99 incompatible: REG_REVISION, sin crear salida ni alterar origen. |
| P07 | Markdown de origen ausente: error IO antes de crear salida; demás archivos intactos. |
| P08 | Fallo controlado tras cada una de las cinco escrituras reales: origen intacto, restos visibles, sin marca final y comprobación rechazada. Cinco posiciones dentro de un caso. |
| P09 | Alterar archivo de salida conservando marca: SALIDA_DIFIERE. |
| P10 | Reutilizar directorio de salida: rechazo; preparación anterior permanece comprobable. |
| P11 | Estado administrativo inventado: REG_ESTADO sin salida. |
| P12 | Última revisión de historial candidata incompatible: REG_HIST_NUEVO sin salida. |
| P13 | Blob diferente: PUB_ARBOL; árbol incompleto: PUB_TRUNCADO. |
| P14 | CSV con comillas escapadas/CRLF válido; cabecera duplicada, comilla abierta, columnas incompatibles y texto tras cierre rechazados. |
| P15 | Solicitud válida admitida; revisión duplicada rechazada; comando verified inexistente rechazado. |

P08 utiliza un punto de inyección que devuelve error después de una escritura real; no simula fielmente un corte de energía, agotamiento de disco o muerte abrupta. Esas propiedades siguen fuera de esta cualificación. No se recontará como ejecución de los discriminadores R06.

## Compilación y ejecución tras precompromiso

```text
/opt/sv-rust-1.98.0/bin/rustc --edition=2021 main.rs -o /tmp/s26-r08-auxiliar
/opt/sv-rust-1.98.0/bin/rustc --edition=2021 banco.rs -o /tmp/s26-r08-banco-debug
/opt/sv-rust-1.98.0/bin/rustc --edition=2021 -C opt-level=3 -C debug-assertions=no banco.rs -o /tmp/s26-r08-banco-opt
/tmp/s26-r08-banco-debug /tmp/s26-r08-debug-nuevo
/tmp/s26-r08-banco-opt /tmp/s26-r08-opt-nuevo
```

Sin dependencias de Cargo, intérprete Python ni lógica de shell. El lanzador sólo ejecuta los programas indicados. Los resultados y directorios se conservarán en una incorporación posterior al banco. Una discrepancia conservará su campaña y exigirá revisión identificada; no se ajustará el esperado para presentarla como conforme.

Después se ejercitará la CLI con un conjunto administrativo real revisado y con las lecturas del árbol Git de esta misma publicación. Eso acreditará uso operativo en ese alcance, sin convertir el descriptor en autoridad ni seleccionar tecnología del SV. S26, R06, Bis y S24 conservan sus límites.
