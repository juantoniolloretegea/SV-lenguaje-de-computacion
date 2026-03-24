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


## Regla de no repetición derivada del relevo WBeta SV-UCBC8

Se incorpora la siguiente disciplina operativa para evitar repetición de errores por parte de futuros agentes:

1. Todo archivo `.zip` subido por el responsable del proyecto debe inspeccionarse realmente por dentro antes de emitir juicio, aunque no se diga expresamente, presumiendo de entrada que puede tratarse de un repositorio fresco o de un parche material relevante.
2. En caso de tensión entre:
   - nombre externo del archivo,
   - contenido interno real del ZIP,
   - clon local,
   - PDF,
   - repo fresco,
   - o memoria conversacional,

   **manda el contenido material verificable más reciente**, no la memoria del agente.
3. Ningún agente debe tratar como “crear” un archivo cuya existencia real en el repo fresco no haya verificado antes con lectura suficiente del árbol.
4. La lectura `100_REAL` obliga a revisar el objeto material subido, no sólo a reconstruir el estado por continuidad del chat.
5. Cuando un agente cometa una inferencia indebida o un error de lectura material, debe dejar constancia explícita para prevenir repetición por agentes posteriores.

### Error constatado en este ciclo

En el relevo que precede a este apunte se detectaron, y quedan expresamente prohibidos para repetición futura, los siguientes fallos:

- tomar el nombre externo de un ZIP como prueba suficiente de que el contenido interno correspondía al repo esperado;
- no inspeccionar inmediatamente un ZIP fresco y completo antes de dictaminar;
- mantener reservas ya levantadas por evidencia material suficiente aportada por PDF o repo fresco;
- y proponer acciones de GitHub como `crear` donde el repo fresco exigía `modificar`.

### Efecto

Esta regla pasa a integrarse como aprendizaje operativo mínimo del sistema de partes por agente y debe considerarse vigente desde 24/03/2026.
