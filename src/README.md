# Frontend Python retirado del camino activo

RETP-2026-082 retira el compilador Python y su API del árbol ejecutable. El
último corte íntegro es [a09b9ef](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/tree/a09b9efef51f88de29048b9b35e7ac085dc0918f/src).
La realización vigente está en `rust/sv_core`; recibe SV conforme a los perfiles,
Gramática 0.2 e IR 0.3. Los oráculos comprometidos permanecen en `tests/conformance`.
El retiro conserva el historial y no acredita por sí solo las deudas diagnósticas
ni la serialización canónica completa. Véase la adenda de retirada en el acta de
[oráculos](../docs/calidad/ACTA_TECNICA_REPARACION_DE_ORACULOS_2026_09_06.md).
