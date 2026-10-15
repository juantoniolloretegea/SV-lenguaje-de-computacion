# AMD EPYC 7B13 — ensayo en Google Cloud

Ensayo del 14 de octubre de 2026. CPU anfitriona, referencia numérica y destino seleccionado en el contraste de Helsinki-NLP/opus-mt-en-es con TPU Google v5e.

## Identificación y entorno

| Elemento | Observación conservada |
|---|---|
| Fabricante y procesador | AMD EPYC 7B13 |
| Plataforma | Google Cloud, zona `us-central1-a` |
| Configuración asociada | Anfitrión de una TPU `v5litepod-1` |
| Procesadores lógicos visibles | 24 |
| Trabajadores de cálculo configurados | 4, mediante Rayon |
| Memoria principal | 50 499 756 032 bytes |
| Sistema | Ubuntu 22.04; glibc 2.35 |
| Realización CPU | Rust, Candle 0.11.0; F32; generación voraz |
| Acelerador del contraste | [Google TPU v5e](../../google/tpu-v5e/readme.md) |

## Resultado

La serie final realizó **doce traducciones CPU y diez TPU**. La CPU sirvió de referencia y fue elegida en **las dos decisiones automáticas** por menor duración del recorrido completo instrumentado.

| Caso | Mediana de generación CPU | Mediana de solicitud completa CPU | Mediana de solicitud completa TPU |
|---|---:|---:|---:|
| NCI_4-0 | 281,263 ms | 327,797 ms | 1.238,158 ms |
| NCI_178-3 | 541,283 ms | 588,563 ms | 2.013,101 ms |

Las trazas de las ejecuciones CPU no registraron actividad TPU. El contraste conservó la misma salida para ambos dispositivos y cumplió las tolerancias fijadas. Las ventajas temporales son específicas del modelo, los dos casos y la instrumentación completa; no implican que CPU supere siempre a TPU.

El muestreo CPU de toda la serie, incluidos los recorridos TPU, conservó **322 observaciones**, intervalo máximo de **149,023 ms** y máximo observado de memoria residente de **6 412 808 192 bytes**. Estas magnitudes no se atribuyen exclusivamente a las doce traducciones CPU.

## Fuentes y alcance

- [Informe del ensayo compartido CPU/TPU](https://github.com/juantoniolloretegea/SV-motor/blob/32239f974af3a6367efd75d36ba984a6c2fc747e/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/helsinki-nlp/opus-mt-en-es.md).
- [Resultado final](https://github.com/juantoniolloretegea/SV-motor/blob/32239f974af3a6367efd75d36ba984a6c2fc747e/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/helsinki-nlp/evidencias/contraste-cpu-tpu-20261014/RESULTADO.json).
- [Cotejo de atribución y resultados](https://github.com/juantoniolloretegea/SV-motor/blob/32239f974af3a6367efd75d36ba984a6c2fc747e/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/helsinki-nlp/evidencias/contraste-cpu-tpu-20261014/COTEJO-INDEPENDIENTE.json).
- [Código del contraste](https://github.com/juantoniolloretegea/SV-motor/blob/32239f974af3a6367efd75d36ba984a6c2fc747e/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/helsinki-nlp/codigo/contraste-cpu-tpu/LEAME-SV.md).

Esta CPU se ensayó en Google Cloud, no en una máquina Instinct de AMD Developer Cloud. La ficha no acredita GPU AMD ni NPU Ryzen AI y no constituye un experimento adicional al documentado para la TPU.

[Volver a AMD](../readme.md).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

