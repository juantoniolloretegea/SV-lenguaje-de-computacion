# Acta técnica de sincronización documental del contrato diagnóstico visible — 24/03/2026

**Estado:** cierre propuesto de microparche documental rebasado  
**Fecha:** 24/03/2026

## 1. Objeto

Esta acta deja constancia del microparche documental destinado a resincronizar la documentación viva del contrato diagnóstico observable del frontend de referencia con el estado real del árbol del repositorio verificado.

## 2. Alcance

El lote afecta únicamente a documentación pública y de calidad:

- `docs/referencia/ERRORES_CANONICOS_SV_v0_2.md`
- `docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.md`
- `docs/calidad/MATRIZ_DE_CONCORDANCIA_DIAGNOSTICA_IR_IMPLEMENTACION_SV.csv`
- `docs/calidad/DICTAMEN_DE_SANEAMIENTO_DEL_BLOQUE_A_CONTRATO_DIAGNOSTICO.md`
- `docs/calidad/README.md`
- `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`
- `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv`

## 3. Evidencia material de resincronización

La lectura adversarial del árbol del repositorio verificado acredita hoy, de forma observable y con cobertura explícita de suite, al menos los siguientes subcasos:

- `E102` mediante `output_semantics_no_declarada.svp`;
- `E104` mediante `conector_target_no_ternario.svp`;
- `E208` mediante `compose_relations_vacias.svp`;
- `E209` mediante `compose_patterns_vacios.svp`;
- `E303` mediante `transition_data_horizon_no_declarado.svp`.

Además, el contraste directo con `src/svp_errors.py` obliga a corregir en la documentación viva los nombres implementativos de `E102` y `E104`, que no podían seguir figurando como `SpecNotFound` y `GraphMissingRelation` en este árbol.

## 4. Decisión

Procede un microparche **estrictamente documental** y **rebasado sobre el árbol del repositorio verificado** para sincronizar:

1. el catálogo público efectivo;
2. la matriz de concordancia diagnóstica;
3. el dictamen de saneamiento, mediante nota posterior explícita;
4. y la traza mínima en el registro maestro.

## 5. Fuera de alcance

Quedan expresamente fuera de este lote:

- `src/`;
- `tests/`;
- gramática, IR, validator, runner y backend;
- regularización amplia de `E106`, `E111` o del catálogo muerto;
- reescritura silenciosa de actas históricas cerradas.

## 6. Criterio adversarial

La resincronización no crea doctrina nueva ni cambia el comportamiento del compilador. Su función es impedir que la documentación viva siga describiendo como ausente una emitibilidad que el árbol del repositorio verificado ya acredita materialmente.
