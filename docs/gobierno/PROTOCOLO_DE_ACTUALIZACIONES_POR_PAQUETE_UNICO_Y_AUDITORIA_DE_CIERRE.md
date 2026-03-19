# Protocolo de actualizaciones por paquete único y auditoría de cierre

## 1. Naturaleza

Este protocolo fija la ruta de mayor rendimiento para actualizar el ecosistema operativo del Lenguaje SV sin degradar la seguridad de la información final ni entrar en iteraciones pobres del tipo «crear un documento y rectificarlo en el paso siguiente».

Su tesis es simple:

**la seguridad no aumenta por fragmentar decisiones inmaduras en varios commits; aumenta cuando la decisión madura fuera del repositorio, se materializa en un solo paquete coherente y se audita al cierre.**

## 2. Problema que corrige

Se corrige el siguiente patrón de bajo rendimiento:

1. crear acta o documento demasiado pronto;
2. descubrir en el paso siguiente que faltaba una pieza, una ruta o una consecuencia;
3. modificar inmediatamente el documento recién creado;
4. multiplicar registro, fricción y ruido sin ganar verdad material.

Ese patrón solo será admisible cuando exista **error material real** o **evidencia nueva contradictoria** no disponible en la precongelación.

## 3. Regla general

Toda actualización deberá intentar nacer ya como **paquete cerrable**, no como cadena de rectificaciones previsibles.

La secuencia ordinaria pasa a ser:

1. **precongelación**;
2. **paquete único de decisión o parche**;
3. **materialización única en repositorio**;
4. **auditoría de cierre**.

## 4. Fase 1 — Precongelación

Antes de tocar GitHub o el repositorio material, la unidad deberá cerrar fuera del repo, como mínimo, estos cinco puntos:

1. qué se quiere cambiar exactamente;
2. por qué procede ahora y no después;
3. qué archivos quedan afectados de verdad;
4. qué artefacto de trazabilidad corresponde;
5. qué condición de cierre permitirá no tocarlo de nuevo al minuto siguiente.

Si alguno de esos cinco puntos no está maduro, no procede todavía la materialización.

## 5. Fase 2 — Paquete único

La unidad deberá preparar un **ZIP único** con exactamente los archivos del cambio y solo esos archivos.

Ese ZIP debe contener ya la solución madura de conjunto:

- documento rector, si procede;
- actualización dual `.md/.csv`, cuando proceda;
- README o índice afectado, solo si su omisión rompería descubrilidad real;
- parche técnico y tests, si el cambio es material;
- y ninguna pieza ornamental o duplicada.

## 6. Fase 3 — Materialización única

La subida al repositorio deberá intentar resolver el cambio en **una sola pasada material**.

Por defecto, queda desaconsejado este patrón:

- crear acta hoy;
- corregir acta mañana;
- crear registro pasado mañana;
- y ajustar README en un cuarto paso.

Si el cambio necesita acta, registro, CSV y README, deberá intentarse que nazcan juntos en el mismo paquete.

## 7. Fase 4 — Auditoría de cierre

La verificación posterior se hará con evidencia suficiente de cierre:

- vista GitHub;
- PDF de confirmación, cuando proceda;
- contraste de rutas reales;
- comprobación de suite o emisión observable, si hay código;
- y constatación de que no falta un archivo esencial del paquete.

La auditoría de cierre no debe reabrir el cambio por capricho. Solo debe detectar:

1. error material;
2. omisión de artefacto esencial;
3. contradicción con la evidencia real del repositorio;
4. o desalineación no visible en la precongelación.

## 8. Qué artefacto de trazabilidad corresponde en cada caso

### 8.1. Acta obligatoria

El acta queda reservada, por regla general, para cambios de rango suficiente, entre ellos:

1. apertura o cierre de fase;
2. apertura o cierre de bloque cerrable;
3. cambio de jerarquía entre sedes;
4. fijación o revisión de política o protocolo rector;
5. alteración del ABI semántico-diagnóstico, del catálogo de errores o de una frontera de fase;
6. decisión pública de arquitectura que deba poder leerse por terceros sin reconstrucción oral.

### 8.2. Registro sin acta autónoma

No toda actualización merece acta propia. Bastará registro `.md/.csv`, por regla general, en casos como:

1. regularización documental;
2. incorporación de entradas al Wishlist IRQ;
3. ampliación de matrices, tablas o dictámenes ya abiertos;
4. ajustes de descubrilidad;
5. documentación de saneamientos menores ya absorbidos por una decisión de rango superior.

### 8.3. Parche técnico con trazabilidad mínima suficiente

Cuando el cambio sea un saneamiento local de implementación, podrá bastar:

- parche de archivos técnicos;
- tests o evidencia observable;
- y actualización mínima del registro o del dictamen vivo.

No toda corrección local exige nueva inflación documental.

## 9. Regla anti-iteración inmediata

Queda fijada esta regla:

**ningún documento recién creado deberá rectificarse en el paso inmediatamente siguiente salvo que concurra una de estas tres causas**:

1. error material real;
2. omisión comprobable de un artefacto esencial que debía haber ido en el paquete;
3. contradicción nueva revelada por repositorio fresco o auditoría de cierre.

Fuera de esos tres supuestos, la modificación deberá esperar al siguiente bloque legítimo de trabajo.

## 10. Regla de Wishlist IRQ

Toda presión creativa lateral deberá entrar primero por el Wishlist IRQ.

La mera incorporación al Wishlist:

- no abre fase;
- no obliga por sí sola a implementación inmediata;
- no desplaza la ruta activa;
- y no autoriza salto de prioridad sobre bloques técnicos ya abiertos.

Solo se elevará de Wishlist a política, bloque o frente cuando exista prioridad, evidencia y madurez de paquete.

## 11. Criterio de rendimiento operativo

La ruta de mayor rendimiento no es la ruta más corta en número bruto de archivos, sino la que minimiza:

- rectificaciones previsibles;
- ruido documental;
- duplicación de trazabilidad;
- y dependencia de memoria oral.

Por eso, en esta fase, se adopta como regla preferente:

**menos pasos, pero más maduros; menos documentos, pero mejor justificados; una sola subida por bloque; una sola auditoría de cierre por paquete.**

## 12. Estado de fase

Estado actual: **protocolo activo para futuras actualizaciones del repositorio operativo del Lenguaje SV**.
