# Registro de partes de trabajo por agente

## Finalidad

Este registro deja constancia mínima, verificable y comparativa del trabajo realizado por cada agente sobre un árbol del repositorio verificado determinado.

## Campos clave

- `Agente`: siempre en formato `Agente X`.
- `Base_Verificacion`: grado real de lectura empleado antes de actuar.
- `Actuaciones_Desde_Ultimo_Control_Secuencial`: resumen de lo ocurrido desde el último barrido formal del árbol.

## Regla operativa

- No debe declararse una lectura `VERIFICACION_INTEGRAL` si el agente no ha recorrido realmente el árbol del repositorio verificado relevante para su dictamen.
- Las actuaciones globales de fase o de registro maestro deben apoyarse en lectura `VERIFICACION_INTEGRAL`.
- Las actuaciones locales pueden apoyarse en lectura `VERIFICACION_ACOTADA`, pero deben declararlo expresamente.

## Regla de no repetición derivada de la continuidad operativa asociada a WBeta SV-UCBC8

1. Todo archivo `.zip` subido por el responsable del proyecto debe inspeccionarse realmente por dentro antes de emitir juicio, aunque no se diga expresamente, presumiendo de entrada que puede tratarse de un repositorio verificado o de un parche material relevante.
2. En caso de tensión entre nombre externo del archivo, contenido interno real del ZIP, clon local, PDF, árbol del repositorio verificado o una reconstrucción no verificada, **manda el contenido material verificable más reciente**, no una memoria referencial no verificable.
3. Ningún agente debe tratar como “crear” un archivo cuya existencia real en el árbol del repositorio verificado no haya verificado antes con lectura suficiente del árbol.
4. La lectura `VERIFICACION_INTEGRAL` obliga a revisar el objeto material subido, no sólo a reconstruir el estado por continuidad documental operativa.
5. Cuando un agente cometa una inferencia indebida o un error de lectura material, debe dejar constancia explícita para prevenir repetición por agentes posteriores.

## Preservación del histórico detallado

El detalle humano completo de `PTA-2026-001` a `PTA-2026-009` queda preservado sin modificación en:

`docs/calidad/historico/REGISTRO_PARTES_DE_TRABAJO_POR_AGENTE_HASTA_PTA_2026_009.md`

El CSV `REGISTRO_PARTES_DE_TRABAJO_POR_AGENTE.csv` conserva la serie tabular completa y su continuidad. Este archivo mantiene desde ahora una lectura humana compacta del tramo vivo, sin eliminar el histórico anterior.

## Tabla maestra de partes

| Parte ID | Fecha | Hora | Agente | Lectura | Alcance declarado |
|---|---|---|---|---|---|
| PTA-2026-001 | 23/03/2026 | 16:59:50 | Agente WLenguaje7 SV | VERIFICACION_INTEGRAL | Control adicional de barridos y base de verificación. |
| PTA-2026-002 | 23/03/2026 | 17:45:05 | Agente WLenguaje7 SV | VERIFICACION_INTEGRAL | Auditoría H2-A y apertura restringida de H2. |
| PTA-2026-003 | 23/03/2026 | 20:02:35 | Agente WLenguaje8 SV | VERIFICACION_INTEGRAL | Parche correctivo funcional acotado del frontend. |
| PTA-2026-004 | 24/03/2026 | 08:48:50 | Agente WBeta SV-UCBC8 | VERIFICACION_INTEGRAL | Primer paquete VII y reglas de no repetición. |
| PTA-2026-005 | 25/03/2026 | 07:15:00 | Agente WBeta SV-UCBC9 | VERIFICACION_INTEGRAL | Consolidación VII hasta VII.3 y prueba de estrés al Lenguaje. |
| PTA-2026-006 | 25/03/2026 | 21:07:32 | Agente WBeta SV-UCBC10 | VERIFICACION_INTEGRAL | Consolidación VII.4–VII.5 y continuidad hacia VII.6. |
| PTA-2026-007 | 26/03/2026 | 09:55:00 | Agente WBeta SV-UCBC11 | VERIFICACION_INTEGRAL | Sellado técnico mínimo prebackend. |
| PTA-2026-008 | 26/03/2026 | NO_CONSTA | Agente WBeta SV-UCBC12 | VERIFICACION_ACOTADA | Asentamiento UCBC12 y piloto de seguridad estructural. |
| PTA-2026-009 | 16/08/2026 | 06:03 | Agente Watson Publicaciones-Lenguaje | VERIFICACION_INTEGRAL | Aprendizaje trazable y preservación pre-DSL. |
| PTA-2026-010 | 18/08/2026 | 18:49:00 | Agente Watson Lenguaje SV | VERIFICACION_INTEGRAL | Reentrada Ruta A, auditoría FFL-A/FFL-B, microcierres E112/E113/E307, reversión E406 y reparación registral. |
| PTA-2026-011 | 24/09/2026 | 13:06:58 | Agente Watson / W-S39-02 | VERIFICACION_ACOTADA | Recepción S39, diagnóstico MXFP4 y recuperación OneCloud. |

## PTA-2026-010 — Agente Watson Lenguaje SV

- **Fecha:** 18/08/2026  
- **Hora (Europe/Madrid):** 18:49:00  
- **Lectura del árbol del repositorio verificado:** `VERIFICACION_INTEGRAL`.  
- **Alcance declarado:** reentrada controlada del Lenguaje tras Ruta A; auditoría del trabajo heredado en FFL-A/FFL-B; materialización y contraste de E112, E113 y E307; reversión íntegra del intento E406 no mínimo; reparación y sincronización del control de calidad antes de rehacer el programa operativo.  
- **Base de repositorio verificada:** `main` fresco de `SV-lenguaje-de-computacion` desde la reapertura del 18/08 hasta el estado posterior a la reparación registral; acta de reapertura; tablero FFL; IR v0.2; catálogo diagnóstico; validator; suite declarativa; matrices/crosswalk/deuda; `docs/calidad`; Dinámica del Suceso como fundamento superior ya cerrado.  
- **Actuaciones desde el último control secuencial:** desde PTA-2026-009 se cerraron las compuertas matemáticas previas, se aprobó Ruta A y otra unidad avanzó FFL-A/FFL-B. Al asumir el frente se detectó una cadena de commits válida en parte, un parche E406 incompleto/no mínimo y un desfase grave: RETP, BARR y PTA no reflejaban la actividad del 18/08.  
- **Actuaciones ejecutadas:** lectura material del repositorio fresco; auditoría de los commits heredados; ratificación de FFL-A bajo Vía B; microcierres E112/E113/E307 bajo radio corto; reversión completa del intento E406; contraste editorial y conceptual de dependencias; revisión de interfaces y precursor del Panel del Experto; reparación de RETP CSV/MD; ejecución del barrido BARR-2026-006 y alta del presente parte.  
- **Artefactos leídos:** acta de reapertura; tablero FFL; IR v0.2; `src/svp_errors.py`; `src/svp_validator.py`; `tests/run_conformance.py`; catálogo de errores; matrices/crosswalk/deuda de `docs/calidad`; registros RETP/BARR/PTA; sede doctrinal pertinente; `SVperitus-dataset` sólo para contraste prospectivo del Panel del Experto.  
- **Resultado:** FFL-A queda trazablemente cerrado bajo Vía B; FFL-B es el único frente técnico activo; E112/E113/E307 están materializados; E406 no está aplicado y sólo podrá reabrirse mediante nueva microauditoría y diff mínimo; los registros de calidad vuelven a sostener una reentrada rápida.  
- **Observaciones:** no se declara una ejecución nueva de la suite global. No se consideran el estado editorial externo de publicaciones en `U`, el saneamiento de espejos ITVIA/GitHub, las interfaces futuras, el Panel del Experto, el manual/wiki/diccionario ni `NL→SVP` como bloqueos del FFL-B inmediato.

## PTA-2026-011 — Agente Watson / W-S39-02

- **Registro:** 2026-09-24T11:06:58Z; 13:06:58, Europe/Madrid.
- **Base:** VERIFICACION_ACOTADA, limitada a S39/TT-0012 y sus registros; corte Lenguaje `d19bb1b33d5dea39926c1861aab5198e9556f186` y [recepción Motor](https://github.com/juantoniolloretegea/SV-motor/blob/a74632b0b6dde70629863113134082dc3f31d521/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/resultados/recuperacion-onecloud-2026-09-24/INFORME.md).
- **Actuación:** lectura material de partes, Sucesos, historial, RETP, tique, actas y accesos; conciliación de intentos y recuperación; inscripción del relevo y de la revisión 16; remisiones a evidencia fijada a commit.
- **Desfase recibido:** S39 ya alcanzaba revisión 15, RETP terminaba en 268 y PTA en 010. Se registra la recepción hoy; no se certifica el intervalo global entre partes ni se crean asientos ficticios.
- **Resultado:** recuperación y arranque básico comprobados; regresión CPU recibida como una prueba aprobada, 340 filtradas; inferencia completa de la candidata pendiente.
- **Límites:** no hay inferencia nueva, modificación doctrinal, auditoría global o cierre de obligaciones ajenas. Ejecuciones heredadas conservan su atribución. Continúa S39 con W-S39-02; TT-0012 permanece abierto.
- **Referencias:** RETP-2026-269; Acta 004 §16; PTA-SVM-002. S42/TT-0010 y cierre parcial Qwen/B conservados.

**Límite del control registral:** El control detecta identificadores repetidos preexistentes en RETP (163, 164, 165, 166, 167 y 170); algunos representan apertura y recepción de un mismo frente. Se preservan sus filas y no se diagnostican ni renumeran sin estudiar su historia. RETP-2026-269 aparece una sola vez. Esta entrega no acredita unicidad global del registro histórico.

## PTA-2026-012 — Agente Watson / W-S39-02

- **Registro:** 2026-09-24T11:47:42Z; 13:47:42, Europe/Madrid.
- **Base:** VERIFICACION_ACOTADA sobre Lenguaje c68020992d19b041574992355de023961f6713d6 y [resultado Motor](https://github.com/juantoniolloretegea/SV-motor/blob/70750a516001cf13314176c529508d0712b7f3c1/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/openai/gpt-oss-20b/resultados/onecloud-2026-09-24/RESULTADO.md).
- **Actuación:** lectura de rectores y condición de cierre; recepción de guardas, identidades y dos inferencias; contraste con originales; actualización S39 revisión 17, TT-0012 e índice, historial, RETP, actas y accesos.
- **Resultado:** OC-01 responde correctamente al caso 3 + 2; OC-02 conserva texto inconexo del anterior. Objetivo material de TT-0012 conseguido y tique finalizado con alcance explícito; S39 sigue abierto.
- **Límites:** un caso por ejecutable, construcciones distintas, rendimiento limitado y contadores finales de cgroup no disponibles. Sin prueba de calidad general, integración ni causalidad exclusiva.
- **Trazabilidad:** RETP-2026-270; Acta004 §17; PTA-SVM-003. No se atribuyen estos resultados a partes históricos ni se declara barrido integral.
