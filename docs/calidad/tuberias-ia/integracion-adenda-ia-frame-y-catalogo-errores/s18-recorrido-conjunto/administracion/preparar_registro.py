from pathlib import Path
R=Path(__file__).resolve().parent
s=(R.parent/'s15-causas/registrar.py').read_text().replace("R.parent/'s14-bases/actualizacion-cierre'","R.parent/'s17-contrato/actualizacion'").replace("'186' if closing else '185'","'190' if closing else '189'").replace('s15-causas-recepcion','s18-recorrido-conjunto').replace('S15','S18').replace('s15','s18').replace('range(15)','range(18)').replace("rows[:15]","rows[:18]").replace("R.parent/'s14-bases/actualizacion-cierre/SUCESOS_SV.csv'","R.parent/'s17-contrato/actualizacion/SUCESOS_SV.csv'")
s=s.replace("fecha_inicio_utc=T,unidad_responsable", "fecha_inicio_utc=datetime.fromtimestamp((R/'lenguaje-base.json').stat().st_mtime,timezone.utc).isoformat(timespec='seconds').replace('+00:00','Z'),unidad_responsable")
s=s.replace('F: causas de recepción, protocolo, cobertura y presentación','Recorrido conjunto de contexto, base consumida, recepción y entrega tipada').replace('Catorce casos sintéticos; receptor acotado, fuentes G1/S2 intactas, registro y presentación textual sin pérdida de causa.','24 casos S17; producción y entrega G1, contexto, base, recepción, cobertura y archivo; sin promoción nuclear.').replace('S14 / RETP-184; cobertura S2 y cuerpo esperado anterior; autorización expresa de continuación.','S17 / RETP-188; fuentes originales G1; S14/S15; autorización expresa de continuación.')
a=s.index('if closing:\n res=');b=s.index("s=io.StringIO(newline='');w=csv.DictWriter",a)
s=s[:a]+'''if closing:
 res=json.loads((D/'RESULTADO.json').read_text()) if (D/'RESULTADO.json').exists() else json.loads((D/'INTERRUPCION.json').read_text())
 if res.get('conforme'):
  row.update(resultado='Conforme en el banco S18: recorrido conjunto con producción G1, base consumida, contexto, recepción y entrega tipada; original y reevaluación conservados.',verificacion=f"144 observaciones en seis ejecuciones debug/release; {res['capturas_identicas_por_ejecucion']} capturas idénticas por ejecución; 29 invocaciones; cuatro mutantes detectados y dos fabricaciones externas rechazadas por privacidad.",siguiente_accion='Recibir el resultado conjunto en la matriz de integración 1+3 y el inventario de causas; decidir el relevo documental al catálogo conservando B/E/K/L y las puertas pendientes.')
 else:
  row.update(estado='pendiente',fecha_fin_utc='',resultado='Campaña detenida sin reparación: '+res['error'],verificacion=str(res['invocaciones'])+' invocaciones conservadas; no se declara conformidad del recorrido.',siguiente_accion='Analizar la interrupción conservada; justificar una nueva fijación antes de cualquier corrección o ejecución adicional.')
 rows[-1]=row
else:
 row.update(resultado='Realización, contrato S17, casos, esperados y presupuesto fijados antes de compilar; sin ejecución funcional nueva todavía.',verificacion='Cortes y rectores cotejados; once archivos fuente recibidos por blob y SHA-256; 24 obligaciones S17 conservadas.',siguiente_accion='Ejecutar reproductor con máximo 29 invocaciones, cero reintentos; conservar y detener ante cualquier fallo inesperado.');rows.append(row)
''' + s[b:]
s=s.replace('de causas de recepción y no admisión','del recorrido documental conjunto').replace('S18 / F integración 1+3','S18 / recorrido integración 1+3').replace('Luz verde tras S14 y objeto F delimitado','Luz verde tras S17 / RETP-188').replace('S14; S13; rectores; G1/S2; cuerpo anterior','S17; S14; S15; rectores; G1; cuerpos anteriores')
(R/'registrar.py').write_text(s)
s=(R.parent/'s17-contrato/publicar.py').read_text();s=s.replace("R=Path(__file__).resolve().parent;CP=R/'PUBLICACION.json'","R=Path(__file__).resolve().parent;fase=sys.argv[1];assert fase in ['apertura','cierre'];CP=R/('PUBLICACION_'+fase.upper()+'.json')")
s=s.replace("message='calidad: fijar contrato del recorrido documental conjunto; S17 / RETP-188'", "message='calidad: '+fase+' del recorrido conjunto S18 / RETP-'+('189' if fase=='apertura' else '190')")
(R/'publicar.py').write_text(s)
