# Criterio transversal de aceptación material en Rust

**Versión 1 · S22 · RETP-2026-208 · 13 de septiembre de 2026 · Juan Antonio Lloret Egea y Watson**

Se recibe la advertencia expresa del autor: la aceptación de datos por una herramienta auxiliar Python no acredita su admisión en Rust. JSON es un formato de datos; su presencia no identifica el lenguaje que implementa una operación.

## Obligaciones distintas

1. **Preparación:** identificar fuentes, conservar bytes y fijar oráculos independientes. Python puede auxiliar esta tarea; su resultado es preparación documental.
2. **Compilación Rust:** compilar la realización y los clientes válidos con toolchain, edición, target, dependencias y features identificados. Las pruebas negativas de construcción privada, tipos o préstamos deben fallar por el motivo previsto, con controles positivos equivalentes.
3. **Recepción de datos:** ejecutar en Rust el receptor concreto sobre entradas literales. Que un binario compile no demuestra que sus datos JSON, su dimensión o su identidad sean admisibles.
4. **Validación SV:** comprobar las restricciones algebraicas y los contratos constituidos; preservar orden, dimensión, pertenencia, revisión y U. La sintaxis correcta y los tipos primitivos correctos no bastan.
5. **Ejecución y observación:** contrastar salidas y rechazos reales con los oráculos previos, conservar comandos, códigos de salida y diagnósticos; demostrar sensibilidad a pérdidas del enlace probado.

Cada informe indicará cuáles de estas obligaciones fueron probadas y cuáles permanecen pendientes. No se elevará «preparado» a «compilado», ni «compilado» a «conforme», ni un resultado de laboratorio a capacidad productiva integrada.

## Frontera de deserialización

Antes de fijar el receptor se declarará su política para campos obligatorios y desconocidos, claves duplicadas, nulos, booleanos frente a números, signo, fracciones, exponentes, rangos y desbordamientos. También se declararán UTF-8, escapes, normalización, límites de tamaño y profundidad, y conservación del orden de arrays. Estas políticas requieren controles positivos y negativos en el receptor elegido; no se presume la configuración de una biblioteca aún no seleccionada.

Los casos destinados a detectar claves duplicadas deben conservarse como bytes originales: convertirlos previamente en un diccionario puede borrar el defecto. Los límites numéricos provienen del tipo y contrato elegidos, no de los enteros sin cota fija de una herramienta auxiliar. La tolerancia de un parser auxiliar no autoriza redondear, rellenar, reordenar o reparar entradas SV. La equivalencia estructural de JSON tampoco sustituye la igualdad de bytes cuando ésta sea el contrato.

El soporte admite tamaños constituidos por versión. Un array `[T; N]` preserva su longitud, pero no demuestra que N esté autorizado, que los parámetros estén en orden o que sus valores cumplan el contrato. Una colección encapsulada requiere igualmente esas guardas. Los diagnósticos conservan la diferencia entre fallo técnico, rechazo y Tri.U.

## Evidencia mínima antes de declarar una realización conforme

Se fijarán fuente y oráculos antes de ejecutar; corte de código, versión de rustc/cargo, edición del crate, target, features y dependencias; comandos reproducibles, salidas completas y controles de alcance. Se ejecutarán los tests relevantes del workspace, los clientes de uso y las entradas adversas de datos. Cuando el riesgo incluya aritmética o diferencias por optimización, se contrastarán debug y release. Un fallo de compilación de un mutante no cuenta como detección funcional del observador.

La comprobación de instalación efectuada en este relevo devuelve rustc 1.98.0 y cargo 1.98.0; `rust/sv_core/Cargo.toml` declara edición 2021. El parámetro edición 2024 de un Playground no cambia esa declaración. Esta comprobación no es una compilación ni una ejecución nueva de C02–C05. El expediente previo C01 conserva sus ensayos nativos.

Los comentarios de las realizaciones nuevas seguirán la política documental ES/EN vigente. Esa documentación no modifica los perfiles lingüísticos de la DSL. BIS-03 decide sedes y BIS-04 realiza los cambios justificados: este criterio no anticipa una implementación ni sustituye las pruebas pendientes.
