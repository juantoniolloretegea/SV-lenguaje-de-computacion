# Google TPU v5e — ensayo en Google Cloud

Ensayo del 14 de octubre de 2026. Contraste de Helsinki-NLP/opus-mt-en-es dirigido por el Selector del Árbitro - Director CPU/GPU/TPU/NPU.

## Identificación y entorno

| Elemento | Observación conservada |
|---|---|
| Plataforma | Google Cloud |
| Zona | `us-central1-a` |
| Tipo de recurso | `v5litepod-1`; una TPU v5e de un chip |
| Dispositivo observado | TPU 0 |
| Memoria comunicada por PJRT | 16 909 336 064 bytes |
| CPU anfitriona | [AMD EPYC 7B13](../../amd/epyc-7b13/readme.md); 24 procesadores lógicos |
| Memoria principal | 50 499 756 032 bytes |
| Sistema | Ubuntu 22.04; glibc 2.35 |
| Compilación y biblioteca del fabricante | Rust 1.98.1; libtpu 0.0.50 |
| Modelo | Helsinki-NLP/opus-mt-en-es; revisión `fdaddf76f50fcc1583ba42f95965862a7ab30f97` |

Esta ficha identifica la configuración Google Cloud de un chip. No describe una TPU de Kaggle ni un conjunto de ocho chips.

## Resultado de la serie final

La serie 04 contiene **22 traducciones: doce CPU y diez TPU**, incluidas las comprobaciones y calibraciones del protocolo. Se ejecutaron seis decisiones del Selector y dos rechazos lógicos de TPU no disponible. Las dos selecciones automáticas eligieron CPU.

| Caso | Generación CPU, ms | Generación TPU, ms | Solicitud completa CPU, ms | Solicitud completa TPU, ms |
|---|---:|---:|---:|---:|
| NCI_4-0 | 281,263 | 106,164 | 327,797 | 1.238,158 |
| NCI_178-3 | 541,283 | 195,340 | 588,563 | 2.013,101 |

Son medianas de tres mediciones por dispositivo y caso, con pesos y programas disponibles. La recuperación, el análisis y la conservación de las trazas incrementaron el recorrido completo TPU. Por ello, generar más deprisa no determinó por sí solo la selección. La carga del modelo y la compilación inicial se informan aparte.

El cotejo recalculó **56 comparaciones vectoriales**: diferencia absoluta máxima **2,28881836 × 10⁻⁵** y error cuadrático medio normalizado máximo **3,09781649 × 10⁻⁶**. Tokens y traducciones coincidieron en los dos casos fijados.

## Realización y observación

Rust expresa el codificador y decodificador Marian en StableHLO, controla la generación y conserva las medidas. PJRT y libtpu realizan la compilación y ejecución TPU; esta dependencia nativa impide describir toda la infraestructura como íntegramente Rust. StableHLO pertenece al adaptador experimental del modelo y no modifica la representación intermedia del SV.

Las trazas XSpace atribuyen programas y operaciones a TPU 0; las ejecuciones CPU no presentan actividad TPU. La CPU mantiene tokenización, selección del siguiente token, recuperación y composición del texto durante la vía TPU. No se midieron energía ni tráfico físico del bus.

## Fuentes y límites

- [Informe completo, código y antecedentes](https://github.com/juantoniolloretegea/SV-motor/blob/32239f974af3a6367efd75d36ba984a6c2fc747e/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/helsinki-nlp/opus-mt-en-es.md).
- [Resultado final](https://github.com/juantoniolloretegea/SV-motor/blob/32239f974af3a6367efd75d36ba984a6c2fc747e/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/helsinki-nlp/evidencias/contraste-cpu-tpu-20261014/RESULTADO.json).
- [Cotejo en Rust](https://github.com/juantoniolloretegea/SV-motor/blob/32239f974af3a6367efd75d36ba984a6c2fc747e/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/helsinki-nlp/evidencias/contraste-cpu-tpu-20261014/COTEJO-INDEPENDIENTE.json).
- [Código y reproducción del contraste](https://github.com/juantoniolloretegea/SV-motor/blob/32239f974af3a6367efd75d36ba984a6c2fc747e/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/helsinki-nlp/codigo/contraste-cpu-tpu/LEAME-SV.md).
- [Originales y ejecutables conservados](https://github.com/juantoniolloretegea/SV-motor/releases/tag/helsinki-cpu-tpu-20261014).

La primera serie completa utilizó una medición parcial y permanece separada. Las cifras anteriores son las de la serie final; no se suman a los antecedentes ni acreditan calidad lingüística general. La indisponibilidad fue lógica, no desconexión física. Recurso y solicitud temporal se retiraron después de conservar y cotejar las evidencias.

[Volver a Google](../readme.md).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

