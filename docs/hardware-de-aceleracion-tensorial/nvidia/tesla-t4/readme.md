# NVIDIA Tesla T4 — ensayo en Kaggle

Ensayo del 14 de octubre de 2026. Dispositivo GPU del contraste Qwen3-8B Q4_K_M mediante SV-Aprender y el Selector del Árbitro - Director CPU/GPU/TPU/NPU.

## Identificación y entorno

| Elemento | Observación conservada |
|---|---|
| Plataforma | Kaggle, configuración T4 × 2 |
| GPU utilizadas para el cálculo | Una: índice 0; la segunda tarjeta se observó por separado |
| Nombre comunicado | Tesla T4 |
| Memoria por tarjeta según NVML | 16 106 127 360 bytes |
| Controlador | NVIDIA 580.178.04 |
| Capacidad de cómputo y destino PTX | 7.5; `sm_75` |
| CPU de referencia | [Intel Xeon a 2,00 GHz](../../intel/xeon-kaggle-20261014/readme.md), cuatro procesadores lógicos visibles |
| Modelo | Qwen/Qwen3-8B-GGUF; archivo Qwen3-8B-Q4_K_M.gguf |
| Compilación | Rust 1.98.1, Linux x86-64 |
| Condición de red | Descarga y cotejo previos; Internet desactivado durante la inferencia |

Los identificadores físicos se conservan por sesión en los originales. No se mezclan los UUID de las series anteriores con los de la generación gobernada.

## Resultado principal

En la ampliación de generación gobernada hubo seis calibraciones y ocho solicitudes: **cinco GPU y tres CPU**. Las dos decisiones automáticas eligieron GPU. Todas las generaciones produjeron la misma secuencia de dieciséis tokens y terminaron por cota.

| Magnitud | Resultado |
|---|---:|
| Mediana CPU de tres calibraciones | 11,835012 s |
| Mediana GPU de tres calibraciones | 1,907238 s |
| Relación temporal CPU/GPU observada | Aproximadamente 6,21 |
| Lanzamientos de las cinco solicitudes GPU | 91 080; 18 216 por solicitud |
| Lanzamientos GPU de toda la serie | 146 487, incluidas calibración y paridad |
| Lanzamientos durante las tres solicitudes CPU | 0 |
| Comparaciones vectoriales recalculadas | 224 |
| Coseno mínimo / error relativo L2 máximo | 0,9996407932 / 0,0278590295 |
| Muestras periódicas / intervalo máximo | 473 / 304 ms |

Los tiempos de calibración corresponden al servicio con pesos preparados e incluyen observación y conservación. La preparación inicial duró 9,212797 s y se informa aparte. La relación temporal no se extrapola a otras cargas o condiciones.

## Realización y atribución

Las operaciones GPU examinadas se definen en Rust y se emiten como PTX. Se utilizaron la API del controlador CUDA, el controlador NVIDIA y NVML. No se delegó ese cálculo en CUDA C++ ni se utilizó cuBLAS/cuBLASLt. Esto **no significa independencia de CUDA** ni que el controlador esté escrito íntegramente en Rust.

La auditoría relacionó doce funciones con los programas PTX y sus lanzamientos. La CPU mantuvo tareas de control y transferencia durante las ejecuciones GPU. Potencia y energía corresponden a medidas globales de tarjeta, no a consumo exclusivo de la solicitud.

## Fuentes y límites

- [Informe completo y antecedentes](https://github.com/juantoniolloretegea/SV-motor/blob/32239f974af3a6367efd75d36ba984a6c2fc747e/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/qwen/qwen3-8b-cuantizacion-q4_k_m-gguf/sv-contraste-qwen-cpu-gpu-rust-20261014.md).
- [Auditoría GPU de la generación gobernada](https://github.com/juantoniolloretegea/SV-motor/blob/32239f974af3a6367efd75d36ba984a6c2fc747e/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/qwen/qwen3-8b-cuantizacion-q4_k_m-gguf/evidencias/gobierno-20261014/AUDITORIA-GPU.json).
- [Verificación de decisiones, vectores e integridad](https://github.com/juantoniolloretegea/SV-motor/blob/32239f974af3a6367efd75d36ba984a6c2fc747e/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/qwen/qwen3-8b-cuantizacion-q4_k_m-gguf/evidencias/gobierno-20261014/VERIFICACION-RUST.json).
- [Código exacto y reproducción](https://github.com/juantoniolloretegea/SV-motor/blob/32239f974af3a6367efd75d36ba984a6c2fc747e/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/qwen/qwen3-8b-cuantizacion-q4_k_m-gguf/codigo/gobierno-20261014/REPRODUCCION.md).
- [Selector común](../../selector-arbitro-director-cpu-gpu-tpu-npu/readme.md).

Es una comprobación breve de ejecución y cumplimiento de políticas. No demuestra saturación prolongada, concurrencia, migración entre GPU ni aptitud científica de Qwen. La sesión se detuvo después de conservar y cotejar los resultados.

[Volver a NVIDIA](../readme.md).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).

