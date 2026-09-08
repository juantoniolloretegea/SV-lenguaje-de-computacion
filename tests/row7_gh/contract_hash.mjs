// Codificación LIG/0.1 §2: campos y longitudes declarados, sin importar Rust.
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {resolve} from 'node:path';
export function encodedContract(c) {
  const chunks=[Buffer.from('SV-LIG-0.1\0','ascii')];
  const count=n=>{if(!Number.isSafeInteger(n)||n<0)throw Error('LONGITUD');const b=Buffer.alloc(8);b.writeBigUInt64BE(BigInt(n));chunks.push(b);};
  const bytes=b=>{count(b.length);chunks.push(b);};
  const text=s=>{if(typeof s!=='string')throw Error('TEXTO');bytes(Buffer.from(s,'utf8'));};
  const flag=b=>{if(typeof b!=='boolean')throw Error('OPCION');chunks.push(Buffer.from([b?1:0]));};
  const nat=s=>{if(typeof s!=='string'||!/^(0|[1-9][0-9]*)$/.test(s))throw Error('NAT');text(s);};
  const list=(xs,f)=>{if(!Array.isArray(xs))throw Error('LISTA');count(xs.length);xs.forEach(f);};
  const ref=r=>{text(r.identifier);text(r.version);text(r.sha256);};
  const rule=r=>{text(r.object);ref(r.definition);};
  const option=(v,f)=>{flag(v!==null);if(v!==null)f(v);};
  text(c.schema);text(c.identifier);text(c.version);
  text(c.program.source_file);text(c.program.source_sha256);text(c.program.projection_sha256);
  text(c.domain);text(c.agent);ref(c.constitution);ref(c.authority);
  list(c.instances,i=>{
    text(i.identifier);text(i.owner);text(i.parameter);nat(i.parameter_id);rule(i.capture);rule(i.admission);
    option(i.ternarizer,rule);list(i.provenance,ref);
  });
  list(c.operations,o=>{
    text(o.identifier);text(o.version);ref(o.definition);
    list(o.uses,u=>{text(u.identifier);text(u.instance);option(u.destination,d=>{text(d.node);nat(d.position);});option(u.alias_of,text);});
    flag(o.requires_destination);list(o.sharing,s=>{text(s.instance);list(s.use_ids,text);ref(s.rule);});
    if(!['BindingsOnly','BindingsWithSideInformation'].includes(o.input_scope))throw Error('ALCANCE');
    text(o.input_scope);list(o.side_information,ref);
  });
  list(c.artifacts,a=>{
    ref(a.reference);
    if(!['Constitution','AuthorityDeclaration','CaptureDefinition','AdmissionDefinition','TernarizerDefinition','Provenance','SharingRule','OperationDefinition','SideInformation'].includes(a.kind))throw Error('CLASE');
    text(a.kind);
    if(typeof a.hex!=='string'||!/^(?:[0-9a-f]{2})*$/.test(a.hex))throw Error('HEX');
    bytes(Buffer.from(a.hex,'hex'));
  });
  return Buffer.concat(chunks);
}
export const contractHash=c=>createHash('sha256').update(encodedContract(c)).digest('hex');

// El vector y el esperado son los fijados en RETP-098; no se toma una salida Rust.
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    if (process.argv.slice(2).join(' ') !== '--referencia') throw Error('Uso: contract_hash.mjs --referencia');
    const bytes = readFileSync(new URL('../row7_bindings/synthetic_hash_contract.json', import.meta.url));
    if (createHash('sha256').update(bytes).digest('hex') !== '4b82fee9fd4ca9987c84cc453a3043392d5531660c3bc02043920fff8cb4ad85') throw Error('VECTOR_FIJO_ALTERADO');
    const expected = readFileSync(new URL('../row7_bindings/synthetic_hash_contract.sha256', import.meta.url), 'utf8').trim();
    if (expected !== 'beff99707d648d591714e431b462bf61546da931771a6168cd80b1c8037f7dc0') throw Error('ESPERADO_FIJO_ALTERADO');
    const contract = JSON.parse(bytes);
    for (const artifact of contract.artifacts) {
      artifact.hex = Buffer.from(artifact.text, 'utf8').toString('hex'); delete artifact.text;
    }
    const actual = contractHash(contract);
    if (actual !== expected) throw Error('CODIFICACION_TESTIGO_INDEPENDIENTE');
    console.log('Codificación LIG/0.1 conforme al testigo independiente:', actual);
  } catch (error) { console.error(error.message); process.exitCode = 1; }
}
