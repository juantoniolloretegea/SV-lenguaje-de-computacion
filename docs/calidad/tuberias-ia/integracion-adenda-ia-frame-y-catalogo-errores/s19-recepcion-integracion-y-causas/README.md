# S19 · Recepción de S18 y relevo al catálogo

[Acta y decisión](ACTA_RECEPCION_S19.md) · [Matriz A–L](MATRIZ_COBERTURA_A_L.md) · [Inventario de causas y estados](INVENTARIO_CAUSAS_Y_ESTADOS.md) · [Siguiente objeto](RELEVO_AL_CATALOGO.md).

Resultado: recepción documental conforme del recorrido conjunto sintético S18. Permite avanzar al catálogo en el alcance acotado; conserva pendientes profesionales y nucleares. Cero ensayos funcionales nuevos.

Datos completos: [matriz JSON](MATRIZ_COBERTURA_A_L_RECEPCION_S18.json), [inventario JSON](INVENTARIO_CAUSAS_Y_ESTADOS.json), [fronteras](FRONTERAS_RECIBIDAS.json), [recepción](RECEPCION.json), [cortes](CORTE_Y_ALCANCE.json), [fuentes recuperables](FUENTES_RECIBIDAS.json).

Revisión documental sin red ni ejecución de Rust:

```sh
python3 verificar.py
```

`construir.py` reproduce la matriz, el inventario, las fronteras y el resumen desde las fuentes conservadas. `verificar.py` coteja además contra antecedentes y oráculos recibidos, y escribe su resultado determinista en VERIFICACION.json. Se ejecutan desde esta carpeta; sólo requieren la biblioteca estándar de Python. Python no decide SV. El manifiesto excluye su propio archivo y la salida VERIFICACION.json para evitar autorreferencia; el commit de publicación identifica el conjunto completo.
