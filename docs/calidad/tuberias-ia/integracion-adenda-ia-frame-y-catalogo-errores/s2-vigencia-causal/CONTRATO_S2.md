# S2 · Contraste causal de vigencia

RETP-2026-165 · 12/09/2026. Contrato y esperados anteriores a la ejecución. Sucesor acotado de S1; integración de adenda IA y frame, criterios C/I.

## Fuentes y regla aplicable

Cortes de entrada: Lenguaje `111b4e8fc24481cc76be347e724d027e1c636973`; laboratorio `35ea2ba7b397d7b8813be82ec8caeff4ff377bad`. AGENTS y las lecturas íntegras de Pilares, perfiles y transición se conservan; sus identidades se recotejan sin cambios en `BANCO_FIJADO.json`.

El perfil IE004-ES-P2/2 §5.3 exige resolver completamente el significado antes de aplicar política y vigencia; una ambigüedad no se resuelve por permiso y ninguna avería se convierte en U. `CONTRATO_RECEPCION_AV_1.md` §1 permite al conductor fijar la instantánea ordinaria o `revocado`. `fuentes/contrato.json` constituye P-IE004/1, que admite LEER/CASO-A/IGG; `BASE_PUBLICA.json` fija el literal artificial `8.40` para IGG/ACTUAL. La realización sucesora /3 conserva esa política. El diagnóstico `PERMISO_REVOCADO` procede de la puerta ya existente en `semantica-a/servicio.rs`; no se añade una regla de autorización.

Se ensayan dos instantáneas sintéticas. No se modifica una ejecución en curso ni se representa una revocación profesional, su momento de eficacia o su historia. La fuente de vigencia queda expresamente declarada por el conductor. V no la elige. S1 y su P3-04 se mantienen íntegros: su resultado ambiguo no alcanza esta puerta.

## Par y montaje fijados

Consulta: «El valor de la IgG.»; contexto LEER/CASO-A/IGG/ACTUAL/VALOR. El mismo archivo A01 se recibe dos veces, byte por byte, por el mismo custodio G1, una vez con cada vigencia. La única diferencia de entrada a la resolución es el booleano confiable; las identidades de invocación siguen siendo distintas.

| Vigencia | Estado exigido | Contenido | Ruta | Causas | Llamadas de política |
| --- | --- | --- | --- | --- | --- |
| true | DATO | `8.40` | `[1,1,1,1,1]` | 0 | 1 |
| false | PERMISO_REVOCADO | null | `[1,1,1,1,1]` | 0 | 1 |

La ausencia de contenido también exige fuente y alcance null en el cuerpo negativo; las versiones, admisión y contexto permanecen. Los cuerpos completos esperados se fijan antes de ejecutar. El positivo conserva el esperado histórico. El negativo aplica exclusivamente las diferencias declaradas a ese formato, conforme a la puerta de política y al contrato del cuerpo.

Para atravesar además enlace, lectura y cobertura se conservan los 24 originales del lote. P3-01 y P3-11 remiten a A01 y tienen iguales pregunta y contexto; sus IDs distintos se preservan, no se califican como solicitudes de bytes idénticos. `DECLARACION_VIGENCIA.json` constituye un montaje sucesor que cambia sólo la vigencia de P3-11 a false, con versión y procedencia nuevas. El enlace sucesor fija su huella; la cápsula anterior y su montaje permanecen recuperables. Se recorren las primeras once posiciones por el orden obligatorio del enlace; las nueve intermedias no amplían la cobertura causal. Los diez primeros cuerpos conservan sus esperados históricos.

## Reutilización y controles

La cápsula RETP-152 conserva sus 71 archivos originales. En la copia ejecutable sólo se sustituyen el montaje y las constantes de versión/huella de `lote-g1/lib.rs`; 69 archivos permanecen idénticos. Custodia, resolución, política, transporte, entrega y lectura no cambian. Se reutiliza la lógica de cobertura de S1, con versión y rangos de las dos posiciones nuevos. La referencia se obtiene del enlace antes de recibir la selección y exige el caso completo y su entrada exacta de montaje.

| Control | Exigencia previa |
| --- | --- |
| CA01 | P3-01: cuerpo positivo completo esperado |
| CA02 | P3-11: cuerpo negativo completo esperado |
| CA03 | Archivo A01 idéntico bajo ambas vigencias; mismos cuerpos que el par del lote |
| CA04 | Primeros diez cuerpos iguales a los esperados anteriores |
| CI01 | P3-11: cuerpo y dos citas íntegros, entrega con cobertura |
| CI02 | Mismo cuerpo y caso verdadero, montaje omitido: FaltaVigencia |
| CI03 | Montaje histórico frente al enlace sucesor: IdentidadMontaje |
| CI04 | Cita de vigencia de P3-01 para P3-11: VigenciaDistinta |
| CI05 | Caso omitido: FaltaCaso |
| CI06 | Identidad de otra invocación: Identidad |
| CI07 | Cambio de una letra dentro de PERMISO_REVOCADO, conservando JSON válido: ContenidoDistinto |
| CI08 | P3-01 completo: entrega con cobertura |

Dos sensibilidades limitadas: omitir la obligación de citar vigencia debe fallar en CI02; desactivar la condición de revocación debe fallar en CA02. Los mutantes no son candidatas entregables. Las comprobaciones de constructores privados ya acreditadas en S1 no se repiten, pues no cambian.

## Presupuesto, evidencia y parada

Una cualificación nativa: Rust 1.98.0, debug y release, tres ejecuciones por modo, doce controles por ejecución. Presupuesto máximo de 23 invocaciones: identificación del compilador; ocho compilaciones y seis ejecuciones de cualificación; ocho compilaciones/ejecuciones de las dos sensibilidades. Límite de 60 segundos por proceso. Las instancias del receptor son llamadas intraproceso, no procesos adicionales.

Se conservan argumentos, retorno, stdout, stderr, binario y fuentes identificados, pared y CPU cuando estén disponibles. RSS por proceso no está disponible en este entorno. Las solicitudes, marcos, trazas, selecciones, cuerpos y rechazos se conservan por bytes; las seis ejecuciones se cotejan. Los esperados y sus huellas se fijan antes de producir salidas. Un fallo detiene la recepción conforme; no se ajusta silenciosamente el espécimen o el esperado ni se amplía el presupuesto.

## Condición de salida y continuidad

La conformidad demostraría el efecto de vigencia sobre este encargo inequívoco y la exigencia de sus dos piezas documentales. No cerraría C/I universal, A–L, inscripción de dominio, autoridad profesional ni P4/P5/P6. La prueba externa conserva su recepción pendiente y la reserva P3 permanece cerrada. No se solicitan nuevas rondas ni se altera código productivo. Las causas se registran durante el recorrido; el catálogo se consolida después. Sucesos SV conserva el estado y el siguiente objeto de la integración en las mismas ramas.
