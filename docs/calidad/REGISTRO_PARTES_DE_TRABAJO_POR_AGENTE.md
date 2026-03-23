# Registro de partes de trabajo por agente

## Finalidad

Este registro deja constancia mínima, verificable y comparativa del trabajo realizado por cada agente sobre un repo fresco determinado.

## Campos clave

- `Agente`: siempre en formato `Agente X`.
- `Lectura_Repo_Fresco`: grado real de lectura empleado antes de actuar.
- `Actuaciones_Desde_Ultimo_Control_Secuencial`: resumen de lo ocurrido desde el último barrido formal del árbol.

## Regla operativa

- No debe declararse una lectura `100_REAL` si el agente no ha recorrido realmente el repo fresco relevante para su dictamen.
- Las actuaciones globales de fase o de registro maestro deben apoyarse en lectura `100_REAL`.
- Las actuaciones locales pueden apoyarse en lectura `AJUSTADA_AL_OBJETO`, pero deben declararlo expresamente.
