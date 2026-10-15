# Intel Xeon — identificación parcial, Kaggle

Ensayo del 14 de octubre de 2026. CPU de referencia y destino obligatorio en el contraste de Qwen3-8B Q4_K_M con NVIDIA Tesla T4.

## Identificación y plataforma

| Elemento | Observación conservada |
|---|---|
| Plataforma | Kaggle; entorno virtual Linux x86-64 con T4 × 2 |
| Nombre expuesto | `Intel(R) Xeon(R) CPU @ 2.00GHz` |
| Procesadores lógicos visibles | 4 |
| Hilos configurados para el cálculo | 4 |
| Modelo comercial exacto | No identificado por el nombre expuesto; no se infiere |
| GPU del contraste | [NVIDIA Tesla T4](../../nvidia/tesla-t4/readme.md), índice 0 |
| Modelo y motor | Qwen3-8B Q4_K_M; variante experimental SV-Aprender |
| Fecha y serie principal | 14/10/2026; generación gobernada por el perfil experimental del Árbitro - Director |

La carpeta incluye la plataforma y la fecha para distinguir este registro de otros Xeon cuya identidad pueda acreditarse con mayor precisión. El inventario de una máquina virtual no demuestra disponibilidad exclusiva de todo el procesador físico.

## Resultado

La mediana de tres calibraciones CPU fue **11,835012 s**; en GPU fue **1,907238 s**, con pesos preparados. Las solicitudes CPU obligatorias emplearon **11,774004**, **11,660557** y **11,749921 s** de servicio. Durante su cálculo se observaron **cero lanzamientos GPU**.

El tiempo CPU acumulado entre hilos fue respectivamente **43,640**, **43,000** y **43,290 s**. Puede superar el tiempo de reloj por la ejecución simultánea de varios hilos. No equivale a energía consumida.

Las tres solicitudes respetaron su destino; las dos automáticas eligieron GPU. La elección CPU obligatoria demuestra cumplimiento de política, no superioridad temporal de CPU para esta carga. Las catorce generaciones de la serie —incluida calibración— produjeron los mismos dieciséis tokens, con terminación por cota. La CPU también intervino en el control de las solicitudes GPU.

## Evidencias y alcance

- [Informe y medidas completas](https://github.com/juantoniolloretegea/SV-motor/blob/32239f974af3a6367efd75d36ba984a6c2fc747e/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/qwen/qwen3-8b-cuantizacion-q4_k_m-gguf/sv-contraste-qwen-cpu-gpu-rust-20261014.md).
- [Verificación de la serie](https://github.com/juantoniolloretegea/SV-motor/blob/32239f974af3a6367efd75d36ba984a6c2fc747e/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/qwen/qwen3-8b-cuantizacion-q4_k_m-gguf/evidencias/gobierno-20261014/VERIFICACION-RUST.json).
- [Código del perfil de ejecución y reproducción](https://github.com/juantoniolloretegea/SV-motor/blob/32239f974af3a6367efd75d36ba984a6c2fc747e/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/qwen/qwen3-8b-cuantizacion-q4_k_m-gguf/codigo/gobierno-20261014/REPRODUCCION.md).
- [Ficha de la T4 del mismo ensayo](../../nvidia/tesla-t4/readme.md).

Este registro corresponde al mismo ensayo descrito en NVIDIA; no se suma como experimento independiente. No acredita otros Xeon, GPU Intel ni NPU Core Ultra.

[Volver a Intel](../readme.md).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

