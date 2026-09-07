# Puerta operativa obligatoria para agentes

Este archivo rige todo el repositorio. No crea doctrina: obliga a leer la doctrina y el estado material vigentes antes de actuar.

## Antes de analizar o modificar

Toda unidad humana o asistida que intervenga en gramática, perfiles fuente, parser, IR, núcleo Rust, antecedente Python retirado, WebAssembly, pruebas, contratos de dominio o agente, frontera u host debe, en este orden:

1. identificar el commit o la rama exactos sobre los que trabaja;
2. leer completo `docs/calidad/PILARES_Y_RESTRICCIONES_DE_DISENO_DEL_LENGUAJE_DE_COMPUTACION_SV_2026_09_05.md`;
3. leer completo `docs/calidad/ACTA_TECNICA_DE_PERFILES_CONTRATOS_Y_ENSAMBLAJE_DEL_LENGUAJE_SV_2026_09_06.md`, que fija las obligaciones de suficiencia para representación, dominio y soporte tecnológico, sus ensamblajes y la prueba previa en laboratorio;
4. leer completo `docs/dominios/inmunologia/ACTA_DE_CONFORMIDAD_DE_TRANSICION_SECUENCIAL_DESDE_OP-IMM-001_AL_LENGUAJE_SV_2026_09_03.md`, incluida su adenda rectora de secuencia de 06/09/2026 (§§12–17) y los relevos de recepción N0-01 (§18), reparación de oráculos (§19), cierre relacional N0-02 (§20) unicidad de proyección N0-03 (§21) y referencia de arquitectura N0-04 (§22), retirada Python y deuda tecnológica diferida (§23), y el acta de fase y las entradas RETP directamente aplicables;
5. declarar en su informe el corte leído y las piezas rectoras consultadas.

Si el documento rector falta, no puede leerse o contradice la tarea recibida, la modificación se detiene y la contradicción se eleva. Quedan prohibidas la reparación silenciosa y la conversión del comportamiento accidental del código en doctrina.

## Guardas mínimas que nunca se presumen disponibles por memoria

- Una célula es un vector plano, ordenado y posicional de longitud `n=b²`, con `b≥3`; no es una matriz `b×b`.
- La unidad de dominio constituye células, tamaños, número, orden y asignación de parámetros. La unidad de agente recibe esa constitución y declara cobertura y capacidades. El Lenguaje representa, valida, preserva y rechaza; no suplanta esas decisiones.
- El inventario preliminar de parámetros no determina `b`, una célula ni el número de células.
- Se prohíben inferencia, redondeo, relleno, reordenación y reparación silenciosa. `U` no es relleno ni fallo técnico; `Bottom` permanece separado de `Tri.U`.
- Ninguna inferencia opaca, estadística o de LLM puede entrar en la cadena soberana de decisión del SV.
- Bus, perfil central de agente, perfil de agente como tipo de IR, transporte y host no están constituidos por el rector.
- `E003 — NSquaredViolation` está catalogado, pero no es hoy una protección ejecutable acreditada.

- Perfil fuente, constitución de dominio y perfil de soporte tecnológico tienen contratos distintos. Su composición no crea autoridad ni acredita por sí sola nuevas capacidades de semántica o IR. Antes de cerrar el alcance nuclear se deberá demostrar la suficiencia de sus operaciones; las realizaciones tecnológicas se ensayan previamente en el laboratorio identificado por el acta.

- La secuencia vigente distingue radiografía N0, niveles N0–N4 de la IR y fases R0–R4. Los retornos IMM/CYB son acotados; las obligaciones tecnológicas acompañan cada etapa, y no desplazan oráculos/K1 ni acreditan automáticamente R2/R3/R4.

Estas líneas son un índice de seguridad, no sustituyen la lectura completa del rector.

## Antes de cerrar una modificación

La unidad debe comprobar que:

- no ha decidido por el dominio o por el agente;
- toda obligación nueva es representable, verificable y tiene prueba positiva y negativa cuando corresponda;
- las ausencias y límites quedan explícitos;
- una decisión de diseño material actualiza el rector o su pieza sucesora y los registros RETP CSV y Markdown en el mismo cambio.

