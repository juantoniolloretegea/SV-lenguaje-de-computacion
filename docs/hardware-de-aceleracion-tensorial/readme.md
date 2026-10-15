# Hardware de cómputo y aceleración tensorial del SV

Este apartado reúne las comprobaciones del **Selector del Árbitro - Director CPU/GPU/TPU/NPU** sobre dispositivos concretos. Su objeto es relacionar la decisión de selección con la ejecución física observada, la concordancia de resultados y las mediciones del recorrido completo.

La organización distingue **fabricante → dispositivo → configuración y plataforma del ensayo**. Kaggle y Google Cloud son plataformas de ejecución; no sustituyen la identidad del fabricante del procesador. Una configuración comprobada no acredita todos los dispositivos de una marca.

## Dispositivos y plataformas documentados

| Fabricante | Dispositivo | Clase | Plataforma del ensayo | Resultado delimitado |
|---|---|---|---|---|
| [NVIDIA](nvidia/readme.md) | [Tesla T4](nvidia/tesla-t4/readme.md) | GPU | Kaggle, T4 × 2; cálculo en una tarjeta | Cinco solicitudes GPU ejecutadas conforme a la decisión, incluidas dos automáticas |
| [Intel](intel/readme.md) | [Xeon a 2,00 GHz; identificación comercial parcial](intel/xeon-kaggle-20261014/readme.md) | CPU | Kaggle, cuatro procesadores lógicos visibles | Tres solicitudes CPU obligatorias ejecutadas sin lanzamientos GPU durante su cálculo |
| [Google](google/readme.md) | [TPU v5e](google/tpu-v5e/readme.md) | TPU | Google Cloud, tipo `v5litepod-1`, un chip | Ejecución TPU comprobada; la selección automática favoreció CPU al medir el recorrido completo instrumentado |
| [AMD](amd/readme.md) | [EPYC 7B13](amd/epyc-7b13/readme.md) | CPU | Google Cloud, anfitrión de la TPU v5e | Referencia CPU y dos selecciones automáticas CPU comprobadas |

Las cuatro fichas corresponden a **dos ensayos con modelos**, no a cuatro experimentos independientes. En Kaggle se utilizó Qwen3-8B Q4_K_M; en Google Cloud, Helsinki-NLP/opus-mt-en-es. Los antecedentes instrumentales y las series anteriores permanecen identificados en los informes de cada ensayo.

## Selector y alcance

La carpeta del [Selector del Árbitro - Director CPU/GPU/TPU/NPU](selector-arbitro-director-cpu-gpu-tpu-npu/readme.md) describe su función, los criterios comprobados y las fuentes Rust utilizadas. La observación del hardware, la decisión y la ejecución tienen funciones diferenciadas; la mera presencia de un acelerador no demuestra que haya calculado una solicitud.

La evidencia se circunscribe a las cargas y configuraciones de las cuatro fichas. La concordancia numérica y el cumplimiento de la selección de dispositivo no constituyen una evaluación de la calidad científica de las respuestas de los modelos.

## Evidencias

Las fichas incorporan identidad, configuración, resultados y límites del dispositivo. Los expedientes de modelos conservan el código exacto, las mediciones completas y los originales:

- [Kaggle: Qwen3-8B, CPU Intel Xeon y GPU NVIDIA T4](https://github.com/juantoniolloretegea/SV-motor/blob/32239f974af3a6367efd75d36ba984a6c2fc747e/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/qwen/qwen3-8b-cuantizacion-q4_k_m-gguf/sv-contraste-qwen-cpu-gpu-rust-20261014.md).
- [Google Cloud: OPUS-MT, CPU AMD EPYC 7B13 y TPU Google v5e](https://github.com/juantoniolloretegea/SV-motor/blob/32239f974af3a6367efd75d36ba984a6c2fc747e/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/helsinki-nlp/opus-mt-en-es.md).

Las referencias se fijan a la revisión `32239f974af3a6367efd75d36ba984a6c2fc747e` de SV-motor para mantener inequívoca la correspondencia documental.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

