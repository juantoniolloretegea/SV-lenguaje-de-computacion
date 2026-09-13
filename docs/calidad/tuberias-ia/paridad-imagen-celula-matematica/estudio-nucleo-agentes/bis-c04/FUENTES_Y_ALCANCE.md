# Fuentes, corte y alcance de BIS-C04

**S22 · RETP-2026-207 · 13 de septiembre de 2026**

Corte del Lenguaje: `fd815e9b7921cb05854ed290a609957e9d07521a`. Corte del laboratorio: `2dd73e10af75fec840ffb31e71feaa7a04e62338`. Se verificaron las referencias de ambas ramas antes de preparar el incremento.

La lectura completa previa de los tres rectores obligatorios —Pilares, perfiles/contratos/ensamblaje y transición secuencial de inmunología con adendas— conserva identidad de bytes respecto del manifiesto C03. El [manifiesto de fuentes](FUENTES.json) identifica sus huellas. Se ha leído el texto completo de los Fundamentos aportado al repositorio; el contrato utiliza específicamente sus §§2, 3 y 4.1. De la Frontera normativa se han consultado la nomenclatura, A.1/A.2 y C.4; de la IR heredada, J5.2. Se leyó completa la adenda de vigencia C01–C03 para evitar confundir su nomenclatura con BIS-C04.

La inspección material se limita a Tri y sus codificaciones explícitas en `lib.rs`, las variantes y campos pertinentes de `ir.rs`, y la búsqueda de VisualCode y construcción polar en `rust/sv_core/src`. No se identificó en ese alcance una entrada que materialice el contrato gráfico completo. Esto no afirma la inexistencia de generadores en otros repositorios: sus antecedentes ya están documentados por S20.

Se consultaron las fórmulas de orientación recogidas en HALLAZGOS y en la explicación V2. La transformación canónica a superior-horaria es (x,y)→(y,x); conserva los índices y permite contrastar convenios sin reordenar el estado. Su formulación no prueba aún una migración de los cascarones. El rol estructural ρᵢ, la influencia de grafo y la codificación visual ρ conservan significados distintos.

## Resultado del incremento

Se han preparado un contrato candidato, el convenio de prueba, tres transformaciones declaradas, cinco oráculos completos y veinte entradas. Los estados de dieciséis posiciones conservan los bytes matemáticos C03; los de veinticinco y cuarenta y nueve proceden de las fuentes sintéticas C01 identificadas. Los oráculos contienen todos los vértices y aristas, además de la leyenda, en un formato experimental explícito.

La verificación documental comprueba huellas, integridad estructural del banco, correspondencia literal de los oráculos con los estados y conservación del historial. No ejecuta un renderizador, la API candidata ni un observador de salida gráfica. C04 mantiene cero variantes ejecutadas. El incremento no modifica versiones, código Rust, constituciones de dominio o soporte, ni acredita E003/E504 como guardas ejecutadas.

## Pendientes delimitados

- BIS-03: sede e imposición de la correspondencia, con enlace C03.
- BIS-04 y etapas de prueba: realización, salidas observadas y sensibilidad del observador.
- Materialización gráfica: precisión, resolución, recorte y legibilidad; ningún presupuesto o tolerancia inventados.
- C05: vinculación de los bytes realmente consumidos con su revisión y el contexto de uso.

Las referencias explicativas anteriores conservan su versión. Este contrato forma parte del desarrollo experimental de BIS-02, conforme al workflow V2. S24 permanece pendiente.
