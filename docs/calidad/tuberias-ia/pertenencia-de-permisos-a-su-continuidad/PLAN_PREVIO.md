# Contraste de pertenencia en la cadena gobernada — RETP-153

12/09/2026. Plan fijado antes de ejecutar. Corte Lenguaje cf366a339f66ba892a1ce783bb873d8c6f704801; laboratorio 3d556f378df9306a73f3497e7ee39351640fec36. Se continúa G2 del workflow V2. La lectura RETP-152 conserva su resultado y no se convierte en permiso.

## Hipótesis y criterio

`DecisionTraceRef` usa ordinales locales. Dos continuidades separadas, con la misma constitución sintética y su primera decisión, podrían aceptar por igualdad de datos un permiso o compromiso ajeno. El criterio exigido para esta candidata es pertenencia a la misma instancia lógica protegida antes de mediar o invocar al ejecutor.

Se reutiliza exclusivamente el montaje interno de R1 (`permission_continuity`, `permission_results`, `TestExecutor`). Su premisa y resultados son de prueba: no constituyen autoridad profesional ni prueban el productor externo pendiente. La sonda usa después las funciones reales públicas de decisión, mediación y ejecución. No se abren constructores para los clientes.

| Testigo | Preparación | Esperado |
| --- | --- | --- |
| PC01 | Permiso y compromiso propios; misma continuidad | Una llamada al ejecutor y confirmación |
| PC02 | Dos continuidades con constitución y decisión ordinal iguales; permiso A presentado en B | Rechazar antes de mediar; B sin marca de mediación |
| PC03 | Ambas continuidades han mediado sus permisos; compromiso A presentado en B | Rechazar antes de ejecutar; cero llamadas y cero eventos de ejercicio en B |

Se ejecutará primero la fuente vigente con los tres testigos. Los fallos observados se conservarán. Si se confirma la hipótesis, sólo se prepara una corrección candidata en laboratorio, sin integrar fuentes productivas. La corrección debe conservar trazas/identificadores públicos y ligar internamente cada token a su instancia protegida. Debe declarar coste, ámbito y agotamiento. Ningún fallo se convierte en U.

Si se usa identificador interno de instancia: entero atómico con incremento comprobado, sin reloj, reinicio ni reutilización; agotamiento impide nuevas decisiones. No acredita identidad durable entre procesos, legitimidad humana ni anfitrión hostil. No se le da semántica de tiempo o autoridad. Se contrastan frontera de agotamiento y preservación al mover el objeto. La regresión del crate se ejecutará en depuración y optimización porque el cambio afecta las tres entradas protegidas públicas.

Condición de cierre de este ensayo: positivos propios y negativos ajenos discriminados, guardas anteriores conservadas, fuentes y capturas recuperables. El enlace profesional completo seguirá pendiente de la admisión externa constituida y de la producción gobernada de comprobaciones. No abrir P3 reservado, /2–/3, P4/P5, dominio, agente o interfaz.
