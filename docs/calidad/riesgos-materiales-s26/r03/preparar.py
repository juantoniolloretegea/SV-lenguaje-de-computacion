"""Construye fixtures derivados y sus esperados sin ejecutar semántica SV.
Uso desde checkout. No modifica R01 ni el montaje histórico.
"""
from pathlib import Path
import hashlib,json,shutil
E=Path(__file__).resolve().parent
A=E.parent/'r01/fixture'
assert not (E/'fixture').exists()
shutil.copytree(A,E/'fixture/A');shutil.copytree(A,E/'fixture/B')
old=(A/'source.bin').read_bytes();new=old+b'\n'
h0=hashlib.sha256(old).hexdigest();h1=hashlib.sha256(new).hexdigest()
(E/'fixture/B/source.bin').write_bytes(new)
for name in ['context.json','request.json','registry.json','oracle.json']:
    p=E/'fixture/B'/name
    s=p.read_text().replace(h0,h1)
    # Sólo contexto y solicitud contienen tamaño del activo fuente.
    if name in ('context.json','request.json'):
        d=json.loads(s);d['fuente']['bytes']=len(new)
        s=json.dumps(d,ensure_ascii=False,indent=2)+'\n'
    p.write_text(s)
bank=dict(version='S26-R03/1',corte='b0f4f5f0093ad59324fcb6e00ed65cc15e3ca8f9',
    alcance='Tres sondas parciales T01/T02/T07; no campaña durable ni GUI.',
    cambio_B=dict(fuente='A + LF',sha_A=h0,sha_B=h1,otros_cambios='Huellas fuente y tamaño fuente concordantes; resto literal.'),
    casos=[dict(id='T01',esperado='A/A y B/B admitidos y certificados; B/A rechazado I02, full identity/custody mismatch',
                limite='Cambiar la custodia por B es un control local explícito, no prueba de autorización institucional.'),
           dict(id='T02',esperado='Metadatos A; inyección B al EOF de metadatos; lectura real de fuente B; rechazo I02, source identity; control sin inyección admitido',
                limite='Intercalación secuencial controlada, no estrés concurrente ni cambio posterior a admit.'),
           dict(id='T07',esperado='Estado intacto leído: observador conforme. Archivo alterado y after copiado: observador conforme (límite esperado). After releído: único fallo preservacion real del estado.',
                limite='Caracterización del observador existente y sensibilidad a evidencia real; no protección del host.')],
    resultados='PENDIENTES',cuotas=dict(fuente_bytes=len(new),canales='Cuotas existentes ReceivedBytes; fixture finito',
    salida='Tres funciones; sin bucles no acotados; archivos de evidencia JSON finitos derivados de fixture',
    timeout_orden_segundos=120))
(E/'BANCO_PREVIO.json').write_text(json.dumps(bank,ensure_ascii=False,indent=2)+'\n')
print('Fixtures A/B y banco previo preparados; no compilación ni ensayo SV.')
