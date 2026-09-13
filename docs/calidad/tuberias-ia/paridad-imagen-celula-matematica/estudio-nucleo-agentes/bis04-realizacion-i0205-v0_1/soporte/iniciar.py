from pathlib import Path
import json,hashlib,tarfile,io,gzip,subprocess
R=Path('manifiesto-sv/checkout');B=R/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes';D=B/'bis04-realizacion-i0205-v0_1'
for name in ('proyecto/src','sondas','soporte','evidencias'): (D/name).mkdir(parents=True,exist_ok=True)
core=R/'rust/sv_core';files=sorted(p for p in core.rglob('*') if p.is_file())
manifest={str(p.relative_to(core)):hashlib.sha256(p.read_bytes()).hexdigest() for p in files}
(D/'NUCLEO_FUENTES.json').write_text(json.dumps({'corte':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),'archivos':manifest},indent=2)+'\n')
b=io.BytesIO()
with tarfile.open(fileobj=b,mode='w') as t:
 for p in files:
  data=p.read_bytes();info=tarfile.TarInfo('sv_core/'+str(p.relative_to(core)));info.size=len(data);info.mode=0o644;t.addfile(info,io.BytesIO(data))
(D/'sv_core-verificado.tar.gz').write_bytes(gzip.compress(b.getvalue(),mtime=0))
(D/'proyecto/Cargo.toml').write_text('[package]\nname = "sv_bis_i0205"\nversion = "0.1.0"\nedition = "2021"\npublish = false\n\n[workspace]\n\n[dependencies]\nsv_core = { path = "../sv_core" }\n')
sha=(R/'docs/calidad/tuberias-ia/ie004/recepcion-av/sha256.rs').read_text();sha=sha[sha.index('const SHA_K'):sha.index('const MAGIC_A')]
sha=sha.replace('RT<','Result<').replace('Result<[u8;32]>','Result<[u8;32], &\'static str>').replace('Result<[u8;64]>','Result<[u8;64], &\'static str>').replace('Result<&str>','Result<&str, &\'static str>').replace('FalloT::Entero','"ENTERO"').replace('FalloT::Utf8','"UTF8"')
sha=sha[:sha.index('fn sha_texto')]+'''pub fn hash(b: &[u8]) -> String { String::from_utf8(hex_sha(&sha256(b).expect("bounded input")).to_vec()).expect("ASCII hex") }\n'''
(D/'proyecto/src/sha.rs').write_text('//! ES: SHA adaptado de IE004; sin marco SVAC ni autoridad normativa.\n//! EN: SHA adapted from IE004; no SVAC framing or normative authority.\n'+sha)
print(D)
