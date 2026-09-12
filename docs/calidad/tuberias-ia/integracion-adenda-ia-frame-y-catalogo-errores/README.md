# Integración de la adenda IA, el frame humano y el catálogo de errores

**RETP-2026-147 · 11/09/2026 · Watson · Continuación autorizada por Juan Antonio Lloret Egea.**

Este expediente reúne el punto 1 (adenda sobre integridad y trazabilidad de IA) con el punto 3 (reconstrucción, recibo y frame). El punto 2 (catálogo de errores y presentación ES/EN) recibirá sus fallos antes de cerrar su realización. Es un único trabajo dividido en ocho pasos; no abre dominios, agentes ni una segunda semántica.

## Respuesta a las dos frases

**«Ahora debemos conocer qué evidencia».** La referencia ya está aportada: seis imágenes, cinco cambios observables, una pregunta de identificación y una finalidad de conservación. La evidencia concreta es el original y cada versión, qué desaparece o se sustituye, qué conclusión sostiene la información conservada y qué se presenta finalmente al humano. En software se añaden las entradas efectivamente recibidas, operaciones observadas, versiones y vínculos recuperables. No hay que pedir al autor que vuelva a explicar esa idea.

**«Pero no demuestra que sepamos identificarla de forma suficiente».** La secuencia y las cinco comparaciones recuperadas muestran identificación de cambios en ese ejemplo. Permiten fijar testigos concretos de pérdida y conservación. La suficiencia para una operación se comprueba contra esos testigos y su referencia; no depende de multiplicar fotogramas. Esa demostración acotada no acredita todavía un reconocedor universal, la fidelidad de cualquier visualización ni seguridad clínica. Una reconstrucción de lo observado no acredita la causa interna de un modelo.

La última imagen ha sustituido el cráter por una cima cerrada y eliminado la lava: si se ofrece como representación fiel de la referencia, ha perdido rasgos esenciales del ejemplo. La ausencia de lava, por sí sola, no demuestra que un objeto no sea un volcán. El referente conocido y la fidelidad de su representación son juicios distintos.

[Memoria original con PDF, Markdown e imágenes](../frame-significado-humano-trazabilidad-y-fidelidad/README.md). No se altera esa memoria.

## Resultado de esta intervención

1. Correspondencia de las seis obligaciones de la adenda: fijada en la tabla siguiente.
2. Doce ataques A–L: convertidos en controles positivos, negativos y criterios de aceptación en [CASOS_Y_CRITERIOS.json](CASOS_Y_CRITERIOS.json). Todavía no ejecutados como campaña integrada.
3. Correspondencia entre recibo y actuación gobernada: delimitada en [G2_CORRESPONDENCIA_Y_LIMITE.md](G2_CORRESPONDENCIA_Y_LIMITE.md), con fuente concreta del requisito ausente.
4. Fallos a recibir por el catálogo: inventariados con sede y causa; sin inventar códigos SV ni confundirlos con `U`.

Esta intervención no ejecuta ensayos funcionales nuevos ni modifica código Rust. Se cotejan identidades de fuentes contra el árbol Git y se reutilizan resultados históricos con su alcance. El registro de fuentes distingue lectura, cotejo y resultado histórico.

## Los ocho pasos y su condición de salida

| Paso | Trabajo | Condición de salida | Estado de este corte |
|---|---|---|---|
| 1 | Unir adenda y frame | Cada obligación tiene sede, evidencia y pendiente explícitos | Correspondencia fijada |
| 2 | Fijar evidencia antes de probar | Cada caso tiene referencia, control, ataque, salida y causa de rechazo; antes de ejecución, montaje y presupuesto identificados | A–L fijados como criterios; montajes materiales todavía no preparados |
| 3 | Reconstruir y custodiar en Rust | Consulta: recibo completo recuperable; actuación: correspondencia y procedencia R1 admitidas | Recibo público realizado en RETP-144/145; enlace productivo sin contrato de recepción de autoridad |
| 4 | Atacar fidelidad | Pérdidas críticas detectadas y controles válidos admitidos sobre el mismo recorrido | Evidencia parcial histórica; campaña integrada A–L pendiente |
| 5 | Atacar seguridad y coste | Puertas funcionales, aislamiento y presupuestos aplicables satisfechos | P3 reservada, P4 y P5 pendientes; no se abre reserva |
| 6 | Consolidar catálogo y ES/EN | Causa técnica preservada desde emisor, contexto original y presentación versionada; DG01–DG14 y regresión pertinente conformes | Contrato previo recibido; inventario de este frente añadido; realización pendiente |
| 7 | Comprobar lo entregado | Correspondencia entre resultado, presentación efectiva y alcance humano comprobada | Pendiente; no se construye ahora una interfaz profesional |
| 8 | Consolidar laboratorio y Calidad | Mismas piezas, evidencia y límites; aceptación humana del resultado concreto | Este punto de control se conserva en ambos destinos; cierre integral pendiente |

No se suman aprobaciones de componentes como si acreditasen el conjunto. Los pasos preparatorios no se contabilizan como ejecución. El paso 6 recibe errores desde el inicio, aunque se consolide después de integrar 1 y 3.

## Correspondencia de la adenda

Fuente exacta: [§12, corte bbac1b44](https://github.com/juantoniolloretegea/SVperitus-dataset/blob/bbac1b44b1d3b845305e9cde492a08221206d631/dominios/ciberseguridad-inteligente/dominio-04-09-26/universos/OP-CYB-001/ACTA_CONTINUIDAD_Y_RELEVO_OP_CYB_001_AL_LENGUAJE_SV_2026_09_09.md#12-adenda-integridad-y-trazabilidad-del-consejo-asistido-por-ia). El corte anterior 169af16d termina antes de esa adenda. Se preservan ambos cortes; no se atribuye el texto nuevo al antiguo.

| Obligación recibida | Encaje existente y evidencia reutilizable | Qué queda por acreditar | Sede |
|---|---|---|---|
| Origen y separación datos/instrucciones/autoridad | G1 y lote-G1 conservan entrada, montaje y pertenencia; RETP-108 distingue declaración de autoridad y permiso protegido | Ataques directos/indirectos A/B; incorporación gobernada de referentes; imposición material | Recepción, contratos y núcleo R1; autoridad por sede competente |
| Cobertura y significado | Ejemplo visual; resolución pública A; CYB RETP-107 conserva F0/HS y detecta pérdida H | C/D/I sobre el recorrido integrado, acceso independiente a evidencia exigida y presentación final | Dominio fija suficiencia; Lenguaje conserva/comprueba; presentación conserva significado |
| Identidad de corpus, reglas, contexto y componentes | RETP-142 recupera 409 archivos y coteja 36 recorridos; RETP-145 liga lote y montaje exactos | G/H sobre toda dependencia relevante; montaje realmente cargado, truncación y episodios | Contrato de recepción y soporte |
| Rechazo, esquema, no admisión y fallo | G1 distingue pérdida de evidencia y respuesta funcional; V conserva causa de rechazo; contrato diagnóstico existente | F y propagación estructurada de cada causa hasta ES/EN | Emisor técnico, receptor y diagnóstico; no Tri.U |
| Confidencialidad y finalidad | Guardas públicas de acceso y montaje ficticio; no acreditan aislamiento ni secreto | E/L, registros y canales declarados, destinatario y finalidad autorizados, límites de retención | Contrato de uso y frontera material; no sólo léxico |
| Continuidad y revisión humana | Recibos y versiones recuperables en memoria; R1 conserva relación decisión/ejercicio intraproceso | J/K, reevaluación separada, objeto realmente mostrado y revisión exigida; persistencia si aplica | Recepción, presentación y fases materiales competentes |

RETP-107/108 y RETP-142/144/145 se citan por sus resultados publicados, no como campañas repetidas en 147. Sus denominadores no son intercambiables: seis imágenes no son seis ensayos R1; 24 posiciones públicas contienen diez preguntas distintas.

## Fallos que debe recibir el catálogo

Esta lista es una clasificación de obligaciones, no un nuevo catálogo normativo ni una asignación de códigos.

| Fallo | Dónde debe nacer su causa | Qué se debe conservar al presentarlo |
|---|---|---|
| Entrada truncada, esquema inválido, canal fallido | Lector o receptor que lo detecta | Operación, etapa, fuente y causa; no presentarlo como rechazo del dominio |
| Evidencia incompleta o de otra invocación | Custodio/enlace | Identidad esperada y recibida según vista autorizada; recibo no completo |
| Información exigida omitida | Comprobador de cobertura del contrato | Distinción ausente y operación afectada; no rellenar ni convertir en U |
| Cambio de sujeto, negación, condición o incertidumbre | Comprobador de correspondencia | Referencia y versión comparadas; localización de la pérdida |
| Solicitud sin autoridad constituida | Frontera de admisión gobernada | Falta del contrato/referente, distinta de permiso revocado o denegado |
| Versión, regla o componente sustituido | Verificador de identidad aplicable | Identidad exacta; un nombre igual no basta |
| Destinatario o finalidad no admitidos | Comprobador de uso antes de entrega | Decisión y causa con mínima exposición autorizada |
| Recursos agotados o efecto indeterminado | Componente que observa el fallo | Etapa y estado real; no éxito, U ni reintento tácito |
| Plantilla ES/EN ausente o inválida | Presentación diagnóstica | Causa original intacta; fallo de presentación separado |

El contrato RETP-109/110 ya exige causa cerrada emitida en origen, perfiles explícitos, localización sobre bytes originales y separación de decisión y texto. No se recuperará una causa buscando palabras en `Debug`. No se traducirán identificadores ni datos del dominio. El cierre exige inventariar emisores alcanzables y comprobarlos; esta tabla no lo sustituye.

## Alcance y continuidad

Se conservan workflow V2 RETP-141, etapas P0–P6, cupos ya consumidos, reserva cerrada y asociación /2–/3 pendiente. Los casos A–L no reinician rondas ni conceden presupuesto. La autorización de trabajo no decide por sí sola esa asociación ni una promoción final.

La lectura pública termina legítimamente en resolución y evidencia recuperable. Si se pretende una actuación gobernada o inscripción, el requisito pendiente es el contrato de recepción de autoridad y la operación constituida correspondiente, no otra explicación del volcán. G2 identifica exactamente esa dependencia; no obliga a inventar un agente ni a elegir células.

Rectoras consultadas completas: AGENTS; Pilares RETP-073; perfiles RETP-075; acta de transición con §§12–30. Se reciben workflow V2, expediente RETP-142, acta RETP-145, contrato CYB con RETP-108 y contrato diagnóstico. Cortes: Lenguaje `e8558ea69aadacc3454cbd82f09a7bf7f4b06648`; laboratorio `53e1b93f2a392b2bac6e229be839e3cc2e4823ee`; adenda CYB `bbac1b44b1d3b845305e9cde492a08221206d631`.

**Dictamen: correspondencia y criterios documentales fijados; trazabilidad pública recibida con sus límites; enlace productivo, seguridad integral y catálogo/localización no cerrados.**


## Revisión de secuencia y relevo · RETP-162 · 12/09/2026

La [revisión de cobertura y secuencia](REVISION_SECUENCIA_COBERTURA_Y_RELEVO_RETP_162.md) sucede al relevo inmediato del catálogo, conservando todos los cortes históricos. Se restituye como prioridad la integración del punto 1 (integridad/trazabilidad IA) y el punto 3 (reconstrucción/recibo/frame), con recogida de errores durante el recorrido y consolidación posterior del punto 2 (catálogo/localización).

Los avances RETP-149/150/152 y 153/154/157–161 se reutilizan según sus contratos; no se suman como cierre de A–L. La [matriz revisada](MATRIZ_COBERTURA_A_L_RETP_162.json) conserva los doce criterios originales y localiza sus carencias. Siguiente objeto único: C/I, cobertura de evidencia requerida con acceso independiente del comprobador, incluyendo una selección que omite una dependencia relevante aunque sus citas sean ciertas. Preparar montaje y esperados antes de ejecutar; reutilizar las capas disponibles y corregir sólo una pérdida demostrada.

La actuación profesional sin premisa/verificadores sigue inhabilitada, sin bloquear toda lectura documental ni reabrir quién autoriza. La reserva P3 y sus compromisos permanecen intactos; custodia humana ya recibida en RETP-130. P4/P5, auditoría P6 y retorno a fila 9 conservan sus condiciones. La prueba externa común del alcance integrado sigue pendiente, distinta del encargo público de reproducción RETP-157. Las conversiones defensivas y el resto del catálogo conservan su deuda, pero dejan de ser el siguiente objeto automático. Revisión documental sin nueva ejecución funcional, nuevas ramas, renumeración ni promoción productiva.


## Recepción S1 y relevo S2 · RETP-164 · 12/09/2026

El [contraste S1 de cobertura independiente](s1-cobertura-independiente/README.md) acredita en dos posiciones del banco la comprobación de las dos piezas documentales requeridas, con ocho controles en seis ejecuciones, dos sensibilidades y conservación de los originales. La cápsula RETP-152 se reutiliza íntegra. El intento instrumental inicial y el espécimen incorrecto de CI07 se conservan junto con su corrección explícita; no se cuentan como resultados conformes.

S1 finaliza en ese alcance. C/I y A–L no cierran íntegramente: P3-04 no prueba una revocación aplicable, porque su petición ambigua evita consultar la política. S2 queda pendiente para fijar el par causal de una consulta inequívoca con vigencia positiva y negativa, con fuente y montaje declarados antes de ejecutar. Se conserva la prioridad de integrar 1+3; las causas alimentan el catálogo durante el recorrido y su consolidación permanece posterior. Prueba externa, reserva, obligaciones profesionales y compuertas del workflow conservan sus condiciones. El registro obligatorio Sucesos SV identifica la actividad vigente.


## Recepción S2 y relevo S3 · RETP-165 · 12/09/2026

El [contraste S2 de vigencia causal](s2-vigencia-causal/README.md) acredita en este banco que una consulta idéntica y de significado único entrega DATO/8.40 con vigencia positiva y PERMISO_REVOCADO/null con vigencia negativa; ambas consultan una vez la política. La omisión de la pieza de vigencia impide acreditar cobertura documental. Doce controles en seis ejecuciones, dos sensibilidades y diez cuerpos anteriores conservados; 23 invocaciones dentro del presupuesto fijado. Montaje sucesor explícito; S1 y P3-04 mantienen su evidencia histórica.

S2 finaliza en alcance sintético nativo. La vigencia profesional y el cierre universal C/I siguen fuera de lo acreditado. S3 queda pendiente para precisar el objeto presentado y el testigo de pérdida de negación del criterio D, reutilizando RETP-149/150/152 y evitando contar de nuevo sus pruebas. Después corresponde preparar la prueba externa común del recorrido integrado con alcance y esperados fijados; su recepción continúa pendiente.

Se mantiene la secuencia: integración 1+3; recogida continua de causas; consolidación posterior del paso 6, catálogo y ES/EN. Las causas S2 se conservan en el expediente con su etapa. Resto A–L, reserva P3, P4/P5/P6 y aceptación humana integral mantienen sus condiciones. Mismas ramas y registro obligatorio Sucesos SV.


## Recepción S3 y relevo S4 · RETP-166 · 12/09/2026

El [contraste S3 de presentación y negación](s3-presentacion-y-negacion/README.md) reutiliza el rechazo previo de RETP-150 y prueba su composición con lectura, cobertura y archivo escrito/recuperado. La pérdida de negación previa impide crear el archivo; la posterior se detecta al comparar los bytes observados mientras la referencia permanece intacta. Ocho controles, seis ejecuciones, dos sensibilidades y 22 invocaciones; 73 fuentes S2 intactas. El cierre se limita a texto y proceso confiable: no acredita pantalla, revisión humana, imposición frente al host ni cierre universal D/C/I o A–L.

S4 queda pendiente: preparar la prueba externa común del recorrido documental acotado, con una misma instrucción, fuentes públicas accesibles, esperados previamente custodiados y formato de recepción comparable. Banco público conocido; sin presentarlo como prueba ciega o reserva inédita. No se han enviado encargos ni recibido resultados externos en S3.

La prioridad sigue siendo la integración 1+3, con recogida continua de causas. El paso 6, catálogo y ES/EN, mantiene su consolidación posterior. Los demás criterios y las compuertas de P3/P4/P5/P6 y aceptación humana conservan sus condiciones. Sucesos SV y las mismas ramas mantienen la continuidad.


## Recepción S4 y relevo S5 · RETP-167 · 12/09/2026

[Prueba común publicada en SVcustos](https://github.com/juantoniolloretegea/SVcustos-dataset/blob/e1dcd9f5df30a1bbbfd446c7c4e5578d0a08a75c/pruebas-externas/s4-recorrido-documental/PRUEBA_COMUN.md), autosuficiente y descargable sin credenciales; mismos archivos para todos. Doce resultados previos cotejados, oráculo comprometido, rúbrica y mediciones fijadas. S4 finalizado como preparación; S5 pendiente para recibir respuestas originales, confirmar lectura y evaluar casos, trazabilidad y procedimiento. No se atribuyen aún resultados externos. Banco público conocido, sin reserva inédita; tiempo y recursos separados de la nota documental. Se mantienen integración 1+3, causas continuas y consolidación posterior del catálogo y demás compuertas.


## Recepción S5 · RETP-168

[DeepSeek, Claude y Qwen recibidos](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/tuberias-ia/integracion-adenda-ia-frame-y-catalogo-errores/s5-recepcion-externa/README.md), originales preservados y cotejo inicial registrado. Grok pendiente de encargo efectivo. S5 en ejecución; etiquetas reservadas para revisión común, sin clasificación de rapidez ni nota definitiva. Continúan integración 1+3 y registro de causas; consolidación del catálogo posterior.


## Cuarta recepción S5 · RETP-169

> **Relevo vigente · S5 / RETP-169:** [Grok recibido y cotejado](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/tuberias-ia/integracion-adenda-ia-frame-y-catalogo-errores/s5-recepcion-externa/grok/README.md); cuatro originales disponibles. S5 en ejecución para revisión común y aceptación. Las entradas anteriores conservan sus cortes históricos.



## Segundo intento cualificado · RETP-170

> **Relevo vigente · S6/S7 · RETP-170:** [segundo intento cualificado](https://github.com/juantoniolloretegea/SVcustos-dataset/blob/fccde9cf524a0d62b2dd1a2ee05170d6f4358074/pruebas-externas/s6-trazabilidad-total/PRUEBA_COMUN.md). S6 finalizado; S7 pendiente de recepción. Exigencia íntegra y aviso de exclusión definidos de antemano; 482 controles previstos satisfechos. Las entradas anteriores conservan su corte histórico.



## Consolidación de recepciones y retorno a integración 1+3 · RETP-178

> **Relevo vigente · S10:** 2026-09-12T16:56:53Z. [Acta y ranquin](https://github.com/juantoniolloretegea/SV-lenguaje-de-computacion/blob/main/docs/calidad/Inventario-sv/ranquin-ias-trazabilidad/ACTA_RESULTADOS_Y_RANQUIN_2026_09_12.md): cinco participantes y diez entregas recibidas, con 120 casos. Grok y Claude conformes documentalmente en la segunda ronda, con exposición alta de Claude; Qwen aceptado provisionalmente por Dirección y original NO_CONFORME conservado; DeepSeek con defecto de fidelidad; Mistral descartado por Dirección tras segunda entrega NO_CONFORME. Las menciones anteriores a recepciones pendientes conservan sólo su corte histórico.

Se puede retomar la integración del punto 1 (integridad/trazabilidad IA) con el 3 (reconstrucción/recibo/frame). S1, S2 y S3 se reutilizan dentro de su alcance; S4–S9 añaden contraste externo documental. Siguiente trabajo: reconciliar la matriz A–L con estas evidencias y fijar el siguiente hueco integrado, conservando objeto presentado, invocación, cobertura y referencia independiente; montaje, controles, ataques, esperados y presupuesto antes de ejecutar. No se abre otro torneo como condición previa. La consolidación del catálogo y ES/EN permanece posterior, con recogida continua de causas.

No se acredita cierre A–L, pantalla o revisión humana, imposición frente al host, persistencia ni actuación profesional. Se conservan reserva P3, custodia RETP-130 y condiciones P4/P5/P6. Esta acta no modifica el núcleo ni promueve una implantación.
