# Lista de deseos IRQ del ecosistema SV

## 1. Naturaleza

El Wishlist IRQ sirve para recoger deseos, ideas, intuiciones, mejoras, aperturas o líneas laterales sin convertirlas automáticamente en trabajo activo.

Su misión es **preservar el aire creativo con control**.

## 2. Regla general

Toda idea nueva que no pertenezca de forma obvia a la ruta activa deberá pasar por este Wishlist antes de transformarse en tarea, parche, fase o frente.

## 3. Relación con la unidad encargada

La unidad encargada será la responsable de valorar cada entrada del Wishlist. Antes de asignar prioridad, deberá:

1. mirar el CSV disponible del registro técnico y de calidad;
2. comprobar si la idea ya está absorbida, registrada, cerrada o rechazada;
3. valorar si pertenece a la ruta activa, a Beta, a aparcamiento o a rechazo;
4. asignar un **IRQ** de prioridad.

## 4. Escala IRQ

- **IRQ-0** — bloqueo estructural o contradicción local suficiente en la ruta activa.
- **IRQ-1** — prioridad muy alta de saneamiento o riesgo de rotura próxima.
- **IRQ-2** — trabajo importante de fase, no bloqueante inmediato.
- **IRQ-3** — mejora útil compatible con la fase, sin urgencia dura.
- **IRQ-4** — idea valiosa, pero no de ejecución actual.
- **IRQ-5** — laboratorio, curiosidad o hipótesis de vigilancia.
- **IRQ-6** — aparcado largo o dependiente de condiciones no presentes.
- **IRQ-7** — no procede en la fase actual.

## 5. Campos mínimos por entrada

- ID
- Fecha
- Agente encargado o origen de la propuesta
- Deseo o idea
- Motivo
- CSV consultado
- Evaluación técnica
- IRQ asignado
- Ruta resultante (`activa`, `beta`, `aparcado`, `rechazado`)
- Decisión
- Estado

## 6. Regla de salida

Una entrada del Wishlist solo puede salir del Wishlist por una de estas vías:

1. absorción en la ruta activa;
2. traslado al carril Beta;
3. aparcamiento razonado;
4. rechazo motivado;
5. cierre por haber quedado ya satisfecha en el ecosistema.

## 7. Regla de autoridad

El IRQ no sustituye al pliego ni al registro técnico. El Wishlist IRQ ordena deseo y presión; no corrige la jerarquía del sistema.


## 8. Política mínima de uso

La lista de deseos IRQ no sirve para abrir frentes por entusiasmo inmediato. Su función es capturar deseo, presión, oportunidad o necesidad lateral y someterla a control de fase.

Cuando una entrada tenga valor estratégico de adopción o ecosistema, podrá dar lugar a una **política y protocolo específicos**, sin que eso obligue todavía a su implementación completa.

## 9. Entradas de referencia

- `WIRQ-2026-000` — inicialización del mecanismo.
- `WIRQ-2026-001` — compatibilidad interlenguajes del Lenguaje SV con prioridad inicial en Python, Rust, C, C++, Kotlin y vigilancia de legado empresarial como Cobol.
- `WIRQ-2026-002` — patrón clínico experto para SV: imagen de las células implicadas, diagnóstico del sistema SV y recomendación de actuación al experto humano bajo soberanía profesional.
- `WIRQ-2026-003` — capa de sinceridad SV sobre bases de datos empresariales como alternativa gobernada a capas de IA opacas, inestables o poco auditables para dirección y decisión.
- `WIRQ-2026-004` — interfaz declarativa en lenguaje natural para programación asistida del SV, con previsualización de resultado y aceptación explícita del desarrollador.
- `WIRQ-2026-005` — shell y entorno interactivo del lenguaje SV orientado al desarrollador, con ayuda contextual solo bajo demanda explícita.
- `WIRQ-2026-006` — capacidad de trabajo sincronizado local/remoto del ecosistema de desarrollo SV, diferida hasta madurez funcional e infraestructural suficiente.
- `WIRQ-2026-007` — generación de agentes especializados por el lenguaje SV a partir de patrones expertos, con configuración precargada, base informativa actualizable y remisión estructural al repositorio `SVperitus`.
