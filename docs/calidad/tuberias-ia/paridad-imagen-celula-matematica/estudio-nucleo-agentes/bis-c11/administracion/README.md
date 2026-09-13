# Código de administración conservado · C11

Se conservan los scripts exactos de preparación, verificación, publicación y cotejo de espejo, con los auxiliares de registro y acceso GitHub. No contienen credenciales. Python administra documentos y registros; no implementa la semántica SV.

Estos scripts dependen de la disposición del workspace original: bis-c11, bis-c02, p1p3-bis, manifiesto-sv/checkout y s6-trazabilidad-total. preparar.py exige el corte base limpio y añade RETP e historial: no se debe reejecutar sobre la versión publicada. verificar.py comprueba el cambio contra ese corte previo; no es un test del núcleo. publicar.py necesita el conector GitHub disponible, checkpoint local y ramas en los cortes esperados; no es una orden de republicación para terceros.

El adaptador github_io.py depende de HostedAppsClient del entorno de trabajo. Se declara esta dependencia; no se ofrecen estos scripts como instalador autónomo. Los resultados y comandos de la comprobación documental se conservan en REGISTRO_PREPARACION.json y cotejo.stdout/stderr. Los commits de publicación y su espejo constituyen evidencia posterior, comunicada al finalizar; no se predicen en este documento.

La reproducción funcional futura de C11 deberá disponer de receptor Rust, oráculo y observador de recursos propios. La disponibilidad del código de administración no acredita esa ejecución.
