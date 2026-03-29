# Acta técnica de cierre auditado del parche correctivo del frontend — 23/03/2026

**Fecha:** 23/03/2026  
**Hora (Europe/Madrid):** 20:02:35  
**Agente:** Agente WLenguaje8 SV

## 1. Objeto

La presente acta cierra, con verificación posterior suficiente, el **parche correctivo funcional acotado del frontend** abierto para corregir errores observables del compilador actual sin reabrir `H2`, sin abrir `H3` y sin tocar backend soberano.

El objeto exacto del lote ha sido restaurar corrección visible y precisión diagnóstica mínima en cuatro puntos concretos del árbol vivo:

- recuperación de emisión específica de `E102`, `E104` y `E303` cuando proceda;
- imposición de no-vaciedad en `compose` para `E208` y `E209`;
- eliminación del falso positivo por orden textual en `AdmissibilitySpec`;
- corrección del diagnóstico de `PendingU` como construcción reconocida por gramática pero no habilitada en `v0.1`.

## 2. Base técnica considerada

La decisión se apoya en verificación documental real del repositorio operativo y, en particular, en:

- `src/svp_errors.py`
- `src/svp_parser.py`
- `src/svp_validator.py`
- `tests/run_conformance.py`
- `tests/conformance/invalid/output_semantics_no_declarada.svp`
- `tests/conformance/invalid/compose_relations_vacias.svp`
- `tests/conformance/invalid/compose_patterns_vacios.svp`
- `tests/conformance/invalid/pending_u_reconocido_no_habilitado.svp`
- `tests/conformance/invalid/transition_data_horizon_no_declarado.svp`
- `tests/conformance/valid/admissibility_spec_states_permutados.svp`
- `tests/conformance/valid/admissibility_spec_states_permutados.expected.json`
- `docs/calidad/PROCEDIMIENTO_AUDITORIA_TECNICA_SV.md`
- `docs/calidad/REGISTRO_EVOLUCION_TECNICA_PROYECTO.md`
- `docs/calidad/REGISTRO_BARRIDOS_DE_ACTIVIDAD_Y_LATENCIA_DEL_REPO.csv`
- `docs/calidad/REGISTRO_PARTES_DE_TRABAJO_POR_AGENTE.csv`

## 3. Verificación material del cierre

En el estado auditado tras subida y contraste del árbol del repositorio verificado real, la base ejecutable queda así:

- `tests/run_conformance.py` → **37/37**
- `tests/run_cli_smoke.py` → **3/3**
- `tests/run_sec0_smoke.py` → **3/3**

Además, el contraste fino confirma en el árbol real:

- `E102` vuelve a emitirse de forma específica;
- `E104` vuelve a emitirse de forma específica;
- `E303` vuelve a emitirse de forma específica;
- `compose([], [])` deja de aceptarse silenciosamente;
- `AdmissibilitySpec` deja de depender del orden textual;
- `PendingU` deja de caer con diagnóstico engañoso.

## 4. Alcance exacto del cierre

El cierre debe leerse de forma estricta:

- **sí** corrige un lote visible y acotado del frontend actual;
- **sí** deja regresiones mínimas suficientes para sostener ese cierre;
- **no** regulariza por completo el catálogo muerto o estructuralmente inalcanzable;
- **no** reabre `E106` ni `E111` como frente inmediato;
- **no** verifica `H2` como hito positivo;
- **no** abre `H3` ni backend soberano.

## 5. Objeción adversarial considerada

La objeción adversarial más fuerte sostiene que, una vez corregidos estos fallos visibles, podría aprovecharse el lote para ensanchar el radio y arrastrar dentro del mismo paquete la depuración amplia del contrato diagnóstico o del catálogo muerto.

No procede.

La virtud del parche ha sido precisamente su **acotación**. Mezclar ahora regularización amplia de `E106`, `E111`, catálogo muerto o ABI semántico-diagnóstico rompería la unidad material del lote y degradaría la trazabilidad del cierre.

## 6. Decisión

Se declara **cerrado y auditado** el parche correctivo funcional acotado del frontend.

En consecuencia:

1. el lote deja de estar abierto como frente inmediato;
2. su traza pasa a gobierno técnico y registro de calidad;
3. el siguiente paso legítimo deja de ser la reapertura del mismo parche y pasa a ser, si procede, una regularización técnica limitada y separada.

## 7. Observación material

Durante la auditoría de cierre apareció en el ZIP de verificación una presencia accesoria de `__pycache__/*.pyc` generados por ejecución local. No forman parte del parche lógico ni deben tratarse como contenido del lote registral.

## 8. Cierre

La lectura correcta, a partir de ahora, es ésta:

> **El parche correctivo funcional acotado del frontend queda cerrado, verificado y separado de cualquier regularización diagnóstica más amplia.**
