# Precompromiso del banco SVG y consumo

**S22 · RETP-238 · W-S26-02 · 14/09/2026. Estado: preparado y compilado; banco sin ejecutar.**

[Contrato y revisión crítica](CONTRATO_SVG_CONSUMO.md) · [banco estructurado](BANCO_SVG_CONSUMO.json) · [entradas congeladas](ENTRADAS_SVG_CONSUMO.tar.gz).

Se preparan diez casos: nueve integrados y un control aislado del lector SVG. Se ejecutará una campaña normal por perfil Rust (debug y optimizado), con retorno esperado 0 y diez casos concordantes. Dos mutantes compilables se ejecutarán separadamente sobre RC01 en cada perfil; cada uno debe producir discrepancia funcional y retorno 1. Un fallo de compilación o pánico no cumple ese oráculo.

Las seis ejecuciones de binarios producirán 24 observaciones: 20 normales y cuatro mutadas. Incluyen 22 entradas al montaje de admisión y dos pruebas aisladas del lector. No se suman a las filas originales ni a los recuentos globales Bis. No se repetirán las campañas LIG, F01/F02 o de fronteras RETP-237.

La preparación reutiliza ES01 y ES02 del archivo histórico ENTRADAS_R01.tar.gz; sus fuentes, estados, constitución, convenio y geometría permanecen literales. Se fijan las nuevas invocaciones y, en RC03, el vínculo A/r2 que ya existe en el registro de custodia. Los originales históricos no se editan. El preparador Rust comprueba la tabla de diferencias y la cota entera sin llamar a admisión, renderizado ni consumo.

El oráculo gráfico mide 792 bytes y tiene SHA-256 `e5a9f9bc6df9b4f59a2a0cb908046239fda11ebbc247577b8e8c32b268a8c578`. El archivo de entradas tiene SHA-256 `9992e725a23b933696652acff4e0728bc05af1fb64f3a2ba90c3e4ed3a053088`. Contiene las envolturas esperadas y ofrecidas, canales de alteración, estados y manifiesto de huellas. El SVG alternativo conserva los mismos puntos con dos posiciones permutadas; no procede de una ejecución del nuevo productor.

Fuentes: [candidato](svg_consumo.rs), [conductor/observador](banco_svg_consumo.rs), [preparador](preparar_svg_consumo.rs). Se reutilizan el registrador de comandos y el cotejador R08 ya cualificados; la administración experimental sigue en Rust. Se conservarán los comandos, cfg de cada mutante, versiones, fuentes, capturas, salidas, retornos e incidencias. Los dos destinos deben publicarse y cotejarse antes de ejecutar el banco.

Las comprobaciones del consumidor preservan identidad y contexto, acotan la recepción y comparan los bytes con el artefacto del representador. La cualificación independiente comprueba además su correspondencia con el estado mediante el SVG y la tabla previos. No se confunden esas dos fronteras de confianza. La prueba de script sólo ensaya el lector restringido; no interpreta ni ejecuta el script.

S26 y Bis permanecen abiertos. Este incremento constituye material SVG y una operación local sobre sus coordenadas; no acredita rasterización, percepción de IA, pantalla, host productivo ni consulta histórica. El catálogo y cierre de fase conservan su precedencia sobre S24/C#/.NET.
