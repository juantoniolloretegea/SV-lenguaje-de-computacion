# Corrección del instrumento, posterior a la cualificación

Las 18 ejecuciones de la candidata se realizaron con la versión exacta conservada en `reproducir_01.py`. Su SHA-256 coincide con `evidencia/IDENTIDAD.json`. Todos esos procesos terminaron normalmente dentro del presupuesto; los cuatro clientes negativos tuvieron los rechazos previstos.

Durante la revisión posterior se localizó por inspección un defecto del instrumento: al cerrarse stdout y stderr, el bucle de vigilancia terminaba y llamaba a `wait()` sin tiempo límite. Un proceso que cerrase ambos descriptores pero siguiera vivo podía superar el presupuesto. No se atribuye ese comportamiento a las ejecuciones ya observadas, ni se afirma haber ejecutado deliberadamente una espera infinita con la versión anterior.

`reproducir.py` incorpora ahora el tiempo restante a esa espera; al excederlo termina el grupo de procesos y conserva `TIEMPO`. Las causas de la candidata, sus fuentes y su oráculo no se han cambiado. No se ha repetido la cualificación para mejorar un resultado.

`comprobar_limite.py` extrae por AST las funciones reales de la versión corregida, sin lanzar la campaña SV. Con un presupuesto abreviado de 0,3 segundos comprueba:

| Caso | Esperado | Observado |
| --- | --- | --- |
| Proceso vivo con salidas abiertas | Terminación por tiempo | `TIEMPO`, código -9, 0,311553184 s |
| Proceso vivo con ambas salidas cerradas | Terminación por tiempo | `TIEMPO`, código -9, 0,300661448 s |
| Flujo de salida excesivo | Terminación por tamaño | `SALIDA`, código -9, 0,021317001 s |

La captura conserva como máximo 2 MiB por flujo e indica el exceso; no afirma conservar los bytes posteriores al límite. El tiempo observado incluye la resolución del temporizador, la terminación y la captura; no es una garantía de tiempo real exacto.

La evidencia está en `../evidencia/instrumento/`. PROCESOS se conserva íntegro comprimido en gzip, con SHA-256 del contenido descomprimido. La instrumentación no acredita aislamiento material del servicio, de sus descendientes que escapasen del grupo ni las compuertas P4/P5. La distinción de versiones impide atribuir a la prueba original una protección que se añadió después.
