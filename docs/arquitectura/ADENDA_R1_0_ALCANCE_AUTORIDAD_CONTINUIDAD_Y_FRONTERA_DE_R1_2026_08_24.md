# Adenda a R1-0 — alcance de autoridad, continuidad y frontera material de R1

**Fecha:** 24 de agosto de 2026  
**Ámbito:** R1 — autoridad, mediación y decisiones protegidas  
**Documento afectado:** `CONTRATO_R1_0_TIPOS_CERRADOS_Y_FRONTERAS_DE_CONSTRUCCION_2026_08_24.md`  
**Estado:** precisión de alcance

## 1. Objeto

Esta adenda precisa tres límites necesarios para interpretar R1-0 y los cortes posteriores de R1 sin ampliar su radio hacia propiedades de implantación, persistencia o gobierno de agentes que pertenecen a otras fases o perfiles.

No modifica los tipos integrados en R1-0, no concede autoridad, no decide permisos y no altera la semántica cerrada de R0.

## 2. Autoridad representada en R1

La autoridad materializada en R1 es una capacidad constituida y gobernada dentro del proceso soberano para habilitar o ejercer formas y efectos sometidos a control.

No se identifica con:

- una credencial del sistema operativo;
- una identidad de proveedor o servicio;
- una firma humana o criptográfica;
- una cuenta administrativa;
- un proceso, contenedor o máquina virtual;
- una raíz material de confianza;
- una facultad externa de implantación.

Por tanto:

```text
autoridad R1
≠ autoridad de implantación
≠ privilegio técnico del anfitrión
≠ identidad externa
≠ credencial criptográfica
```

La capacidad técnica de invocar una función o poseer un identificador tampoco constituye autoridad SV.

## 3. Continuidad autoritativa en el SUT de R1

R1 modela únicamente el estado lógico intra-proceso necesario para decidir si una continuidad autoritativa representada está todavía no habitada o ya contiene autoridad admitida.

La condición de ocupación no puede deducirse de:

- PID;
- `instance_id`;
- `boot_nonce`;
- dirección de memoria;
- nacimiento de un proceso;
- reinicio técnico;
- cambio de adaptador;
- cualquier identificador efímero equivalente.

En el alcance de R1:

```text
Uninhabited
→ no existe todavía autoridad admitida en la continuidad lógica representada

Inhabited
→ la continuidad lógica representada ya contiene autoridad admitida
```

Clonar, copiar o reconstruir dentro del mismo proceso una representación ya habitada no restablece la disponibilidad de T-0.

Un nuevo proceso que no incorpore ni continúe estado autoritativo previo puede constituir, dentro de este modelo limitado, una continuidad todavía no habitada. Determinar materialmente continuidad entre procesos, restauraciones, réplicas o estados persistentes queda fuera de R1 y corresponde a las fases posteriores previstas para persistencia y plataforma.

## 4. Efecto protegido en R1

R1 materializa únicamente la mediación intra-proceso de efectos que el propio núcleo represente como protegidos.

La afirmación:

```text
efecto protegido mediado en R1
```

significa que, dentro de la API y de las fronteras de `sv_core`, no existe una vía ordinaria que produzca ese efecto sin atravesar el control constituido correspondiente.

No significa mediación material completa frente a:

- escritura externa de memoria o almacenamiento;
- privilegios del sistema operativo o hipervisor;
- administración de infraestructura;
- depuración externa;
- carga o sustitución material del ejecutable;
- manipulación de una raíz de confianza;
- otros falsificadores externos al proceso.

Estas materias requieren las fases materiales posteriores y no quedan acreditadas por R1.

## 5. Capacidades expresamente fuera de R1

No forman parte de los entregables de R1:

- ITI completo;
- gobierno integral de agentes especializados;
- perfiles de IA auxiliar;
- cliente o canal hacia un motor de IA;
- API específica de un proveedor de modelos;
- persistencia autoritativa durable;
- recuperación material entre reinicios;
- identidad externa;
- firma humana o criptográfica;
- raíz material de confianza;
- cadena de suministro;
- Garantía I;
- Garantía II.

La eventual integración de cualquiera de estas capacidades deberá respetar el orden de fases y el perfil que corresponda, sin atribuir a R1 propiedades no ejercidas.

## 6. Consecuencia para los cortes R1-1…R1-6

Los cortes posteriores deberán conservar conjuntamente:

```text
autoridad intra-proceso
+ continuidad lógica no identificada con la instancia técnica
+ mediación limitada al perímetro de sv_core
```

Ningún tipo, forma, descriptor, comprobación o permiso de R1 podrá utilizarse para afirmar por sí mismo autoridad externa, persistencia durable o seguridad material del entorno anfitrión.

## 7. Estado

Esta adenda no reabre R1-0 ni modifica su cierre. Precisa su lectura para los cortes posteriores.

```text
R0 = CERRADO
R1 = ABIERTO
R1-0 = CERRADO
R1-1 = EN DESARROLLO
R2–R4 = NO INICIADOS
Garantía I = NO_PROBADO
Garantía II = NO_PROBADO
```
