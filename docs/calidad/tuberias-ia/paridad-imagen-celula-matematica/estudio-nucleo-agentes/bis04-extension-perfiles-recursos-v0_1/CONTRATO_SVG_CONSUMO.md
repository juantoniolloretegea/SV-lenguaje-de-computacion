# Contrato experimental de representación SVG y consumo local

**S22 · RETP-2026-238 · 14/09/2026 · candidato de laboratorio.**

## Fuente y sede

Corte Lenguaje `58f415b6c183b84df238122fa2ef5ea6d5d518e1`; laboratorio `b361da80e5cf6c10d89298b18170905a6a35dc48`. Se reutilizan [C04](../bis-c04/CONTRATO_PARIDAD_CANDIDATO_v0_1.md), [C05](../bis-c05/CONTRATO_CONSUMO_CANDIDATO_v0_1.md), el [montaje admitido](../bis04-realizacion-i0205-v0_1/README.md) y el [relevo RETP-237](RELEVO_REPRESENTACION_CONSUMO.md). Es una pieza técnica sucesora para este incremento, subordinada a esos contratos. No cambia la gramática, IR, semántica, inventario de soporte ni constituciones de dominio/agente.

Pilares, acta de perfiles/contratos/ensamblaje y acta de transición se leyeron completos en esta intervención; se ha comprobado que conservan sus bytes frente al precompromiso anterior. Se consultaron la decisión de sedes y su interfaz, el código de admisión/geometría/entrega y el código y resultado S14. El alcance de esta inspección es concreto: el montaje entrega el descriptor y S14 discrimina una base documental no utilizada; ninguno de esos dos componentes produce y consume este SVG. Se reutilizan sus interfaces y criterio de falsación sin repetir sus campañas ni sustituir el núcleo.

La nueva sede es un auxiliar Rust externo que recibe `AdmittedDelivery` por su API pública. `Rendered` tiene campos privados y accesores inmutables. La entrada del consumidor se captura mediante un lector real de archivo, después de materializar los bytes producidos por el representador. No se sustituye ese archivo por el SVG esperado en los controles positivos.

## Perfil material y precisión

El perfil `SV-SVG16-MATERIAL/1` admite únicamente el descriptor ya admitido de 16 posiciones y transformación `identidad` del montaje sintético. Esta limitación no reduce el soporte del Lenguaje ni convierte 16 en valor predeterminado. Otros tamaños y transformaciones requieren otro incremento explícito.

La representación usa un SVG restringido: cabecera, título, descripción, polígono de 16 pares ordenados y leyenda fijos. No admite contenido XML general, scripts, entidades, referencias externas, estilos libres ni recursos de red. `data-orden` conserva P1…P16; P1 corresponde a +X. El cierre lo expresa el elemento `polygon`; no se añade una posición ternaria.

La proyección `micro16-pantalla/1` aproxima sólo coordenadas gráficas. Conserva fuera del SVG el descriptor exacto, el vector y la identidad de origen. Escala: 1000000 unidades de SVG por unidad canónica. Las direcciones de un cuadrante usan las constantes enteras 1000000, 923880, 707107, 382683 y 0; signos y posiciones se declaran explícitamente para los dieciséis ejes. Para radio r se producen X=r·C y Y=−r·S. No hay trigonometría flotante, normalización de estados, interpolación semántica ni redondeo de cardinalidad durante la ejecución.

Las aproximaciones C/1000000 y S/1000000 se fijan antes del ensayo. Mediante las identidades radicales de π/8 y π/4 y desigualdades de cuadrados en `i128`, el preparador comprueba un error menor que 1/1000000 por componente de dirección; para r≤3, menor que 3/1000000 por coordenada canónica. Esta cota geométrica no es una cota de píxeles ni acredita rasterización, antialiasing, legibilidad o percepción. La escritura SVG es exacta respecto de los enteros comprometidos y aproximada respecto de la poligonal canónica. No se atribuye equivalencia exacta a esta proyección.

El `viewBox` es −4000000, −4000000, 8000000, 9000000; viewport declarado 320×360. Son parámetros exclusivos de este artefacto de prueba, sin selección de GUI. La leyenda conserva 0→radio 1, 1→radio 2 y U→radio 3. No se dibuja `Tri::as_u8()` como radio.

## Identidad, lectura y operación

La envoltura conserva íntegros `CellIdentity` y el contexto de entrega del descriptor, su SHA-256, el perfil de proyección, operación, consumidor, longitud y SHA-256 del SVG. El consumidor específico se identifica como `consumidor-svg16-local/1`; no se atribuye al receptor histórico la ejecución de esta operación nueva. La identidad y la invocación de origen permanecen enlazadas sin cambiar su significado ni simular una transición de Frame.

La expectativa material es el objeto privado producido por el representador confiable del montaje; la envoltura ofrecida no puede modificarlo. La cualificación contrasta ese productor con una envoltura y un SVG de custodia independientes fijados antes de ejecutarlo. Los hashes no autentican por sí mismos al productor ni sustituyen esa comparación.

Precedencia: canal ausente → contexto discordante → lectura acotada/fallo E/S → bytes distintos del artefacto autorizado → perfil SVG inválido → operación aritmética. La lectura permite 4096 bytes y un byte detector de exceso; un rechazo de contexto no lee el canal. La ausencia de canal es distinta de un canal presente vacío. El registro distingue el intento de recepción del consumo completado.

El lector SVG reconoce sólo la escritura comprometida, dieciséis pares de enteros canónicos con magnitud ≤3000000 y la plantilla exacta. Sobre esas coordenadas efectivamente interpretadas calcula las dieciséis diferencias cíclicas entre vértices consecutivos. El resultado conserva vértices y diferencias; no se reconstruye desde el vector ternario original. Esta es la operación material acreditable, no comprensión de una imagen ni consejo profesional.

Cuotas: SVG 4096 bytes, 16 vértices, dos coordenadas por vértice, registro JSON de ensayo ≤65536 bytes, una ejecución concurrente, sin red ni reintentos. Recepción/admisión del descriptor conservan las cuotas anteriores. Las multiplicaciones y restas del candidato son comprobadas; la variante optimizada mantiene esas comprobaciones. No se constituye una cuota universal de memoria ni tiempo.

## Oráculos y falsación

El [SVG literal](ORACULO_SVG_CONSUMO.svg) y los [vértices y diferencias](ORACULO_SVG_CONSUMO.json) se escribieron desde el convenio y el vector del fixture. El preparador coteja la aritmética de la tabla y deriva las referencias de custodia sin enlazar ni invocar al representador o consumidor. El observador compara los bytes y objetos completos con esos ficheros, sin pedir al candidato que determine su propio éxito.

Se compilarán dos mutantes: `svg_permutado`, que intercambia dos posiciones y conserva una envoltura coherente con su salida errónea; y `consumo_falso_svg`, que conserva la captura y declara su hash correcto, pero calcula con un SVG alternativo. Deben compilar, ejecutar el control RC01 y producir discrepancia funcional con retorno 1 del banco. Un fallo de compilación, pánico o ausencia de resultado no cuenta como detección.

## Límites y revisión crítica previa

Un SVG material sigue sin acreditar píxeles. El lector restringido no es un motor gráfico. La prueba de script corresponde al lector aislado y no se declara prueba de ataque a toda la cadena. El control de consumo falso discrimina la dependencia de la operación instrumentada; no demuestra percepción por una IA, lectura humana, autorización profesional, aislamiento del proceso o resistencia a un anfitrión malicioso.

Se evita una falsa prueba de consumo pasando al lector el archivo efectivamente generado en las rutas positivas. Se evita la circularidad del esperado preservando las tablas y el SVG de custodia independientes. Se conserva la distinción de instancia/revisión aun con bytes iguales. No se promueve la coherencia entre artefacto y su propio hash a paridad con el estado. Los dos mutantes cubren precisamente esas insuficiencias posibles del montaje confiable.

Las 202 filas originales C02–C12 y los escenarios globales Bis conservan sus recuentos. Los nuevos RC son un incremento propio. S26/F01/F02 sigue pendiente de interfaz material; Bis continúa abierto. Después corresponderá contrastar el paso de SVG a representación rasterizada y su captor según un contrato específico, sin adelantar S24. Catálogo y cierre de fase preceden a la GUI C#/.NET.
