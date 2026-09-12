# RETP-158 · Plan previo de comprobación

Corte de Lenguaje: 4cacf3ec6bd5d0f31206197c7374515a56b34a89. Laboratorio: 1ac32131e2c328e395d828268a5e3517ce43fcca. Base experimental: cápsula RETP-157. Se han consultado completos los pilares, el acta de perfiles, el acta de transición IMM y el workflow V2; sus bytes se han comprobado contra el corte vigente. Contrato de diagnósticos 109/110 leído completo.

## Alcance

Paso 6 del plan de integración: conservar fuera de la IR la unidad original, su perfil ES/EN, nombre, SHA-256 y rangos de bytes. Estructurar en el emisor E004 (vacío/repetición), E115 (repetidas/ausentes/ajenas) y colisión de identificadores. Conservar EOF en la unidad responsable. Ninguna causa se deduce de Debug ni del texto de un error. Los emisores no migrados se declaran expresamente pendientes. Un único recorrido alimentará las API detallada y heredada.

## Esperados fijados antes del cambio

- El banco canónico conserva 14 fuentes válidas y 106 inválidas; el resultado heredado debe coincidir byte a byte con la candidata RETP-157, tanto en éxitos como en rechazos.
- E004 vacío y E004 repetición conservan causas distintas y el mismo código canónico.
- E115 conserva separadas las listas de claves repetidas, ausentes y ajenas; la forma autónoma no inventa un codominio.
- Las colisiones señalan las dos declaraciones, aun con el mismo nombre de archivo y perfiles distintos. No se asigna un código SV no constituido.
- EOF tiene rango len..len de su unidad; los rangos se expresan en bytes originales incluso con UTF-8, CRLF y tabulaciones. El contexto no se obtiene reconstruyendo la IR.
- Una fuente ES recibe mensaje ES; una EN, EN; un ensamblaje mixto dispone de ambos. La localización no cambia el juicio ni las listas tipadas.
- La presentación ordinaria no incorpora literales de semántica ni el texto entero de las fuentes. Se conserva la compatibilidad explícita del error heredado.
- Las pruebas previas y las nuevas deben pasar en debug y release. Los casos focales se repiten tres veces por modo; la repetición no sustituye la corrección.

## Límites y conservación

Una candidata causal y una cualificación acotada; los fallos y sus correcciones se conservan. Sólo destino nativo disponible. P3 reservado continúa cerrado; P4, P5, WASI/browser, resto de emisores y productor profesional no quedan acreditados por estas pruebas. No se publica el laboratorio privado ni se abre una ruta productiva de autoridad. Este paquete es reproducción y auditoría; todavía no es un examen ciego entre modelos.
