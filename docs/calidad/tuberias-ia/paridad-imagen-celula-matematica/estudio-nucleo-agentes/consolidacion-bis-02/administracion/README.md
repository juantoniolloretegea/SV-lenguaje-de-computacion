# Código y operaciones de consolidación

Se conservan los scripts exactos de preparación, verificación, publicación y cotejo posterior, junto con registro.py, gestion.py, github_io.py y publicar_archivos.py. Python administra documentos; no implementa el compilador SV.

Dependen de la disposición original del workspace y, para publicar, de HostedAppsClient y el conector GitHub. preparar.py requiere checkout limpio en el corte de entrada y añade RETP e historial: no se reejecuta sobre el cambio publicado. verificar.py coteja contra ese corte. Los scripts se identifican por SHA-256 en el manifiesto previo.

Operaciones realizadas: lectura GitHub de cabeceras y documentos; git fetch y merge --ff-only para recuperar C12; lectura local de fuentes; cotejo SHA-256 de rectores; comprobación de rustc/cargo y búsqueda acotada sin localizarlos; preparación documental y cotejo Python. Se conservaron los errores de ausencia como limitación y no se ejecutó código Rust. Las capturas de esta verificación se adjuntan en cotejo.stdout y cotejo.stderr.

La publicación crea blobs, árbol, commit y avance de rama sin fuerza, primero laboratorio y después público. Los checkpoints conservan el corte, archivos y commit. verificar_publicacion.py coteja identidades y espejo antes de sincronizar el checkout. El diff del commit identifica exactamente los estados previos y posteriores de los ocho archivos vigentes modificados. No se modifica ningún banco histórico.
