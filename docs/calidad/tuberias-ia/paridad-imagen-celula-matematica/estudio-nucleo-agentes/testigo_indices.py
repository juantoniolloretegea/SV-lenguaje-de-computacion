"""Contraejemplo de índices del evaluador histórico; no ejecuta una CNN.
Reproducción: python testigo_indices.py > TESTIGO_INDICES.json
Los índices numéricos son etiquetas del clasificador, no valores Tri SV.
"""
import json
canonical = {'NO_APTO': 0, 'INDETERMINADO': 1, 'APTO': 2}
disk = {'APTO': 0, 'INDETERMINADO': 1, 'NO_APTO': 2}
remap = [canonical[name] for name in sorted(disk, key=disk.get)]
# Caso discriminante: un predictor hipotético perfecto en índices canónicos.
true_names = ['NO_APTO', 'INDETERMINADO', 'APTO']
labels_disk = [disk[name] for name in true_names]
preds_canonical = [canonical[name] for name in true_names]
labels_canonical = [remap[i] for i in labels_disk]
preds_after_second_remap = [remap[i] for i in preds_canonical]
assert preds_canonical == labels_canonical
assert preds_after_second_remap == [2, 1, 0]
assert sum(p == y for p, y in zip(preds_after_second_remap, labels_canonical)) == 1
print(json.dumps(dict(
 alcance='Contraejemplo lógico aislado del remapeo; no entrenamiento ni métrica de modelo real',
 fuente='SVperitus-dataset@47dc27aec9e7b517c27cfcf39b7ad1b186d36a4c',
 supuesto='Predicciones ya canónicas, como corresponde al entrenamiento con target_transform',
 etiquetas_disco=labels_disk, etiquetas_canonicas=labels_canonical,
 predicciones_canonicas=preds_canonical, predicciones_tras_segundo_remapeo=preds_after_second_remap,
 aciertos_antes=3, aciertos_despues=1, casos=3,
 resultado='El segundo remapeo intercambia NO_APTO y APTO; conserva INDETERMINADO'
), ensure_ascii=False, indent=2))
