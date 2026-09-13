# S26 R04 · Recepción instrumentada del testigo posterior

**RETP-228 · Variante experimental previa; ocho sondas pendientes.**

Recibe [R03](../r03/README.md) y C03/C04/C07 de [R02](../RECEPCION_CONTRACTUAL_R02.md).
Corte: Lenguaje `10c84db9d330f5aeac1ba8b74f069024c6260e5d`; laboratorio
`b763f5ab9f0dab59265ab1f2e73573ce954730d5`. Se cotejan sus archivos publicados
y las 26 fuentes de `FUENTES.json`; Pilares, perfiles/contratos y transición
secuencial leídos en esta continuidad mantienen sus bytes. La recepción es
subordinada a esas sedes y R2-0; no modifica la semántica del Lenguaje.

## Variante y objeto observado

`preparar.py` copia el fragmento de recepción/ejecución del conductor I0205
hasta antes de sus campañas instrumentales, conserva literalmente el observador
y crea `conductor.rs`. En el fragmento, `run_case` se denomina
`run_case_antecedente`; sus copias iniciales de `canonical` se retiran y la
afirmación de preservación se inicializa a falso. La nueva entrada
`run_observed_case`, definida en `recepcion.rs`, envuelve ese recorrido:

1. Abre y lee `state.bin` desde la ruta de ensayo fijada por el conductor.
2. Si la lectura falla o difiere del esperado reservado, no inicia la operación.
3. Ejecuta el recorrido I0205 recibido, conservando su resultado y entrega.
4. Tras el retorno normal, vuelve a abrir y leer `state.bin`.
5. Registra ambas observaciones y califica la preservación por separado.

**Lo observado es el archivo de entrada matemática del montaje**, no la memoria
interna del objeto admitido, una BD ni el estado físico del mundo. La ruta es
una dependencia de la custodia experimental; no autentica por sí misma el
archivo ni acredita que una sustitución por bytes idénticos sea el mismo objeto
material. Dos lecturas tampoco demuestran que no hubo cambios intermedios.

Los originales, el núcleo y la campaña histórica de 26 casos permanecen
intactos. Se integra la recepción en la copia acotada de la operación, no en
todo el ejecutable ni en todos sus casos. La copia se compila y prueba por su
nueva entrada; el primer positivo utiliza la entrada sin inyección. Las otras
sondas usan un punto de inyección posterior al retorno, disponible únicamente
con `cfg(test)`.

## Contrato material local, fijado antes de los ensayos

| Situación | Resultado de observación y continuación |
| --- | --- |
| Ambas lecturas completas coinciden con el esperado | `CONCORDANCIA_OBSERVADA` de los extremos leídos. No garantiza ausencia de modificaciones entre ellos ni duración posterior. |
| Lectura completa distinta | `ALTERACION_DETECTADA`; si es inicial se detiene antes de operar; si es posterior se conserva la entrega ya producida. |
| Fallo de apertura, lectura o cuota | `NO_ACREDITADO`, con etapa y causa. El contenido no disponible se representa como ausencia explícita de observación. |
| Fallo inicial | No hay operación ni lectura posterior; se conservan el intento de lectura inicial y su fallo. |
| Fallo posterior | Se conservan resultado, captura y recibo previos. No se infiere rollback ni ausencia de entrega; no hay reintento automático. |

Las etiquetas son del informe experimental, no estados nuevos de SV. La
preservación no sustituye el dictamen de admisión/entrega: ambos se muestran
separados. `legacy_evidence()` sólo entrega al comparador histórico una copia
con `before` y `after` cuando existen ambas lecturas completas. No transforma
un fallo en un vector vacío ni en U. En el informe bruto de entrega los campos
heredados de estado quedan nulos; las observaciones materiales tienen sede propia.

La lectura admite hasta 1024 bytes de testigo, cuota recibida del canal de estado
I0205. Como máximo se leen 1025 para detectar exceso; no se continúa acumulando
el resto. Esto limita bytes de ese lector, no mide RAM total ni acredita todas
las cotas del recorrido. Los otros cargadores y cuotas históricos se conservan.

Un mismo proceso podría falsear tanto lector como informe. Esta variante
elimina la copia inicial como implementación ordinaria del testigo posterior
en su recorrido, pero no acredita protección frente a proceso/host comprometido.
La ruta observada puede cambiar durante una lectura; atomicidad, identidad
material y coherencia frente a ese fallo siguen pendientes de perfil y ensayo.
Pánico, terminación abrupta o bloqueo del recorrido antecedente no están
resueltos por envolver su retorno normal; no se promete informe posterior si
éste no retorna. El límite de 120 segundos por orden del reproductor es técnico.

## Banco previo de cualificación

A son los quince fixtures de R01 literales. La variante matemática modifica
únicamente el primer componente `Zero` a `One` del vector de 16 posiciones;
conserva los otros quince valores, b y n. Su JSON está fijado antes de ejecutar.
No altera la constitución ni pretende ser un nuevo suceso productivo.

| Sonda | Estímulo y control | Resultado esperado |
| --- | --- | --- |
| P01 | Estado intacto; entrada pública sin inyección. | Recibo completo original, ambas lecturas iguales al testigo, concordancia y comparador heredado sin discrepancias. |
| P02 | Añadir un espacio al archivo después de la operación. | Diferencia literal detectada en relectura; recibo previo conservado; discrepancia de preservación en el comparador. |
| P03 | Cambiar primer componente a One después de la operación. Luego alterar sólo la evidencia adaptada sustituyendo after por before. | Relectura detecta el vector distinto; el comparador detecta los bytes reales, pero vuelve a aceptar la copia falsa. Este último resultado conserva visible el límite de confianza del observador. |
| P04 | Eliminar el archivo tras la operación. | Fallo de apertura posterior explícito; entrega previa y un intento conservados; adaptación al observador no disponible. |
| P05 | Eliminar el archivo antes de la operación. | No acreditado, cero intentos y ninguna ejecución del punto posterior. |
| P06 | Sustituir archivo por 2048 espacios después de la operación. | Exceso detectado tras 1025 bytes; sin adaptación al observador ni conversión a U; entrega previa conservada. |
| P07 | Vector alterado ya en la lectura inicial. | Alteración detectada, cero intentos, sin ejecutar el punto posterior. |
| P08 | Sustituir el archivo por directorio después de la operación, en Linux. | Apertura seguida de error de lectura explícito; entrega previa conservada; no acreditado. |

Estos controles cubren detección en extremos, separación del fallo y detención
inicial en el montaje. P02/P03/P04/P06/P08 no prueban prevención de la entrega:
la alteración o el fallo se introduce después. Las ocho funciones no equivalen
a ocho casos globales S26 cerrados ni a una auditoría completa del conductor.

## Ejecución y conservación

`BANCO_PREVIO.json`, `PRECOMPROMISO.json`, los fuentes y fixtures se publican
antes de ejecutar Cargo. `reproducir.py` crea una campaña nueva y compila la
copia de conductor con el admisor/núcleo archivados mediante `--locked --offline`.
Se conservan órdenes, versiones, stdout/stderr y un informe de observación por
caso. Python sólo prepara, orquesta y registra; la operación y las aserciones
se ejecutan en Rust. Se utiliza el conector GitHub para publicar las mismas
copias conforme a `publicar.py`; no se invoca inferencia externa ni otro agente.

La siguiente integración depende del resultado y deberá cubrir las salidas
restantes, interrupción/pánico y sustitución durante lectura antes de acreditar
el recorrido completo. Las obligaciones de R2, S22 y el relevo de S24 se conservan.

## Resultado / RETP-229

Ocho sondas conformes a sus oráculos, cero fallos de aserción, una campaña Rust/Cargo 1.98.0 offline. Precompromiso público `aee7ced37e2edb17e35e0f9ee270e15312e7d9c5`. Fuentes, banco y código congelados conservados.

P01 admite el control intacto. P02 detecta bytes distintos; P03 detecta el cambio del primer componente del archivo a One mientras el vector admitido y la entrega anteriores conservan Zero. El informe distingue esa secuencia: no reescribe la entrega ni presenta el archivo posterior como su contenido original.

P04/P08 conservan entrega previa y muestran fallos de apertura/lectura posteriores. P06 observa 1025 bytes y declara exceso sin recorrer los 2048. P05/P07 detienen la operación ante fallo o alteración inicial, con cero intentos de despacho. El control que sustituye after por before sigue haciendo pasar al observador heredado: se conserva ese límite y no se confunde la recepción ordinaria corregida con resistencia a evidencia falseada por el mismo proceso.

[Resultados](RESULTADOS.json), [stdout](evidencia/02.stdout), [stderr](evidencia/02.stderr), [órdenes](evidencia/ORDENES.json) y [huellas](evidencia/HUELLAS.json). Las observaciones y archivos posteriores están en evidencia/observaciones, con inventario de rutas ausentes/directorio.

Se cualifica esta variante local, no el ejecutable completo ni todos los casos Bis. El siguiente incremento debe recibir las salidas restantes y los fallos que impiden el retorno normal, y concretar sustitución durante lectura. S26 sigue abierto; R2 y el relevo de GUI conservan sus dependencias.
