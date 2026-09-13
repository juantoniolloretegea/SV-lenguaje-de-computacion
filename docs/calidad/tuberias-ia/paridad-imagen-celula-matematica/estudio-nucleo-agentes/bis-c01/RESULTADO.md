# Resultado nativo BIS-C01 y continuidad de BIS-02

**13 de septiembre de 2026 · S22 · RETP-2026-203 · Juan Antonio Lloret Egea y Watson**

## Resultado comprobado

Se han ejecutado **trece variantes, trece conformes**, correspondientes a los escenarios BIS-C01-P y BIS-C01-N. Siete entradas admitidas conservan la IR completa esperada; seis entradas incompatibles se rechazan por la condición prevista. Los controles de sensibilidad detectan cuatro alteraciones del observable: posiciones, dimensión, referencia de especificación y stdout indebido en rechazo.

El [banco y contrato](README.md) se comprometieron en el laboratorio, commit `8cfcf1931a842cbad5b13483684ace38f6275c1f`, antes de ejecutar. Se conservan [resultados](evidencia/ejecucion-1/RESULTADO.json), stdout y stderr de cada variante, [herramientas y huellas de fuentes](evidencia/ENTORNO_Y_CORTE.json). Corte del Lenguaje probado: `5ba3fbab751d9e4561dcecf2659e0ee4a54f7b4f`. Rust y Cargo 1.98.0; compilación nativa sin red, código de salida cero y 25 advertencias del código existente. No se corrigieron ni suprimieron para este ensayo.

| Condición | Observación | Alcance de la conclusión |
| --- | --- | --- |
| b=3,4,5,6,7 y longitud exacta | Admisión y N=9,16,25,36,49 preservados | Geometría y longitud de esas entradas; sin máximo universal ni autorización de soporte. |
| Vectores no constantes y permutación expresamente declarada | Conservación íntegra del orden recibido | Fidelidad posicional, sin prueba de ligadura a parámetros de dominio. |
| Mismo estado de 16 coordenadas en ES/EN | Mismos objetos canónicos; identidad de cada fuente conservada | Control puntual de perfil; no cierre de BIS-C12. |
| Longitudes 15/17 y 24/26 | Rechazo `CellState State: longitud de vector incompatible` | No emisión de E003 ni conversión a U. |
| b=2 con cuatro o dieciséis coordenadas | Rechazo `CellSpec C: b debe ser >= 3` | En el segundo caso prevalece el defecto de CellSpec; no se acredita llegar a longitud. |

## Consecuencia contractual

La relación n=b² ya se deriva desde `b` en el descenso; la longitud del estado se contrasta antes de devolver una IR aceptada. Esta prueba no justifica sustituir ese recorrido por una nueva clase Frame ni realizar una migración general a arrays. La elección de almacenamiento sigue pendiente de los contratos de soporte y construcción. Las instancias sintéticas no modifican dominios vigentes.

Tampoco se confunde una conservación del vector recibido con la detección de una permutación respecto de una constitución externa. Esta última necesita las ligaduras posicionales de BIS-C03/C04. La paridad gráfica exige además identidad, revisión, convenio y observación de la representación consumida.

## Relevo verificable

BIS-02 permanece **en ejecución**: de los 24 escenarios originales, dos se han ensayado en el alcance aquí declarado y 22 siguen pendientes. No están cerrados los doce contratos, BIS-03–08 ni el Bis completo. El [banco previo v0.1](../BANCO_PREVIO_BIS_02_v0_1.json) conserva su estado histórico de cero ejecuciones; el [estado vigente](../ESTADO_WORKFLOW.json) registra el avance.

El siguiente objeto es **BIS-C02: contrato de soporte por versión**, con S={16,25} exclusivamente sintético para su banco, y rechazo de N=49 por ausencia de soporte, separado del juicio algebraico. La ruta SVP ensayada no recibe ese contrato: no puede usarse su admisión de b=7 como prueba de autorización de soporte. Antes de ejecutar ese contraste deben fijarse la entrada contractual, su sede y el observador; no procede inventar un límite de inmunología o ciberseguridad.

## Secuencia acordada por el autor

Se completa primero (p1+p3)-Bis; después se retoma y cierra el catálogo de errores y la fase aplicable. El análisis, selección e instalación de una GUI quedan para el paso posterior. C#/.NET, Rust u otra tecnología no se dan por seleccionados. La eventual presentación web de casos reproducibles acompañará la revisión humana bajo su propio alcance; este incremento no despliega cambios en el sitio web.
