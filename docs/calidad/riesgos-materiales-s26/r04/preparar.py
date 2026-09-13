from pathlib import Path
import shutil,json
E=Path(__file__).resolve().parent
B=Path.cwd()/'docs/calidad/tuberias-ia/paridad-imagen-celula-matematica/estudio-nucleo-agentes/bis04-realizacion-i0205-v0_1/proyecto/src'
assert not (E/'fixture').exists()
shutil.copytree(E.parent/'r01/fixture',E/'fixture')
d=json.loads((E/'fixture/canonical.bin').read_text());assert d['vector'][0]=='Zero';d['vector'][0]='One'
(E/'fixture/vector-alterado.bin').write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n')
s=(B/'main.rs').read_text().split('fn instrumental(',1)[0]
s=s.replace('mod observer;','#[path="observer.rs"]\nmod observer;').replace('use observer::{inspect, Evidence};','pub use observer::{inspect, Evidence};')
s=s.replace('fn run_case(', 'fn run_case_antecedente(').replace('canonical: &[u8]', '_canonical: &[u8]')
s=s.replace('before: canonical.to_vec(),','before: vec![],').replace('after: canonical.to_vec(),','after: vec![],').replace('preservation_claim: true,','preservation_claim: false,')
(E/'conductor.rs').write_text(s+'\ninclude!("recepcion.rs");\n')
(E/'observer.rs').write_bytes((B/'observer.rs').read_bytes())
print('Variante preparada; observador literal y quince fixtures preservados; no ensayos ejecutados.')
