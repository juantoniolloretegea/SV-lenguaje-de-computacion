# Apertura de R2 — persistencia y continuidad material

**Fecha:** 25 de agosto de 2026  
**Repositorio:** `SV-lenguaje-de-computacion`  
**Estado:** R2 abierto  
**R0:** cerrado  
**R1:** cerrado  
**R3–R4:** no iniciados  
**Garantía I:** `NO_PROBADO`  
**Garantía II:** `NO_PROBADO`

## 1. Objeto

R2 materializa las condiciones de persistencia y continuidad material necesarias para que los estados, decisiones, autoridades, revocaciones y consumos que deban sobrevivir al proceso no se reconstruyan por conveniencia, orden accidental, copia técnica o apariencia de actualidad.

La fase parte de las propiedades intra-proceso cerradas en R0 y R1 y no modifica la semántica del Lenguaje SV, la gramática, la IR ni las separaciones entre `Tri`, D-A/D-R/D-N, autoridad, permiso y ejercicio.

## 2. Punto de partida

R2 se abre sobre el corte soberano:

```text
main = 243bf4c2d6cdf93329185e2a628ee73aaf12a8e3
```

con:

```text
R0 = CERRADO
R1 = CERRADO
R2 = ABIERTO
R3 = NO INICIADO
R4 = NO INICIADO
```

## 3. Base contractual

R2 se interpreta conjuntamente con los contratos SEC.0, en particular SEC.0-M, y con la especificación arquitectónica vigente del entorno de ejecución soberano.

Su alcance material corresponde a:

```text
AStore
PDep
revocación persistente
presupuestos cuando sean aplicables
tiempo sólo cuando una obligación constituida lo requiera
recuperación no circular
bifurcación y retroceso
consumo único cuando el perfil lo ofrezca
```

La presencia de estos objetos en el alcance no presume una tecnología concreta de almacenamiento, reloj, bloqueo, base de datos, servicio, sistema operativo ni plataforma.

## 4. Distinciones obligatorias

R2 deberá mantener separadas, cuando resulten aplicables:

```text
estado de proceso
≠ estado derivado
≠ estado persistente autoritativo
≠ continuidad vigente
```

Asimismo:

```text
copia técnica ≠ continuidad legítima
fecha mayor ≠ vigencia
último registro ≠ autoridad
restauración ≠ nueva génesis
persistencia de D-A ≠ aplicabilidad permanente
```

Una vista, caché, índice o réplica no autoritativa no podrá sustituir por sí sola al estado persistente que deba gobernar una decisión.

## 5. Dependencias persistentes

Toda decisión o capacidad que sobreviva a reinicio o recuperación deberá poder reconstruir las dependencias persistentes suficientes para justificar su aplicabilidad.

Cuando una dependencia necesaria no pueda acreditarse después de recuperación:

```text
no se hereda la acreditación anterior
no se fabrica una continuidad favorable
no se promueve el fallo a Tri.U
```

La consecuencia deberá conservar la separación contractual entre fallo técnico, no verificabilidad y semántica.

## 6. Recuperación no circular

R2 deberá impedir que el estado recuperado sea simultáneamente la única fuente de la prueba que acredita la legitimidad de ese mismo estado frente al fallo considerado.

Una recuperación no podrá legitimarse únicamente porque:

- el registro restaurado se declare a sí mismo vigente;
- posea la fecha o secuencia numérica mayor;
- provenga de la copia técnicamente más reciente;
- conserve una clave o testigo que pueda haberse retrocedido o clonado junto con él.

Cuando una propiedad necesite independencia material que R2 no pueda establecer por sí solo, dicha dependencia permanecerá declarada y su acreditación corresponderá a R3 o a la fase material aplicable.

## 7. Bifurcación, clonación y consumo

R2 deberá representar las condiciones necesarias para distinguir continuidad legítima de bifurcaciones incompatibles cuando esa distinción sea exigible.

Si un perfil ofrece consumo único resistente a reinicio, restauración o carrera concurrente, no bastará un contador local susceptible de retroceder o clonarse con el mismo estado que pretende proteger.

La fase no presume que todos los perfiles deban ofrecer esa garantía; cuando se afirme, deberá demostrarse dentro del modelo de fallos correspondiente.

## 8. Tiempo y presupuesto

El tiempo no se introduce como primitiva semántica universal.

Sólo cuando una forma u obligación previamente constituida dependa de actualidad, expiración, presupuesto o ventana temporal podrá incorporarse una fuente temporal explícita como dependencia de esa comprobación.

Se conserva:

```text
marca temporal mayor ≠ continuidad legítima
orden cronológico ≠ autoridad
reloj técnico ≠ Tri
```

Del mismo modo, `Budget(F | C)` sólo se materializará cuando el contrato aplicable lo exija. R2 no abre por sí solo un presupuesto universal del sistema.

## 9. Exclusiones de R2

R2 no materializa ni acredita por sí solo:

- raíz material de confianza;
- cadena completa de construcción, distribución y carga;
- identidad del artefacto ejecutado frente a sustitución material;
- aislamiento de sistema operativo, hipervisor o hardware;
- control material universal de red, periféricos o dispositivos;
- atestación de plataforma;
- confidencialidad material completa;
- agentes especializados;
- motor de IA;
- IA de seguridad;
- R3 o R4;
- Garantía I;
- Garantía II.

Estas cuestiones no se trasladarán a R2 por anticipación.

## 10. Continuidad arquitectónica gobernada

El núcleo y las capas de continuidad que se materialicen en R2 se implementarán como una estructura mínima garantista suficiente para las propiedades exigibles del alcance constituido. El diseño actual no deberá bloquear revisiones, sustituciones ni arquitecturas futuras que acrediten una solución técnicamente superior, más segura o más adecuada.

Esa apertura no constituye una excepción de seguridad. Una arquitectura posterior no podrá heredar automáticamente autoridad, evidencia, garantías ni vigencia por el mero hecho de sustituir a la anterior. Toda modificación causalmente relevante deberá quedar gobernada, acreditada y trazada conforme al **Pliego de Condiciones del Sistema Vectorial SV**, DOI `10.21428/39829d0b.bbcac925`, y a las transiciones y obligaciones que resulten aplicables.

En consecuencia:

```text
la evolución futura permanece posible
∧
ninguna evolución futura entra por una vía no gobernada
```

## 11. Regla de apertura

Este documento abre R2, pero no fija todavía su descomposición interna ni autoriza una tecnología concreta.

La primera tarea de R2 será cerrar su contrato material y su descomposición de trabajo antes de introducir código productivo nuevo. Esa descomposición deberá demostrar qué propiedades pertenecen realmente a R2 y cuáles deben permanecer diferidas a R3 o R4.

## 12. Estado resultante

```text
R0 = CERRADO
R1 = CERRADO
R2 = ABIERTO
R3 = NO INICIADO
R4 = NO INICIADO

T-E = PRODUCTIVA POR VÍA GOBERNADA
T-G / T-C / T-R = NO PRODUCTIVAS

BudgetΣ / IA-SEC = NO ABIERTOS
Garantía I / II = NO_PROBADO
```
