"""Oráculo de codificación LIG/0.1 derivado de §2, independiente del emisor Rust.

La entrada fija es synthetic_hash_contract.json. No importa sv_core ni toma su salida.
"""
import hashlib,json,struct
from pathlib import Path
ROOT=Path(__file__).resolve().parent

def encoded(c):
    result=bytearray(b'SV-LIG-0.1\0')
    def count(n): result.extend(struct.pack('>Q',n))
    def blob(b): count(len(b));result.extend(b)
    def text(s): blob(s.encode('utf-8'))
    def flag(value): result.append(int(value))
    def ref(r):
        for k in ('identifier','version','sha256'):text(r[k])
    def refs(rs):
        count(len(rs))
        for r in rs:ref(r)
    def rule(r):text(r['object']);ref(r['definition'])
    for k in ('schema','identifier','version'):text(c[k])
    for k in ('source_file','source_sha256','projection_sha256'):text(c['program'][k])
    text(c['domain']);text(c['agent']);ref(c['constitution']);ref(c['authority'])
    count(len(c['instances']))
    for i in c['instances']:
        for k in ('identifier','owner','parameter','parameter_id'):text(i[k])
        rule(i['capture']);rule(i['admission']);flag(i['ternarizer'] is not None)
        if i['ternarizer'] is not None:rule(i['ternarizer'])
        refs(i['provenance'])
    count(len(c['operations']))
    for op in c['operations']:
        text(op['identifier']);text(op['version']);ref(op['definition']);count(len(op['uses']))
        for u in op['uses']:
            text(u['identifier']);text(u['instance']);flag(u['destination'] is not None)
            if u['destination'] is not None:
                text(u['destination']['node']);text(u['destination']['position'])
            flag(u['alias_of'] is not None)
            if u['alias_of'] is not None:text(u['alias_of'])
        flag(op['requires_destination']);count(len(op['sharing']))
        for s in op['sharing']:
            text(s['instance']);count(len(s['use_ids']))
            for u in s['use_ids']:text(u)
            ref(s['rule'])
        text(op['input_scope']);refs(op['side_information'])
    count(len(c['artifacts']))
    for a in c['artifacts']:
        ref(a['reference']);text(a['kind']);blob(a['text'].encode('utf-8'))
    return bytes(result)

if __name__=='__main__':
    actual=hashlib.sha256(encoded(json.loads((ROOT/'synthetic_hash_contract.json').read_text()))).hexdigest()
    expected=(ROOT/'synthetic_hash_contract.sha256').read_text().strip()
    assert actual==expected,(actual,expected)
    print('Codificación LIG/0.1 conforme al testigo independiente:',actual)
