# Acta técnica de regularización registral del ciclo VII y fijación de la regla de inspección material de paquetes comprimidos

**Fecha:** 24/03/2026  
**Frente:** Lenguaje de computación del Sistema Vectorial SV  
**Fase:** IV — frente final del lenguaje (pre-backend)  
**Agente:** WLenguaje8 SV

## 1. Objeto

Regularizar la trazabilidad registral del ciclo documental y de gobierno semántico ya materializado en `docs/calidad/` el 24/03/2026, incorporando además la evidencia externa de `PTA-2026-004` y fijando de forma explícita la regla de inspección real de todo ZIP.

## 2. Hechos constatados

1. El árbol del repositorio verificado contiene ya los documentos de consolidación doctrinal y de vigilancia conceptual de la familia VII:
   - `ACTA_TECNICA_DE_CONSOLIDACION_DE_LA_FAMILIA_VII_Y_ALERTA_DE_GOBIERNO_SEMANTICO_PARA_EL_LENGUAJE_SV_2026_03_24.md`
   - `PROTOCOLO_CORTO_DE_VIGILANCIA_CONCEPTUAL_ABSOLUTA_DEL_LENGUAJE_SV_ANTE_FRENTES_DOCTRINALES_ABIERTOS_2026_03_24.md`
   - `ESPEJO_DOCTRINAL_COLECCIONES_LENGUAJE_SV.md`
   - `ESPEJO_DOCTRINAL_COLECCIONES_LENGUAJE_SV.csv`
2. El árbol del repositorio verificado incorpora también la cautela `H-10` en `MATRIZ_UCBC_HORIZONTES_LENGUAJE_SV.csv`.
3. El árbol vigente no había absorbido todavía en sus registros la pieza externa `PTA-2026-004`, a pesar de que ésta aporta una regla operativa correctora material sobre la inspección real de ZIP y la prevalencia del contenido interno del objeto sobre su nombre externo o sobre la memoria del agente.
4. La suite sigue sana en este árbol real: `run_conformance.py` 37/37, `run_cli_smoke.py` 3/3 y `run_sec0_smoke.py` 3/3.

## 3. Regla fijada

Queda fijado, de forma expresa y reutilizable por futuras continuidades operativas, lo siguiente:

> En todo ZIP entregado como evidencia operativa, prevalece el contenido interno del objeto sobre su nombre externo, su ruta de descarga o la memoria conversacional del agente. No procede declarar su naturaleza ni su alcance sin inspección real del contenido.

## 4. Alcance del lote

Este lote:

- **sí** regulariza la capa registral y de partes del ciclo del 24/03/2026;
- **sí** deja trazabilidad explícita de la regla de inspección material de paquetes comprimidos;
- **no** toca `src/`, `tests/`, `grammar/`, `spec/`, `stdlib/` ni backend;
- **no** abre todavía la rectificación documental de `E303`;
- **no** reabre `H2`, `H3` ni ningún frente funcional.

## 5. Decisión

Procede cerrar primero la asimetría registral del ciclo documental ya materializado y reservar para un lote posterior, separado y rebasado, cualquier sincronización documental de `E303`.
